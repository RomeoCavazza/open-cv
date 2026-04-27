package com.irina.myfirstgame.world;

import java.util.ArrayList;
import java.util.List;
import java.lang.Runnable;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.maps.tiled.renderers.OrthogonalTiledMapRenderer;
import com.badlogic.gdx.math.Rectangle;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.Spawner;
import com.irina.myfirstgame.entities.enemy.Enemy;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.objects.Item;
import com.irina.myfirstgame.objects.Projectile;
import com.irina.myfirstgame.objects.SpawnerObject;
import com.irina.myfirstgame.systems.Physics;
import com.irina.myfirstgame.systems.SpawnManager;

/**
 * Conteneur principal du monde du jeu.
 * <p>
 * Gère la carte, la caméra, les entités (ennemis, items, projectiles),
 * les collisions, le spawn et la mise à jour de tous les éléments du jeu.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class World {

    private Map map;
    private CollisionGrid collisions;
    private Camera camera;
    private Spawner spawner;
    private SpawnerObject spawnerObject;
    private SpawnManager spawnManager;
    private com.irina.myfirstgame.systems.ScoreManager scoreManager;
    private LevelLoader levelLoader;
    private Physics physics;
    private OrthogonalTiledMapRenderer mapRenderer;
    // Callback invoked when the Pie enemy is killed
    private Runnable pieKilledCallback;
    // Callback invoked when the player dies, with the cause of death
    private java.util.function.Consumer<String> deathCauseCallback;
    private String collisionLayerName;
    private Player boundPlayer;
    private final List<Enemy> enemies = new ArrayList<>();
    // Temporary list to avoid ConcurrentModificationException when removing during
    // iteration
    private final List<Enemy> enemiesToRemove = new ArrayList<>();
    private final List<Item> items = new ArrayList<>();
    private final List<Projectile> projectiles = new ArrayList<>();
    private float surfaceLevel;
    private com.irina.myfirstgame.systems.CollisionHandler enemyCollisionHandler;
    private float damageCooldown = 0f;

    public World(String mapFile, float viewportWidth, float viewportHeight) {
        this.map = new Map(mapFile);
        this.collisions = new CollisionGrid();
        this.camera = new Camera(viewportWidth, viewportHeight);
        this.spawner = new Spawner();
        this.spawnerObject = new SpawnerObject();
        this.levelLoader = new LevelLoader();
        this.physics = new Physics();
        this.mapRenderer = new OrthogonalTiledMapRenderer(map.getTiledMap(), 1f);
        this.collisionLayerName = null;
        this.surfaceLevel = 0f;
    }

    public World(String mapFile, float viewportWidth, float viewportHeight, String collisionLayerName) {
        this(mapFile, viewportWidth, viewportHeight);
        this.collisionLayerName = collisionLayerName;
        this.surfaceLevel = map.calculateSurfaceLevel(collisionLayerName);
    }

    public void update(float delta) {
        if (damageCooldown > 0) {
            damageCooldown -= delta;
        }

        if (camera != null) {
            camera.update(delta);
            camera.clampTo(map);
        }

        handleEnemyCollisions();

        // Mettre à jour les ennemis et les forcer au sol si nécessaire
        for (Enemy enemy : enemies) {
            enemy.update(delta);
            // FORCER Ant et Spider au sol en surface (comme le joueur)
            // La Pie doit pouvoir voler, donc on ne la contraint pas
            // Appelé APRÈS enemy.update() pour forcer au sol même si le pathfinding les
            // remet en l'air
            if (enemyCollisionHandler != null && surfaceLevel > 0f) {
                // Vérifier si c'est Ant ou Spider (pas Pie)
                if (enemy instanceof com.irina.myfirstgame.entities.enemy.SurfaceEnemy) {
                    enemyCollisionHandler.constrainToGround(enemy, surfaceLevel);
                }
            }
        }
        for (Item item : items) {
            item.update(delta);
        }

        // Mettre à jour les projectiles et supprimer ceux qui ne sont plus actifs
        projectiles.removeIf(projectile -> {
            projectile.update(delta);
            if (!projectile.isActive()) {
                return true;
            }
            // Vérifier collisions avec les ennemis
            for (Enemy enemy : enemies) {
                // Ignorer les ennemis déjà marqués pour suppression (évite les doublons de points)
                if (enemiesToRemove.contains(enemy)) {
                    continue;
                }
                
                if (checkProjectileEnemyCollision(projectile, enemy)) {
                    projectile.hit(enemy);
                    // Vérifier si l'ennemi est mort après avoir reçu des dégâts
                    if (enemy.isDead() && !enemiesToRemove.contains(enemy)) {
                        // Ajouter les points selon la charte du scoreboard (seulement pour Super Worm)
                        if (scoreManager != null && boundPlayer != null && boundPlayer.getWorm() != null) {
                            Wormy currentWorm = boundPlayer.getWorm().getCurrent();
                            if (currentWorm instanceof com.irina.myfirstgame.entities.wormy.Super) {
                                if (enemy instanceof com.irina.myfirstgame.entities.enemy.Ant) {
                                    scoreManager.addPoints(100); // Tuer une fourmi : +100 points
                                } else if (enemy instanceof com.irina.myfirstgame.entities.enemy.Spider) {
                                    scoreManager.addPoints(200); // Tuer une araignée : +200 points
                                }
                            }
                        }
                        
                        // Notifier le SpawnManager si c'est un ennemi souterrain (Ant ou Spider)
                        if (spawnManager != null) {
                            if (enemy instanceof com.irina.myfirstgame.entities.enemy.SurfaceEnemy) {
                                spawnManager.onUndergroundEnemyKilled(enemy);
                            } else if (enemy instanceof com.irina.myfirstgame.entities.enemy.Pie) {
                                // Si c'est la Pie, déclencher le callback pour Game Over
                                Gdx.app.log("World", "Pie killed! Triggering GameOver via callback.");
                                if (pieKilledCallback != null) {
                                    pieKilledCallback.run();
                                }
                            }
                        }
                        // Marquer l'ennemi pour suppression AVANT d'ajouter des points supplémentaires
                        enemiesToRemove.add(enemy);
                        Gdx.app.log("World", "Enemy " + enemy.getClass().getSimpleName() + " killed by projectile!");
                    }
                    return true; // Supprimer le projectile après impact
                }
            }
            return false;
        });

        // Supprimer les ennemis morts après la boucle des projectiles
        if (!enemiesToRemove.isEmpty()) {
            int removedCount = enemiesToRemove.size();
            enemies.removeAll(enemiesToRemove);
            enemiesToRemove.clear();
            Gdx.app.log("World", "Removed " + removedCount + " dead enemies from the world.");
        }

        handleItemCollection();
    }

    private boolean checkProjectileEnemyCollision(Projectile projectile, Enemy enemy) {
        if (projectile == null || enemy == null || !projectile.isActive()) {
            return false;
        }
        Vector2 projPos = projectile.getPosition();
        Vector2 enemyPos = enemy.getPosition();
        float projWidth = projectile.getWidth();
        float projHeight = projectile.getHeight();
        float enemyWidth = enemy.getWidth();
        float enemyHeight = enemy.getHeight();

        float projLeft = projPos.getX() - projWidth * 0.5f;
        float projRight = projPos.getX() + projWidth * 0.5f;
        float projBottom = projPos.getY() - projHeight * 0.5f;
        float projTop = projPos.getY() + projHeight * 0.5f;

        float enemyLeft = enemyPos.getX() - enemyWidth * 0.5f;
        float enemyRight = enemyPos.getX() + enemyWidth * 0.5f;
        float enemyBottom = enemyPos.getY() - enemyHeight * 0.5f;
        float enemyTop = enemyPos.getY() + enemyHeight * 0.5f;

        return projLeft < enemyRight && projRight > enemyLeft &&
                projBottom < enemyTop && projTop > enemyBottom;
    }

    private void handleItemCollection() {
        if (boundPlayer == null || boundPlayer.getWorm() == null) {
            return;
        }

        Wormy worm = boundPlayer.getWorm().getCurrent();
        if (worm == null) {
            return;
        }

        Rectangle playerRect = new Rectangle(
                boundPlayer.getPosition().getX(),
                boundPlayer.getPosition().getY(),
                boundPlayer.getWidth(),
                boundPlayer.getHeight());

        for (Item item : items) {
            if (!item.isActive() || item.isCollected()) {
                continue;
            }

            if (playerRect.overlaps(item.getBounds())) {
                // Ajouter les points AVANT collect() pour éviter les doublons
                // (on a déjà vérifié !item.isCollected() dans le if de la ligne 199)
                if (scoreManager != null) {
                    if (item instanceof com.irina.myfirstgame.objects.Soda) {
                        scoreManager.addPoints(10); // Soda mangé : +10 points
                    } else if (item instanceof com.irina.myfirstgame.objects.Frites) {
                        scoreManager.addPoints(20); // Frites mangées : +20 points
                    } else if (item instanceof com.irina.myfirstgame.objects.Burger) {
                        scoreManager.addPoints(30); // Burger mangé : +30 points
                    } else if (item instanceof com.irina.myfirstgame.objects.Diamond) {
                        scoreManager.addPoints(200); // Ramasser diamant : +200 points
                    }
                }
                
                // Collecter l'item après avoir ajouté les points
                // collect() met collected = true et active = false, donc il ne sera plus comptabilisé
                item.collect(worm);

                // Notifier le SpawnManager si c'est de la nourriture
                if (spawnManager != null && (item instanceof com.irina.myfirstgame.objects.Burger ||
                        item instanceof com.irina.myfirstgame.objects.Frites ||
                        item instanceof com.irina.myfirstgame.objects.Soda)) {
                    spawnManager.onFoodCollected(item);
                }

                // Notifier si c'est le diamant
                if (spawnManager != null && item instanceof com.irina.myfirstgame.objects.Diamond) {
                    spawnManager.onDiamondCollected();
                }
            }
        }
    }

    private void handleEnemyCollisions() {
        if (boundPlayer == null || boundPlayer.getWorm() == null || damageCooldown > 0) {
            return;
        }

        Wormy worm = boundPlayer.getWorm().getCurrent();
        if (worm == null) {
            return;
        }

        // Centrer le rectangle sur la position (qui est le centre de l'entité)
        Rectangle playerRect = new Rectangle(
                boundPlayer.getPosition().getX() - boundPlayer.getWidth() / 2,
                boundPlayer.getPosition().getY() - boundPlayer.getHeight() / 2,
                boundPlayer.getWidth(),
                boundPlayer.getHeight());

        for (Enemy enemy : enemies) {
            // Centrer le rectangle sur la position
            Rectangle enemyRect = new Rectangle(
                    enemy.getPosition().getX() - enemy.getWidth() / 2,
                    enemy.getPosition().getY() - enemy.getHeight() / 2,
                    enemy.getWidth(),
                    enemy.getHeight());

            if (playerRect.overlaps(enemyRect)) {
                int damage = 0;
                boolean isInsect = false;

                if (enemy instanceof com.irina.myfirstgame.entities.enemy.Ant) {
                    damage = 5;
                    isInsect = true;
                } else if (enemy instanceof com.irina.myfirstgame.entities.enemy.Spider) {
                    damage = 10;
                    isInsect = true;
                } else if (enemy instanceof com.irina.myfirstgame.entities.enemy.Pie) {
                    damage = 100;
                }

                if (damage > 0) {
                    worm.takeDamage(damage);

                    // Check if this damage killed the player
                    if (worm.getHealth().isDead() && deathCauseCallback != null) {
                        String enemyName = enemy.getClass().getSimpleName();
                        deathCauseCallback.accept(enemyName);
                    }
                    // Pénalités spécifiques aux insectes (Ant/Spider)
                    if (isInsect) {
                        // Reset du combo pour tout le monde (Baby et Adult)
                        if (boundPlayer.getWorm() != null) {
                            boundPlayer.getWorm().resetCombo();
                        }

                        // Si c'est un Adult, il redevient Baby
                        if (worm instanceof com.irina.myfirstgame.entities.wormy.Adult) {
                            com.irina.myfirstgame.entities.wormy.Baby baby = new com.irina.myfirstgame.entities.wormy.Baby();
                            // PlayScreen s'occupera d'initialiser les animations (lazy loading)
                            boundPlayer.getWorm().evolveTo(baby);
                        }
                    }

                    damageCooldown = 1.0f; // 1 second invulnerability
                    return; // Only take damage from one enemy per frame
                }
            }
        }
    }

    public void render() {
        if (mapRenderer != null && camera != null) {
            mapRenderer.setView(camera.getLibgdxCamera());
            mapRenderer.render();
        }
    }

    public void renderEnemies(SpriteBatch batch) {
        for (Enemy enemy : enemies) {
            enemy.render(batch);
        }
    }

    public Map getMap() {
        return map;
    }

    public Camera getCamera() {
        return camera;
    }

    public CollisionGrid getCollisionGrid() {
        return collisions;
    }

    public Spawner getSpawner() {
        return spawner;
    }

    public SpawnerObject getSpawnerObject() {
        return spawnerObject;
    }

    public SpawnManager getSpawnManager() {
        return spawnManager;
    }

    public void setSpawnManager(SpawnManager spawnManager) {
        this.spawnManager = spawnManager;
    }

    public com.irina.myfirstgame.systems.ScoreManager getScoreManager() {
        return scoreManager;
    }

    public void setScoreManager(com.irina.myfirstgame.systems.ScoreManager scoreManager) {
        this.scoreManager = scoreManager;
    }

    // Setter for the Pie killed callback
    public void setPieKilledCallback(Runnable callback) {
        this.pieKilledCallback = callback;
    }

    // Setter for the death cause callback
    public void setDeathCauseCallback(java.util.function.Consumer<String> callback) {
        this.deathCauseCallback = callback;
    }

    public List<Enemy> getEnemies() {
        return enemies;
    }

    public List<Item> getItems() {
        return items;
    }

    public LevelLoader getLevelLoader() {
        return levelLoader;
    }

    public Physics getPhysics() {
        return physics;
    }

    public float getSurfaceLevel() {
        return surfaceLevel;
    }

    public String getCollisionLayerName() {
        return collisionLayerName;
    }

    public void setCollisionLayer(String collisionLayerName) {
        this.collisionLayerName = collisionLayerName;
        if (collisionLayerName != null) {
            this.surfaceLevel = map.calculateSurfaceLevel(collisionLayerName);
            this.enemyCollisionHandler = new com.irina.myfirstgame.systems.CollisionHandler(map, collisionLayerName);
        }
    }

    public void bindPlayer(Player player) {
        this.boundPlayer = player;
    }

    public void spawnInitialEnemies(Assets assets) {
        if (boundPlayer == null || assets == null || !enemies.isEmpty()) {
            return;
        }
        enemies.add(spawner.spawnPie(this, assets, boundPlayer));

        float antWidth = spawner.computeAntHitboxWidth(assets);
        float antHeight = spawner.computeAntHitboxHeight(assets);
        List<Vector2> antCells = spawner.collectDeepDirtCells(this, antWidth, antHeight);
        Vector2 antCell = spawner.takeRandomDeepCell(antCells);

        float spiderWidth = spawner.computeSpiderHitboxWidth(assets);
        float spiderHeight = spawner.computeSpiderHitboxHeight(assets);
        List<Vector2> spiderCells = spawner.collectDeepDirtCells(this, spiderWidth, spiderHeight);
        if (antCell != null) {
            spiderCells.removeIf(cell -> Math.abs(cell.getX() - antCell.getX()) < 0.1f
                    && Math.abs(cell.getY() - antCell.getY()) < 0.1f);
        }
        Vector2 spiderCell = spawner.takeRandomDeepCell(spiderCells);

        enemies.add(spawner.spawnAntAt(this, assets, boundPlayer, antCell));
        enemies.add(spawner.spawnSpiderAt(this, assets, boundPlayer, spiderCell));
    }

    public void spawnInitialItems(Assets assets) {
        if (assets == null || !items.isEmpty()) {
            return;
        }

        Item burger = spawnerObject.spawnBurgerInDeepTunnel(this, assets);
        if (burger != null) {
            items.add(burger);
        }

        Item frites = spawnerObject.spawnFritesInDeepTunnel(this, assets);
        if (frites != null) {
            items.add(frites);
        }

        Item soda = spawnerObject.spawnSodaInDeepTunnel(this, assets);
        if (soda != null) {
            items.add(soda);
        }
    }

    public void spawnDiamond(Assets assets) {
        if (assets == null) {
            return;
        }

        // Vérifier qu'il n'y a pas déjà un diamant spawné
        for (Item item : items) {
            if (item instanceof com.irina.myfirstgame.objects.Diamond && item.isActive()) {
                return; // Déjà un diamant actif
            }
        }

        Item diamond = spawnerObject.spawnDiamondAtSurface(this, assets);
        if (diamond != null) {
            items.add(diamond);
        }
    }

    public void addItem(Item item) {
        if (item != null) {
            items.add(item);
        }
    }

    public void addProjectile(Projectile projectile) {
        if (projectile != null) {
            projectiles.add(projectile);
        }
    }

    public void renderItems(SpriteBatch batch) {
        for (Item item : items) {
            if (item.isActive()) {
                item.render(batch);
            }
        }
    }

    public void renderProjectiles(SpriteBatch batch, Assets assets) {
        for (Projectile projectile : projectiles) {
            if (projectile.isActive() && assets != null) {
                com.badlogic.gdx.graphics.g2d.TextureRegion bulletRegion = assets.getTextureRegion("bullet.png");
                if (bulletRegion != null) {
                    com.irina.myfirstgame.entities.valueobjects.Vector2 pos = projectile.getPosition();
                    // Utiliser les dimensions du projectile (déjà réduites)
                    float width = projectile.getWidth();
                    float height = projectile.getHeight();
                    if (width <= 0)
                        width = 6f; // Taille par défaut (6x6 pixels)
                    if (height <= 0)
                        height = 6f;
                    // Centrer le projectile sur sa position
                    batch.draw(bulletRegion, pos.getX() - width * 0.5f, pos.getY() - height * 0.5f,
                            width, height);
                }
            }
        }
    }

    public void dispose() {
        if (mapRenderer != null) {
            mapRenderer.dispose();
        }
        if (map != null) {
            map.dispose();
        }
    }
}
