package com.irina.myfirstgame.interfaces;

import com.irina.myfirstgame.entities.wormy.Wormy;

/**
 * Interface pour les items qui peuvent être mangés par le ver.
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public interface Eatable {
    void onEatenBy(Wormy target);
}

