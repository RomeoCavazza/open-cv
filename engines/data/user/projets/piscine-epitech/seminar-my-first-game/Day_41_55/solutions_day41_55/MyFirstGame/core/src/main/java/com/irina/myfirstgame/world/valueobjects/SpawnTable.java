package com.irina.myfirstgame.world.valueobjects;

import java.util.Map;

/**
 * Table de spawn pondérée pour sélectionner aléatoirement des entités.
 * <p>
 * Utilise un système de poids pour déterminer la probabilité d'apparition
 * de chaque type d'entité.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class SpawnTable {
    public Map<String, Float> entries;

    public String random() {
        float total = 0f;
        for (Float weight : entries.values()) {
            total += weight;
        }
        float r = (float) Math.random() * total;

        for (Map.Entry<String, Float> entry : entries.entrySet()) {
            r -= entry.getValue();
            if (r <= 0) {
                return entry.getKey();
            }
        }
        return null;
    }
}
