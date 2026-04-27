package com.irina.myfirstgame.objects;

import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.entities.enemy.Enemy;

/**
 * Projectile tiré par le Super-Wormy.
 * <p>
 * Le projectile se déplace en ligne droite dans une direction donnée
 * et inflige des dégâts aux ennemis qu'il touche. Il disparaît après
 * 5 secondes ou s'il sort des limites de la carte.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Projectile extends Entity {
    private float speed;
    private Vector2 direction;
    private int damage;
    private boolean active;
    private float lifetime;

    public Projectile() {
        super();
        this.speed = 200f;
        this.direction = new Vector2(1, 0);
        this.damage = 100; // Increased from 10 to 100 for one-shot kills
        this.active = false;
        this.lifetime = 0f;
    }

    public void launch(Vector2 from) {
        if (from != null) {
            setPosition(from);
            this.active = true;
            this.lifetime = 0f;
        }
    }

    public void move(float delta) {
        if (active && direction != null) {
            Vector2 pos = getPosition();
            pos.add(direction.getX() * speed * delta, direction.getY() * speed * delta);
        }
    }

    public void hit(Enemy target) {
        if (target != null && active) {
            target.takeDamage(damage);
            active = false;
        }
    }

    @Override
    public void update(float delta) {
        if (!active)
            return;

        lifetime += delta;
        if (lifetime > 5f) {
            active = false;
            return;
        }

        move(delta);

        Vector2 pos = getPosition();
        if (pos.getX() < -100 || pos.getX() > 10000 ||
                pos.getY() < -100 || pos.getY() > 10000) {
            active = false;
        }
    }

    public boolean isActive() {
        return active;
    }

    public void setDirection(Vector2 direction) {
        if (direction != null) {
            // Créer une copie et normaliser pour garantir une direction unitaire
            // (trajectoire droite)
            this.direction = new Vector2(direction);
            this.direction.nor(); // Normaliser pour garantir une vitesse constante
        }
    }

    public void setDamage(int damage) {
        this.damage = damage;
    }
}
