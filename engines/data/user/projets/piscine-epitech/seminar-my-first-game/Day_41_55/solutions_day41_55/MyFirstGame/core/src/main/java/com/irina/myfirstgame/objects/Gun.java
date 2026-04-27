package com.irina.myfirstgame.objects;

import com.badlogic.gdx.Gdx;
import com.irina.myfirstgame.entities.wormy.Super;
import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Item pistolet qui peut être collecté par le Super-Wormy.
 * <p>
 * Donne au Super-Wormy la capacité de tirer des projectiles pour attaquer les ennemis.
 * Le pistolet est automatiquement fourni avec 6 munitions.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Gun extends Item {

    public Gun() {
        super();
    }

    @Override
    protected void onCollect(Wormy by) {
        // Donner le pistolet seulement si le ver est Super
        if (by instanceof Super) {
            Super superWorm = (Super) by;
            superWorm.setHasGun(true);
            Gdx.app.log("Gun", "Super-Wormy a obtenu le pistolet avec 6 balles !");
        }
    }
}