package com.irina.myfirstgame.systems;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

import com.badlogic.gdx.Gdx;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.enemy.Enemy;
import com.irina.myfirstgame.objects.Item;
import com.irina.myfirstgame.objects.Diamond;
import com.irina.myfirstgame.world.World;

/**
 * Gestionnaire centralisé pour le spawn dynamique des ennemis, food et diamant.
 * 
 * Règles:
 * - Ennemis souterrains (Ant/Spider): 2-3 actifs, respawn 30s après mort
 * - Food items: 3-4 actifs, respawn 10s après collecte (plus rapide)
 * - Diamant: 1 seul, respawn quand Super perd son pouvoir
 */
public class SpawnManager {

    private static final int MIN_UNDERGROUND_ENEMIES = 2;
    private static final int MAX_UNDERGROUND_ENEMIES = 3;
    private static final int MIN_FOOD_ITEMS = 5;
    private static final int MAX_FOOD_ITEMS = 7;
    private static final float ENEMY_RESPAWN_DELAY = 10f; // 10 secondes pour les ennemis (plus rapide)
    private static final float FOOD_RESPAWN_DELAY = 10f; // 10 secondes pour la nourriture (plus rapide)
    private static final float DIAMOND_RESPAWN_DELAY = 30f; // 30 secondes pour le diamant

    private final Random random = new Random();

    // Listes des entités actives
    private final List<Enemy> activeUndergroundEnemies = new ArrayList<>();
    private final List<Item> activeFoodItems = new ArrayList<>();
    private Diamond activeDiamond = null;

    // Timers de respawn
    private float enemyRespawnTimer = 0f;
    private float foodRespawnTimer = 0f;
    private float diamondRespawnTimer = 0f;

    // Flags
    private boolean needDiamondRespawn = false;

    // Flag pour le spawn unique de la Pie
    private boolean pieSpawned = false;

    // Nombre cible d'entités (aléatoire au début)
    private int targetEnemyCount;
    private int targetFoodCount;

    public SpawnManager() {
        // Déterminer les nombres cibles aléatoirement
        targetEnemyCount = MIN_UNDERGROUND_ENEMIES
                + random.nextInt(MAX_UNDERGROUND_ENEMIES - MIN_UNDERGROUND_ENEMIES + 1);
        targetFoodCount = MIN_FOOD_ITEMS + random.nextInt(MAX_FOOD_ITEMS - MIN_FOOD_ITEMS + 1);

        Gdx.app.log("SpawnManager",
                "Initialized - Target enemies: " + targetEnemyCount + ", Target food: " + targetFoodCount);
    }

    /**
     * Initialise le spawn initial au début de la partie
     */
    public void initialize(World world, Assets assets, Player player) {
        Gdx.app.log("SpawnManager", "Starting initial spawn...");

        // Spawn initial des ennemis souterrains
        spawnInitialEnemies(world, assets, player);

        // Spawn initial de la nourriture
        spawnInitialFood(world, assets);

        // Spawn du diamant unique
        spawnDiamond(world, assets);

        // Spawn unique de la Pie dans le ciel
        if (!pieSpawned) {
            spawnPie(world, assets, player);
            pieSpawned = true;
        }

        Gdx.app.log("SpawnManager", "Initial spawn complete - Enemies: " + activeUndergroundEnemies.size() +
                ", Food: " + activeFoodItems.size() + ", Diamond: " + (activeDiamond != null ? "Yes" : "No") +
                ", Pie: " + pieSpawned);
    }

    /**
     * Met à jour les timers et gère les respawns
     */
    public void update(float delta, World world, Assets assets, Player player) {
        // Gérer le respawn des ennemis
        if (activeUndergroundEnemies.size() < targetEnemyCount) {
            enemyRespawnTimer += delta;
            if (enemyRespawnTimer >= ENEMY_RESPAWN_DELAY) {
                spawnRandomUndergroundEnemy(world, assets, player);
                enemyRespawnTimer = 0f;
            }
        } else {
            enemyRespawnTimer = 0f;
        }

        // Gérer le respawn de la nourriture
        if (activeFoodItems.size() < targetFoodCount) {
            foodRespawnTimer += delta;
            if (foodRespawnTimer >= FOOD_RESPAWN_DELAY) {
                spawnRandomFood(world, assets);
                foodRespawnTimer = 0f;
            }
        } else {
            foodRespawnTimer = 0f;
        }

        // Gérer le respawn du diamant
        if (needDiamondRespawn && activeDiamond == null) {
            diamondRespawnTimer += delta;
            if (diamondRespawnTimer >= DIAMOND_RESPAWN_DELAY) {
                spawnDiamond(world, assets);
                diamondRespawnTimer = 0f;
                needDiamondRespawn = false;
            }
        }
    }

    /**
     * Appelé quand un ennemi souterrain meurt
     */
    public void onUndergroundEnemyKilled(Enemy enemy) {
        activeUndergroundEnemies.remove(enemy);
        Gdx.app.log("SpawnManager",
                "Enemy killed - Active: " + activeUndergroundEnemies.size() + "/" + targetEnemyCount);

        // La logique de respawn est gérée automatiquement dans update()
    }

