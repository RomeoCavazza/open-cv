package com.irina.myfirstgame.objects;

import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Aliment Soda qui restaure la faim et la santé du ver.
 * <p>
 * Le Soda restaure 10 points de faim et 10 points de santé.
 * Il fait partie du combo requis pour l'évolution de Baby vers Adult.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Soda extends Food {
    private static final String TEXTURE_NAME = "soda.png";

    public Soda(Assets assets, Vector2 spawnPos) {
        this.name = "soda";
        this.hungerPoints = 10;
        this.value = 5;

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
