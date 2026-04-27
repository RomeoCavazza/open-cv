package com.irina.myfirstgame.objects;

import com.irina.myfirstgame.entities.wormy.WormState;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.interfaces.Eatable;

/**
 * Classe abstraite représentant un aliment consommable par le ver.
 * <p>
 * Les aliments restaurent la faim et la santé du ver, et peuvent déclencher
 * l'évolution du ver si tous les aliments requis sont consommés.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class Food extends Item implements Eatable {
    protected String name;
    protected int hungerPoints;
    protected int value;
    
    @Override
    public void update(float delta) {
        super.update(delta);
    }
    
    @Override
    public void onEatenBy(Wormy target) {
        restore(target);

        WormState state = target.getEvolutionState();
        if (state != null && name != null) {
            state.registerFood(this.name);
        }
    }
    
    @Override
    protected void onCollect(Wormy by) {
        if (by != null) {
            onEatenBy(by);
        }

        // Ici, plus tard : gestion score/combo, etc.
        // ex:
        // by.getScoreboard().increase(value, name);
        // by.getComboManager().register(this);
    }

    public abstract void restore(Wormy target);
}

