package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.objects.Burger;
import com.irina.myfirstgame.objects.Item;

/**
 * Factory pour créer des items à partir d'identifiants.
 * <p>
 * Stub pour une fonctionnalité future de création d'items.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class ItemFactory {
    private final Assets assets;

    public ItemFactory(Assets assets) {
        this.assets = assets;
    }

    /* public Item create(String id) {
        /* switch (id) {
            case "burger":
                return new Burger(assets.getTextureRegion("burger.png"));
            default:
                return null;
        } 
    } */
}
