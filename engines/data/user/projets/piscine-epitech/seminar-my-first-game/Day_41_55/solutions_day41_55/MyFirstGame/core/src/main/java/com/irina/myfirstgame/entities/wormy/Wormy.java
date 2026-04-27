package com.irina.myfirstgame.entities.wormy;

import java.util.List;

import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.interfaces.Damageable;
import com.irina.myfirstgame.interfaces.Eater;
import com.irina.myfirstgame.interfaces.Movable;
import com.irina.myfirstgame.stats.Health;
import com.irina.myfirstgame.stats.Hunger;
import com.irina.myfirstgame.stats.StatusEffect;
import com.irina.myfirstgame.systems.Sprite;

/**
 * Classe abstraite représentant le ver jouable (Wormy).
 * <p>
 * Wormy est l'entité principale contrôlée par le joueur. Il peut évoluer
 * entre différents stades (Baby, Adult, Super) et possède des statistiques
 * de santé, faim et des effets de statut.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class Wormy extends Entity implements Movable, Damageable, Eater {
    private Health health;
    private Hunger hunger;
    private List<StatusEffect> statusEffects;
    private WormState evolutionState;
    
    /**
     * Constructeur par défaut.
     * Initialise la santé, la faim et la liste des effets de statut.
     */
    public Wormy() {
        super();
        this.health = new Health();
        this.hunger = new Hunger();
        this.statusEffects = new java.util.ArrayList<>();
    }
    
    /**
     * Retourne la santé du ver.
     *
     * @return L'objet Health
     */
    public Health getHealth() {
        return health;
    }
    
    /**
     * Retourne la faim du ver.
     *
     * @return L'objet Hunger
     */
    public Hunger getHunger() {
        return hunger;
    }
    
    /**
     * Retourne la liste des effets de statut actifs.
     *
     * @return La liste des StatusEffect
     */
    public List<StatusEffect> getStatusEffects() {
        return statusEffects;
    }
    
    /**
     * Permet au ver de s'enfouir dans le sol.
     * À implémenter dans les sous-classes.
     */
    public void burrow() {}
    
    /**
     * Permet au ver de remonter à la surface.
     * À implémenter dans les sous-classes.
     */
    public void surface() {}
    
    /**
     * Déplace le ver.
     * À implémenter dans les sous-classes.
     */
    @Override
    public void move() {}
    
    /**
     * Inflige des dégâts au ver.
     *
     * @param amount Le montant de dégâts
     */
    @Override
    public void takeDamage(int amount) {
        if (health != null) {
            health.takeDamage(amount);
        }
    }
    
    /**
     * Fait manger un item au ver.
     *
     * @param item L'item à manger
     */
    @Override
    public void eat(com.irina.myfirstgame.interfaces.Eatable item) {
        if (item != null) {
            item.onEatenBy(this);
        }
    }

    /**
     * Retourne l'état d'évolution du ver.
     *
     * @return Le WormState
     */
    public WormState getEvolutionState() {
        return evolutionState;
    }

    /**
     * Définit l'état d'évolution du ver.
     *
     * @param state Le nouvel état d'évolution
     */
    public void setEvolutionState(WormState state) {
        this.evolutionState = state;
    }

    /**
     * Retourne le sprite du ver.
     *
     * @return Le sprite
     */
    public abstract Sprite getSprite();
    
    /**
     * Définit le sprite du ver.
     *
     * @param sprite Le nouveau sprite
     */
    public abstract void setSprite(Sprite sprite);
}

