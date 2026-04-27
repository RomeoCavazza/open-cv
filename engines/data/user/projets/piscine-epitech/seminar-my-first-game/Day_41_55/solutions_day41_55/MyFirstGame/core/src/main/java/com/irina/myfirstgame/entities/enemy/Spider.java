package com.irina.myfirstgame.entities.enemy;

import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.world.Map;

/**
 * Ennemi Araignée qui se déplace à la surface et dans les tunnels.
 * Hérite de SurfaceEnemy pour toute la logique commune de mouvement.
 * Attaque en empoisonnant la cible.
 */
public class Spider extends SurfaceEnemy {

    private static final float WALK_FPS = 12f;
    private static final float MOVE_SPEED = 85f;
    private static final float ATTACK_RANGE = 36f;
    private static final float ATTACK_COOLDOWN = 1.25f;
    private static final int ATTACK_DAMAGE = 4;
    private static final float REPATH_INTERVAL = 0.35f;
    private static final String TEXTURE_NAME = "spider.png";
    private static final float HITBOX_SCALE = 0.6f;

    public Spider(Assets assets, Vector2 spawnPosition, Player target, float surfaceLevel, Map map, String collisionLayer) {
        super(spawnPosition, target, surfaceLevel, map, collisionLayer, 
              createAnimation(assets, TEXTURE_NAME, WALK_FPS), 
              createSprite(assets, TEXTURE_NAME));
        setupHitbox(assets, TEXTURE_NAME, this, HITBOX_SCALE);
    }

    @Override
    protected float getMoveSpeed() {
        return MOVE_SPEED;
    }

    @Override
    protected float getAttackRange() {
        return ATTACK_RANGE;
    }

    @Override
    protected float getAttackCooldown() {
        return ATTACK_COOLDOWN;
    }

    @Override
    protected int getAttackDamage() {
        return ATTACK_DAMAGE;
    }

    @Override
    protected float getRepathInterval() {
        return REPATH_INTERVAL;
    }

    @Override
    protected String getTextureName() {
        return TEXTURE_NAME;
    }

    @Override
    protected float getAnimationFPS() {
        return WALK_FPS;
    }
}
