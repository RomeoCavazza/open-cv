package com.irina.myfirstgame.interfaces;

/**
 * Interface pour les entités qui peuvent être désactivées/retirées du jeu.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface Despawnable {
    void onDespawn();
    boolean shouldDespawn();
}

