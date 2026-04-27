package com.irina.myfirstgame.world;

/**
 * Classe abstraite représentant une tuile du monde.
 * <p>
 * Les tuiles peuvent être bloquantes ou creusables selon leur type.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class Tile {
    public boolean isBlocking() {
        return false;
    }
    
    public boolean isDiggable() {
        return false;
    }
}