    /**
     * Appelé quand un item food est collecté
     */
    public void onFoodCollected(Item food) {
        activeFoodItems.remove(food);
        Gdx.app.log("SpawnManager", "Food collected - Active: " + activeFoodItems.size() + "/" + targetFoodCount);

        // La logique de respawn est gérée automatiquement dans update()
    }

    /**
     * Appelé quand le diamant est collecté
     */
    public void onDiamondCollected() {
        activeDiamond = null;
        Gdx.app.log("SpawnManager", "Diamond collected");
    }

    /**
     * Appelé quand le Super perd son pouvoir (toutes les balles tirées)
     */
    public void onSuperPowerLost() {
        needDiamondRespawn = true;
        diamondRespawnTimer = 0f;
        Gdx.app.log("SpawnManager", "Super power lost - Diamond respawn scheduled in 30s");
    }

    // ========== MÉTHODES PRIVÉES DE SPAWN ==========

    private void spawnInitialEnemies(World world, Assets assets, Player player) {
        for (int i = 0; i < targetEnemyCount; i++) {
            spawnRandomUndergroundEnemy(world, assets, player);
        }
    }

    private void spawnInitialFood(World world, Assets assets) {
        for (int i = 0; i < targetFoodCount; i++) {
            spawnRandomFood(world, assets);
        }
    }

    private void spawnRandomUndergroundEnemy(World world, Assets assets, Player player) {
        // Collecter les cellules disponibles dans les souterrains
        // Utiliser une taille générique pour les ennemis
        float enemyWidth = 16f;
        float enemyHeight = 16f;

        List<com.irina.myfirstgame.entities.valueobjects.Vector2> cells = world.getSpawner().collectDeepDirtCells(world,
                enemyWidth, enemyHeight);

        if (cells.isEmpty()) {
            Gdx.app.error("SpawnManager", "No available cells for enemy spawn!");
            return;
        }

        // Prendre une cellule aléatoire
        com.irina.myfirstgame.entities.valueobjects.Vector2 spawnCell = world.getSpawner().takeRandomDeepCell(cells);

        if (spawnCell == null) {
            Gdx.app.error("SpawnManager", "Failed to get random cell for enemy spawn!");
            return;
        }

        // 50% chance Ant, 50% chance Spider
        Enemy enemy;
        if (random.nextBoolean()) {
            enemy = world.getSpawner().spawnAntAt(world, assets, player, spawnCell);
            Gdx.app.log("SpawnManager", "Spawned Ant at (" + spawnCell.getX() + ", " + spawnCell.getY() + ")");
        } else {
            enemy = world.getSpawner().spawnSpiderAt(world, assets, player, spawnCell);
            Gdx.app.log("SpawnManager", "Spawned Spider at (" + spawnCell.getX() + ", " + spawnCell.getY() + ")");
        }

        if (enemy != null) {
            activeUndergroundEnemies.add(enemy);
            world.getEnemies().add(enemy);
        }
    }

    private void spawnRandomFood(World world, Assets assets) {
        // 33% chance chaque type
        int foodType = random.nextInt(3);
        Item food = null;

        switch (foodType) {
            case 0:
                food = world.getSpawnerObject().spawnBurgerInDeepTunnel(world, assets);
                Gdx.app.log("SpawnManager", "Spawned Burger");
                break;
            case 1:
                food = world.getSpawnerObject().spawnFritesInDeepTunnel(world, assets);
                Gdx.app.log("SpawnManager", "Spawned Frites");
                break;
            case 2:
                food = world.getSpawnerObject().spawnSodaInDeepTunnel(world, assets);
                Gdx.app.log("SpawnManager", "Spawned Soda");
                break;
        }

        if (food != null) {
            activeFoodItems.add(food);
            world.getItems().add(food);
        }
    }

    private void spawnDiamond(World world, Assets assets) {
        activeDiamond = (Diamond) world.getSpawnerObject().spawnDiamondAtSurface(world, assets);
        if (activeDiamond != null) {
            world.getItems().add(activeDiamond);
            Gdx.app.log("SpawnManager", "Spawned Diamond at surface");
        }
    }

    private void spawnPie(World world, Assets assets, Player player) {
        Enemy pie = world.getSpawner().spawnPie(world, assets, player);
        if (pie != null) {
            // On n'ajoute pas la Pie à activeUndergroundEnemies car elle ne compte pas dans
            // la limite
            // et ne respawn pas
            world.getEnemies().add(pie);
            Gdx.app.log("SpawnManager", "Spawned Pie in the sky");
        }
    }

    // ========== GETTERS ==========

    public List<Enemy> getActiveUndergroundEnemies() {
        return activeUndergroundEnemies;
    }

    public List<Item> getActiveFoodItems() {
        return activeFoodItems;
    }

    public Diamond getActiveDiamond() {
        return activeDiamond;
    }
}
