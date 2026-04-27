package com.irina.myfirstgame.entities;

import com.irina.myfirstgame.core.input.InputController;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.entities.wormy.Adult;
import com.irina.myfirstgame.entities.wormy.Baby;
import com.irina.myfirstgame.entities.wormy.Super;
import com.irina.myfirstgame.entities.wormy.WormState;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.interfaces.Eatable;
import com.irina.myfirstgame.interfaces.Eater;

/**
 * Représente le joueur contrôlé par l'utilisateur.
 * <p>
 * Le joueur gère l'état du ver (Wormy) et traduit les entrées utilisateur
 * en mouvements et actions. Le ver peut évoluer entre différents états :
 * Baby, Adult et Super.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Player implements Eater {

    public static final float COLLISION_MARGIN_X = 6f;
    public static final float COLLISION_MARGIN_Y = 6f;

    private String name;
    private WormState worm;
    private InputController inputController;
    private float speed;
    private float width;
    private float height;

    /**
     * Constructeur par défaut.
     * Initialise le joueur avec un nom par défaut et un ver au stade Baby.
     */
    public Player() {
        this.name = "Player";
        this.speed = 80f;
        this.worm = new WormState();
        Baby baby = new Baby();
        worm.evolveTo(baby);
    }

    /**
     * Retourne le nom du joueur.
     *
     * @return Le nom du joueur
     */
    public String getName() {
        return name;
    }

    /**
     * Définit le nom du joueur.
     *
     * @param name Le nouveau nom (ne peut pas être null ou vide)
     */
    public void setName(String name) {
        if (name != null && !name.trim().isEmpty()) {
            this.name = name.trim();
        }
    }

    /**
     * Définit le contrôleur d'entrée pour gérer les commandes du joueur.
     *
     * @param inputController Le contrôleur d'entrée
     */
    public void setInputController(InputController inputController) {
        this.inputController = inputController;
    }

    /**
     * Retourne l'état du ver (WormState).
     *
     * @return L'état du ver
     */
    public WormState getWorm() {
        return worm;
    }

    /**
     * Traite les entrées utilisateur et met à jour la vélocité du ver.
     *
     * @param delta Temps écoulé depuis la dernière frame
     */
    public void handleInput(float delta) {
        if (inputController == null || worm.getCurrent() == null) {
            return;
        }

        Wormy current = worm.getCurrent();

        Vector2 vel = current.getVelocity();
        vel.set(0, 0);

        boolean movingX = false;
        boolean movingY = false;

        Baby baby = current instanceof Baby ? (Baby) current : null;
        Adult adult = current instanceof Adult ? (Adult) current : null;
        Super superWorm = current instanceof Super ? (Super) current : null;

        // Utiliser la vitesse du ver (Super a une vitesse plus élevée)
        float wormSpeed = speed;
        if (superWorm != null) {
            wormSpeed = superWorm.getSpeed();
        } else if (adult != null) {
            wormSpeed = adult.getSpeed();
        } else if (baby != null) {
            wormSpeed = baby.getSpeed();
        }

        if (inputController.isLeftPressed()) {
            vel.setX(-wormSpeed);
            movingX = true;
            if (baby != null) {
                baby.setFaceLeft(true);
            }
            if (adult != null) {
                adult.setFaceLeft(true);
            }
            if (superWorm != null) {
                superWorm.setFaceLeft(true);
            }
        }
        if (inputController.isRightPressed()) {
            vel.setX(wormSpeed);
            movingX = true;
            if (baby != null) {
                baby.setFaceLeft(false);
            }
            if (adult != null) {
                adult.setFaceLeft(false);
            }
            if (superWorm != null) {
                superWorm.setFaceLeft(false);
            }
        }

        if (inputController.isUpPressed()) {
            vel.setY(wormSpeed);
            movingY = true;
        }
        if (inputController.isDownPressed()) {
            vel.setY(-wormSpeed);
            movingY = true;
        }

        float velY = vel.getY();
        float velX = vel.getX();
        float absVelY = Math.abs(velY);
        float absVelX = Math.abs(velX);

        boolean wantUp = movingY && velY > 0 && (!movingX || absVelY >= absVelX);
        boolean wantDown = movingY && velY < 0 && (!movingX || absVelY >= absVelX);

        if (baby != null) {
            boolean wasUp = baby.isMovingUp();
            boolean wasDown = baby.isMovingDown();

            baby.setMovingUp(wantUp);
            baby.setMovingDown(wantDown);

            if (wantUp && !wasUp) {
                baby.setTurningUp(true);
                baby.setTurningDown(false);
            } else if (wantDown && !wasDown) {
                baby.setTurningDown(true);
                baby.setTurningUp(false);
            } else if (!wantUp && !wantDown) {
                baby.setTurningUp(false);
                baby.setTurningDown(false);
            }
        }

        if (adult != null) {
            boolean wasUp = adult.isMovingUp();
            boolean wasDown = adult.isMovingDown();

            adult.setMovingUp(wantUp);
            adult.setMovingDown(wantDown);

            if (wantUp && !wasUp) {
                adult.setTurningUp(true);
                adult.setTurningDown(false);
            } else if (wantDown && !wasDown) {
                adult.setTurningDown(true);
                adult.setTurningUp(false);
            } else if (!wantUp && !wantDown) {
                adult.setTurningUp(false);
                adult.setTurningDown(false);
            }
        }

        if (superWorm != null) {
            boolean wasUp = superWorm.isMovingUp();
            boolean wasDown = superWorm.isMovingDown();

            superWorm.setMovingUp(wantUp);
            superWorm.setMovingDown(wantDown);

            if (wantUp && !wasUp) {
                superWorm.setTurningUp(true);
                superWorm.setTurningDown(false);
            } else if (wantDown && !wasDown) {
                superWorm.setTurningDown(true);
                superWorm.setTurningUp(false);
            } else if (!wantUp && !wantDown) {
                superWorm.setTurningUp(false);
                superWorm.setTurningDown(false);
            }
        }
    }

    /**
     * Met à jour le joueur et son ver.
     *
     * @param delta Temps écoulé depuis la dernière frame
     */
    public void update(float delta) {
        handleInput(delta);
        if (worm.getCurrent() != null) {
            worm.getCurrent().update(delta);
        }
    }

    /**
     * Fait manger un item au ver.
     *
     * @param item L'item à manger
     */
    @Override
    public void eat(Eatable item) {
        if (item != null && worm.getCurrent() != null) {
            worm.getCurrent().eat(item);
        }
    }

    /**
     * Retourne la position du ver actuel.
     *
     * @return La position du ver, ou null si aucun ver n'est actif
     */
    public Vector2 getPosition() {
        if (worm.getCurrent() != null) {
            return worm.getCurrent().getPosition();
        }
        return null;
    }

    /**
     * Définit la position du ver actuel.
     *
     * @param x Position X
     * @param y Position Y
     */
    public void setPosition(float x, float y) {
        if (worm.getCurrent() != null) {
            worm.getCurrent().setPosition(x, y);
        }
    }

    /**
     * Retourne la largeur du joueur.
     *
     * @return La largeur
     */
    public float getWidth() {
        return width;
    }

    /**
     * Définit la largeur du joueur.
     *
     * @param width La largeur
     */
    public void setWidth(float width) {
        this.width = width;
    }

    /**
     * Retourne la hauteur du joueur.
     *
     * @return La hauteur
     */
    public float getHeight() {
        return height;
    }

    /**
     * Définit la hauteur du joueur.
     *
     * @param height La hauteur
     */
    public void setHeight(float height) {
        this.height = height;
    }

    private static final Vector2 ZERO_VELOCITY = new Vector2(0, 0);

    /**
     * Retourne la vélocité du ver actuel.
     *
     * @return La vélocité du ver, ou (0, 0) si aucun ver n'est actif
     */
    public Vector2 getVelocity() {
        if (worm.getCurrent() != null) {
            return worm.getCurrent().getVelocity();
        }
        return ZERO_VELOCITY;
    }
}
