package com.irina.myfirstgame.entities;

import com.irina.myfirstgame.entities.valueobjects.UUID;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.interfaces.Collidable;
import com.irina.myfirstgame.interfaces.Stateful;

/**
 * Classe abstraite de base pour toutes les entités du jeu.
 * <p>
 * Représente une entité avec une position, une vélocité, une taille et un identifiant unique.
 * Implémente les interfaces {@link Collidable} et {@link Stateful} pour gérer les collisions
 * et les états.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class Entity implements Collidable, Stateful {
    private UUID id;
    private Vector2 position;
    private Vector2 velocity;
    private float width;
    private float height;

    /**
     * Constructeur par défaut.
     * Initialise la position et la vélocité à (0, 0) et génère un identifiant unique.
     */
    public Entity() {
        this.position = new Vector2();
        this.velocity = new Vector2();
        this.id = new UUID();
    }

    /**
     * Constructeur avec position initiale.
     *
     * @param x Position X initiale
     * @param y Position Y initiale
     */
    public Entity(float x, float y) {
        this();
        this.position.set(x, y);
    }

    /**
     * Met à jour l'entité à chaque frame.
     *
     * @param delta Temps écoulé depuis la dernière frame (en secondes)
     */
    public abstract void update(float delta);

    /**
     * Retourne la position de l'entité.
     *
     * @return La position (Vector2)
     */
    public Vector2 getPosition() {
        return position;
    }

    /**
     * Définit la position de l'entité.
     *
     * @param x Position X
     * @param y Position Y
     */
    public void setPosition(float x, float y) {
        position.set(x, y);
    }

    /**
     * Définit la position de l'entité.
     *
     * @param pos La nouvelle position
     */
    public void setPosition(Vector2 pos) {
        position.set(pos);
    }

    /**
     * Retourne la vélocité de l'entité.
     *
     * @return La vélocité (Vector2)
     */
    public Vector2 getVelocity() {
        return velocity;
    }

    /**
     * Définit la vélocité de l'entité.
     *
     * @param x Vélocité X
     * @param y Vélocité Y
     */
    public void setVelocity(float x, float y) {
        velocity.set(x, y);
    }

    /**
     * Retourne la largeur de l'entité.
     *
     * @return La largeur
     */
    public float getWidth() {
        return width;
    }

    /**
     * Définit la largeur de l'entité.
     *
     * @param width La largeur
     */
    public void setWidth(float width) {
        this.width = width;
    }

    /**
     * Retourne la hauteur de l'entité.
     *
     * @return La hauteur
     */
    public float getHeight() {
        return height;
    }

    /**
     * Définit la hauteur de l'entité.
     *
     * @param height La hauteur
     */
    public void setHeight(float height) {
        this.height = height;
    }

    /**
     * Retourne l'identifiant unique de l'entité.
     *
     * @return L'UUID de l'entité
     */
    public UUID getId() {
        return id;
    }

    /**
     * Appelé lors d'une collision avec une autre entité collidable.
     * <p>
     * À implémenter dans les sous-classes pour définir le comportement spécifique.
     * </p>
     *
     * @param other L'autre entité en collision
     */
    @Override
    public void onCollision(Collidable other) {
        // À implémenter dans les sous-classes
    }

    /**
     * Retourne l'état actuel de l'entité.
     *
     * @return L'état sous forme de chaîne (par défaut "idle")
     */
    @Override
    public String getState() {
        return "idle";
    }
}

