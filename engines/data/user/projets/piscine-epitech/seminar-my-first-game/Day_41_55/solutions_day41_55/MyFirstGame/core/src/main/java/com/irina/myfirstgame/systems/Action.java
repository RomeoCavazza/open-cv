package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.entities.valueobjects.Vector2;

/**
 * Classe de base pour les actions exécutables par les entités.
 * <p>
 * Utilisée par le système d'IA pour définir des comportements.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Action {
    public void execute(Entity entity) {
    }
    
    public Vector2 getTarget() {
        return null;
    }
}

