package com.irina.myfirstgame.entities.valueobjects;

/**
 * Représente un identifiant unique universel (UUID).
 * <p>
 * Cette classe encapsule une chaîne UUID générée aléatoirement ou fournie.
 * Utilisée pour identifier de manière unique les entités du jeu.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class UUID {
    private String value;
    
    /**
     * Constructeur par défaut.
     * Génère un UUID aléatoire.
     */
    public UUID() {
        this.value = java.util.UUID.randomUUID().toString();
    }
    
    /**
     * Constructeur avec valeur fournie.
     *
     * @param value La valeur UUID
     */
    public UUID(String value) {
        this.value = value;
    }
    
    /**
     * Retourne la valeur de l'UUID.
     *
     * @return La valeur UUID sous forme de chaîne
     */
    public String getValue() {
        return value;
    }
    
    @Override
    public String toString() {
        return value;
    }
    
    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        UUID uuid = (UUID) obj;
        return value != null ? value.equals(uuid.value) : uuid.value == null;
    }
    
    @Override
    public int hashCode() {
        return value != null ? value.hashCode() : 0;
    }
}

