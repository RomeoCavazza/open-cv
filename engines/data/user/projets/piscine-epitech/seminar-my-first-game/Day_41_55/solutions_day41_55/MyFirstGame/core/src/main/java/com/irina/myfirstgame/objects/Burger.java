package com.irina.myfirstgame.objects;

import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Aliment Burger qui restaure la faim et la santé du ver.
 * <p>
 * Le Burger restaure 30 points de faim et 30 points de santé.
 * Il fait partie du combo requis pour l'évolution de Baby vers Adult.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Burger extends Food {

    private static final String TEXTURE_NAME = "burger.png";

    public Burger(Assets assets, Vector2 spawnPosition) {
        this.name = "burger";
        this.hungerPoints = 30;
        this.value = 30;

        TextureRegion region = new TextureRegion(assets.getTexture(TEXTURE_NAME));
        this.sprite = region;

        spawn(spawnPosition);
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