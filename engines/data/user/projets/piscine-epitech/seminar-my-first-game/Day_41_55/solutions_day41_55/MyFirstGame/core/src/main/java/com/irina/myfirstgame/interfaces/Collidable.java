package com.irina.myfirstgame.interfaces;

/**
 * Interface pour les entités qui peuvent entrer en collision avec d'autres entités.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface Collidable {
    void onCollision(Collidable other);
}

