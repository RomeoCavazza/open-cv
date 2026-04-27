package com.irina.myfirstgame.entities.wormy;

import com.badlogic.gdx.Gdx;
import com.irina.myfirstgame.entities.valueobjects.Vector2;

/**
 * Gère l'état d'évolution du ver et le suivi des aliments consommés.
 * <p>
 * Cette classe permet de suivre l'évolution du ver entre les différents stades
 * (Baby, Adult, Super) et de gérer les conditions d'évolution basées sur
 * la consommation de certains aliments (Burger, Frites, Soda).
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class WormState {

    private boolean ateBurger;
    private boolean ateFrites;
    private boolean ateSoda;

    private Wormy current;

    /**
     * Constructeur par défaut.
     * Initialise l'état sans ver actif.
     */
    public WormState() {
        this.current = null;
    }

    /**
     * Fait évoluer le ver vers un nouvel état.
     * <p>
     * Préserve la position et la vélocité du ver précédent lors de l'évolution.
     * </p>
     *
     * @param state Le nouvel état du ver
     */
    public void evolveTo(Wormy state) {
        if (state != null) {
            Wormy previousState = this.current;
            
            if (previousState != null) {
                // IMPORTANT : Lire les valeurs directement depuis le Vector2 pour éviter les problèmes
                Vector2 prevPos = previousState.getPosition();
                Vector2 prevVel = previousState.getVelocity();
                
                float savedX = prevPos.getX();
                float savedY = prevPos.getY();
                float savedVelX = prevVel.getX();
                float savedVelY = prevVel.getY();
                
                // Copier la position et la vélocité dans le nouvel état
                // Utiliser setPosition(float, float) pour garantir la copie des valeurs
                state.setPosition(savedX, savedY);
                state.setVelocity(savedVelX, savedVelY);
                
                // Vérification immédiate après la copie (débug)
                Vector2 newPos = state.getPosition();
                if (Math.abs(newPos.getX() - savedX) > 0.001f || Math.abs(newPos.getY() - savedY) > 0.001f) {
                    Gdx.app.error("WormState", String.format(
                        "ERREUR copie position: attendu (%.2f, %.2f), obtenu (%.2f, %.2f)",
                        savedX, savedY, newPos.getX(), newPos.getY()));
                }
            }

            // Définir l'état d'évolution
            state.setEvolutionState(this);

            Gdx.app.log("WormState",
                    "Evolution: "
                            + (previousState != null ? previousState.getClass().getSimpleName() : "null")
                            + " vers "
                            + state.getClass().getSimpleName());

            // Changer current EN DERNIER
            this.current = state;
        }
    }

    /**
     * Retourne le ver actuel.
     *
     * @return Le ver actuel, ou null si aucun
     */
    public Wormy getCurrent() {
        return current;
    }

    /**
     * Vérifie si un ver est actuellement actif.
     *
     * @return true si un ver est actif, false sinon
     */
    public boolean hasCurrent() {
        return current != null;
    }

    /**
     * Enregistre la consommation d'un aliment.
     * <p>
     * Déclenche automatiquement une vérification d'évolution si nécessaire.
     * </p>
     *
     * @param foodName Le nom de l'aliment consommé
     */
    public void registerFood(String foodName) {
        if (foodName == null) {
            return;
        }

        switch (foodName.toLowerCase()) {
            case "burger":
                ateBurger = true;
                break;
            case "frites":
                ateFrites = true;
                break;
            case "soda":
                ateSoda = true;
                break;
            default:
                break;
        }
        checkEvolution();
    }

    private boolean checkEvolution() {
        if (getCurrent() instanceof Baby
                && ateBurger && ateFrites && ateSoda) {

            Gdx.app.log("WormState", "Evolution: Baby vers Adult");
            Adult adult = new Adult();
            evolveTo(adult);

            // Set health to 100 and hunger to 50 as requested
            adult.getHealth().setCurrentHealth(100);
            adult.getHunger().setCurrentHunger(50);

            return true; // Le ver est devenu adulte
        }
        return false; // Pas d'évolution
    }

    /**
     * Vérifie si le ver a évolué vers l'état Adult.
     *
     * @return true si le ver est Adult et a consommé tous les aliments requis
     */
    public boolean hasEvolvedToAdult() {
        return getCurrent() instanceof Adult && ateBurger && ateFrites && ateSoda;
    }

    /**
     * Vérifie si le Burger a été consommé.
     *
     * @return true si le Burger a été consommé
     */
    public boolean hasAteBurger() {
        return ateBurger;
    }

    /**
     * Vérifie si les Frites ont été consommées.
     *
     * @return true si les Frites ont été consommées
     */
    public boolean hasAteFrites() {
        return ateFrites;
    }

    /**
     * Vérifie si le Soda a été consommé.
     *
     * @return true si le Soda a été consommé
     */
    public boolean hasAteSoda() {
        return ateSoda;
    }

    /**
     * Réinitialise le combo d'aliments consommés.
     */
    public void resetCombo() {
        ateBurger = false;
        ateFrites = false;
        ateSoda = false;
        Gdx.app.log("WormState", "Combo reset!");
    }
}