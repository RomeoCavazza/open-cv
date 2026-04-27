package com.irina.myfirstgame.interfaces;

import com.badlogic.gdx.math.Vector2;

/**
 * Interface pour les entités qui peuvent être spawnées à une position donnée.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface Spawnable {
    void spawn(Vector2 at);
    default void onSpawn() {}
}

