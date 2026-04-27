package com.irina.myfirstgame.objects;

import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.math.Vector2;

/**
 * Classe abstraite de base pour tous les objets du jeu.
 * <p>
 * Représente un objet avec une position, un identifiant et un état actif/inactif.
 * Utilise le Vector2 de LibGDX pour la position.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class GameObject {
    protected int id;
    protected final Vector2 position = new Vector2();
    protected boolean active = true;

    public void setActive(boolean active) {
        this.active = active;
    }

    public boolean isActive() {
        return active;
    }

    public Vector2 getPosition() {
        return position;
    }

    public void update(float delta) {}
    public void render(SpriteBatch batch) {}
}
