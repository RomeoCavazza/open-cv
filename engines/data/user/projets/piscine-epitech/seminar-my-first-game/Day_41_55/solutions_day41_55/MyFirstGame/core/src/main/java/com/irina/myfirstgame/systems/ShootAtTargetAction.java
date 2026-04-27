package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.objects.Projectile;

/**
 * Action permettant à une entité de tirer un projectile vers une cible.
 * <p>
 * Crée un projectile qui se déplace en ligne droite vers la position cible.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class ShootAtTargetAction extends Action {

    private final Vector2 target;

    public ShootAtTargetAction(Vector2 target) {
        this.target = target;
    }

    @Override
    public void execute(Entity entity) {
        // Vérifie qu'on a bien un Wormy
        if (!(entity instanceof Wormy)) {
            return;
        }

        Wormy wormy = (Wormy) entity;

        // Position de départ du tir
        Vector2 wormPos = wormy.getPosition();
        Vector2 from = new Vector2(wormPos.getX(), wormPos.getY());

        // Direction ver -> cible
        Vector2 dir = new Vector2(
                target.getX() - from.getX(),
                target.getY() - from.getY()
        );
        dir.nor(); // normalisation (comme tu fais dans PlayScreen)

        // Création du projectile
        Projectile projectile = new Projectile();
        projectile.setDirection(dir);
        projectile.launch(from);

        // 💡 Ici on NE l’injecte PAS dans le monde, car on n'y a pas accès.
        // Tu pourras plus tard adapter pour que le système/monde qui appelle cette Action
        // récupère le projectile et le spawn.
        // Exemple possible plus tard :
        // wormy.getGun().onShot(projectile);
    }

    @Override
    public Vector2 getTarget() {
        return target;
    }
}
