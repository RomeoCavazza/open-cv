package com.irina.myfirstgame.objects;

import com.irina.myfirstgame.entities.wormy.WormState;

/**
 * Gère les combos d'aliments pour déclencher l'évolution du ver.
 * <p>
 * Stub pour une fonctionnalité future de gestion des combos.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class ComboManager {
    private Timer timer;
    private WormState wormState;
    private boolean sodaConsumed;
    private boolean fritesConsumed;
    private boolean burgerConsumed;
    
    public void register(Food food) {
    }
    
    public boolean isComboActive() {
        return false;
    }
    
    public void grantEvolution(WormState state) {
    }
    
    public void checkCombo() {
    }
    
    public void reset() {
    }
}

