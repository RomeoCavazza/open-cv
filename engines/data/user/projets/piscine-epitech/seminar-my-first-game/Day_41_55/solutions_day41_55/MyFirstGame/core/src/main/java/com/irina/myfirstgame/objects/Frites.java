package com.irina.myfirstgame.objects;

import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Aliment Frites qui restaure la faim et la santé du ver.
 * <p>
 * Les Frites restaurent 20 points de faim et 20 points de santé.
 * Elles font partie du combo requis pour l'évolution de Baby vers Adult.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Frites extends Food {

    private static final String TEXTURE_NAME = "frites.png";

    public Frites(Assets assets, Vector2 spawnPos) {
        this.name = "frites";
        this.hungerPoints = 20;
        this.value = 20;

        TextureRegion region = new TextureRegion(assets.getTexture(TEXTURE_NAME));
        this.sprite = region;

        spawn(spawnPos);
    }

    @Override
    public void restore(Wormy target) {
        if (target != null) {
            if (target.getHunger() != null) {
                target.getHunger().increase(hungerPoints);
            }
            if (target.getHealth() != null) {
                target.getHealth().heal(hungerPoints);
            }
        }
    }
}