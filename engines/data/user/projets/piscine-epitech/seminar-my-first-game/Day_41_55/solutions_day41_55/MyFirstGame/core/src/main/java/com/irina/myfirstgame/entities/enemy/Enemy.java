package com.irina.myfirstgame.entities.enemy;

import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.interfaces.Damageable;
import com.irina.myfirstgame.interfaces.Movable;
import com.irina.myfirstgame.stats.Health;

/**
 * Classe abstraite de base pour tous les ennemis du jeu.
 * <p>
 * Les ennemis sont des entités hostiles qui peuvent se déplacer et subir des dégâts.
 * Par défaut, ils ont 50 points de vie.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public abstract class Enemy extends Entity implements Movable, Damageable {
    protected Health health;
    
    /**
     * Constructeur par défaut.
     * Initialise l'ennemi avec 50 points de vie maximum.
     */
    public Enemy() {
        super();
        this.health = new Health();
        // Par défaut, les ennemis ont 50 points de vie (tués en un coup par projectiles de 100 dégâts)
        this.health.setMaxHealth(50);
    }

    /**
     * Constructeur avec position initiale.
     *
     * @param x Position X initiale
     * @param y Position Y initiale
     */
    public Enemy(float x, float y) {
        super(x, y);
        this.health = new Health();
        this.health.setMaxHealth(50);
    }
    
    /**
     * Déplace l'ennemi.
     * Le mouvement est géré par les sous-classes via la méthode update().
     */
    @Override
    public void move() {
        // Mouvement géré par les sous-classes via update()
    }
    
    /**
     * Inflige des dégâts à l'ennemi.
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
     * Vérifie si l'ennemi est mort.
     *
     * @return true si l'ennemi est mort, false sinon
     */
    public boolean isDead() {
        return health != null && health.isDead();
    }
    
    /**
     * Retourne la santé de l'ennemi.
     *
     * @return L'objet Health
     */
    public Health getHealth() {
        return health;
    }
    
    /**
     * Dessine l'ennemi.
     * À implémenter dans les sous-classes.
     *
     * @param batch Le SpriteBatch pour le rendu
     */
    public void render(SpriteBatch batch) {
        // À implémenter dans les sous-classes
    }
}

