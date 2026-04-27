package com.irina.myfirstgame.world.tiles;

import com.irina.myfirstgame.world.Tile;
import com.irina.myfirstgame.interfaces.Diggable;

/**
 * Tuile de terre creusable.
 * <p>
 * Stub pour une fonctionnalité future de creusage.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Dirt extends Tile implements Diggable {
    public float hardness;
    
    @Override
    public void dig() {
    }
}

