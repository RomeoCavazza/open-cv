package com.irina.myfirstgame.entities.valueobjects;

/**
 * Représente un vecteur 2D avec des coordonnées x et y.
 * <p>
 * Cette classe fournit des opérations mathématiques de base sur les vecteurs
 * (addition, multiplication par scalaire, normalisation, calcul de longueur).
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Vector2 {
    private float x;
    private float y;
    
    /**
     * Constructeur par défaut.
     * Initialise le vecteur à (0, 0).
     */
    public Vector2() {
        this(0, 0);
    }
    
    /**
     * Constructeur avec coordonnées.
     *
     * @param x Coordonnée X
     * @param y Coordonnée Y
     */
    public Vector2(float x, float y) {
        this.x = x;
        this.y = y;
    }
    
    /**
     * Constructeur de copie.
     *
     * @param other Le vecteur à copier
     */
    public Vector2(Vector2 other) {
        this(other.x, other.y);
    }
    
    /**
     * Retourne la coordonnée X.
     *
     * @return La coordonnée X
     */
    public float getX() {
        return x;
    }
    
    /**
     * Définit la coordonnée X.
     *
     * @param x La nouvelle coordonnée X
     */
    public void setX(float x) {
        this.x = x;
    }
    
    /**
     * Retourne la coordonnée Y.
     *
     * @return La coordonnée Y
     */
    public float getY() {
        return y;
    }
    
    /**
     * Définit la coordonnée Y.
     *
     * @param y La nouvelle coordonnée Y
     */
    public void setY(float y) {
        this.y = y;
    }
    
    /**
     * Définit les coordonnées du vecteur.
     *
     * @param x La coordonnée X
     * @param y La coordonnée Y
     */
    public void set(float x, float y) {
        this.x = x;
        this.y = y;
    }
    
    /**
     * Copie les coordonnées d'un autre vecteur.
     *
     * @param other Le vecteur source
     */
    public void set(Vector2 other) {
        this.x = other.x;
        this.y = other.y;
    }
    
    /**
     * Ajoute des valeurs au vecteur.
     *
     * @param x Valeur X à ajouter
     * @param y Valeur Y à ajouter
     * @return Ce vecteur pour le chaînage
     */
    public Vector2 add(float x, float y) {
        this.x += x;
        this.y += y;
        return this;
    }
    
    /**
     * Ajoute un autre vecteur à ce vecteur.
     *
     * @param other Le vecteur à ajouter
     * @return Ce vecteur pour le chaînage
     */
    public Vector2 add(Vector2 other) {
        this.x += other.x;
        this.y += other.y;
        return this;
    }
    
    /**
     * Multiplie le vecteur par un scalaire.
     *
     * @param scalar Le scalaire
     * @return Ce vecteur pour le chaînage
     */
    public Vector2 scl(float scalar) {
        this.x *= scalar;
        this.y *= scalar;
        return this;
    }
    
    /**
     * Calcule la longueur du vecteur.
     *
     * @return La longueur du vecteur
     */
    public float len() {
        return (float) Math.sqrt(x * x + y * y);
    }
    
    /**
     * Calcule la longueur au carré du vecteur (plus rapide que len()).
     *
     * @return La longueur au carré
     */
    public float len2() {
        return x * x + y * y;
    }
    
    /**
     * Normalise le vecteur (le transforme en vecteur unitaire).
     *
     * @return Ce vecteur pour le chaînage
     */
    public Vector2 nor() {
        float len = len();
        if (len != 0) {
            this.x /= len;
            this.y /= len;
        }
        return this;
    }
    
    @Override
    public String toString() {
        return "Vector2(" + x + ", " + y + ")";
    }
}

