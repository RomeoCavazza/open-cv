package com.irina.myfirstgame.world;

import com.irina.myfirstgame.world.valueobjects.SpawnTable;

import java.util.Map;

/**
 * Définit les règles de spawn pour les ennemis et les ressources.
 * <p>
 * Contient des tables de spawn pondérées pour déterminer quels ennemis
 * et ressources apparaissent dans le monde.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class SpawnRules {

    public final SpawnTable enemies;
    public final SpawnTable resources;

    public SpawnRules() {
        enemies = new SpawnTable();
        resources = new SpawnTable();

        resources.entries = Map.of(
            "burger", 0.5f,
            "shovel", 0.5f
        );

        // Exemple pour les ennemis plus tard :
        // enemies.entries = Map.of(
        //     "basicEnemy", 0.7f,
        //     "fastEnemy", 0.3f
        // );
    }
}