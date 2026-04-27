package com.irina.myfirstgame.interfaces;

import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Interface pour les items qui peuvent être collectés par le ver.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface Collectible {
    void collect(Wormy by);
}

