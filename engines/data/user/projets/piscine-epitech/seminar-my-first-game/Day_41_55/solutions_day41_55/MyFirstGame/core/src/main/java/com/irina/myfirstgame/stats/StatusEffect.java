package com.irina.myfirstgame.stats;

import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Classe abstraite représentant un effet de statut appliqué au ver.
 * <p>
 * Les effets de statut peuvent être positifs (Buff) ou négatifs (Poison, Slow, Debuff).
 * Ils ont une durée limitée et peuvent modifier les capacités du ver.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class StatusEffect {
    private String name;
    private float duration;
    private float elapsed;
    private boolean active;
    
    public void apply(Wormy target) {
    }
    
    public void update(float delta) {
    }
    
    public void end(Wormy target) {
    }
    
    public boolean isExpired() {
        return false;
    }
}

