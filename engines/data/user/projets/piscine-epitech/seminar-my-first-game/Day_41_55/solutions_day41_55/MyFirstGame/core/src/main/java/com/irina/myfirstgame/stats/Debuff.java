package com.irina.myfirstgame.stats;

import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Effet de statut négatif qui réduit les capacités du ver.
 * <p>
 * Stub pour une fonctionnalité future de debuff.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Debuff extends StatusEffect {
    private String penaltyType;
    private float penaltyValue;
    
    @Override
    public void apply(Wormy target) {
    }
    
    @Override
    public void end(Wormy target) {
    }
}

