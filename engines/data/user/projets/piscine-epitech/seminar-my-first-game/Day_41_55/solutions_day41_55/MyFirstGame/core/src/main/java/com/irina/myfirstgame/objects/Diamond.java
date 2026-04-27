package com.irina.myfirstgame.objects;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.wormy.Super;
import com.irina.myfirstgame.entities.wormy.WormState;
import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Item spécial qui transforme le ver en Super-Wormy.
 * <p>
 * Lorsqu'il est collecté, le diamant fait évoluer le ver vers l'état Super
 * et lui donne automatiquement un pistolet avec 6 munitions.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Diamond extends Item {

    private static final String TEXTURE_NAME = "diamond.png";

    public Diamond(Assets assets, Vector2 spawnPosition) {
        if (assets != null) {
            TextureRegion region = new TextureRegion(assets.getTexture(TEXTURE_NAME));
            this.sprite = region;
        }
        spawn(spawnPosition);
    }

    @Override
    protected void onCollect(Wormy by) {
        if (by == null) {
            return;
        }
        
        WormState wormState = by.getEvolutionState();
        if (wormState != null) {
            Gdx.app.log("Diamond", "Le ver collecte le diamant et devient Super-Wormy !");
            // Transformer le ver en super-wormy
            Super superWorm = new Super();
            wormState.evolveTo(superWorm);
            // Donner automatiquement le pistolet avec 6 munitions quand on collecte le diamant
            superWorm.setHasGun(true);
            Gdx.app.log("Diamond", "Super-Wormy a obtenu le pistolet avec 6 balles !");
        }
    }

    @Override
    public void onSpawn() {
        // Optionnel : FX / son à l'apparition
        Gdx.app.log("Diamond", "Diamant spawné à la surface !");
    }
}