package com.irina.myfirstgame.stats;

public class Health {
    private int currentHealth;
    private int maxHealth;
    
    public void takeDamage(int amount) {
        currentHealth -= amount;
        if (currentHealth < 0) {
            currentHealth = 0;
        }
    }
    
    public void heal(int amount) {
        currentHealth += amount;
        if (currentHealth > maxHealth) {
            currentHealth = maxHealth;
        }
    }
    
    public boolean isDead() {
        return currentHealth <= 0;
    }

    public void setMaxHealth(int maxHealth) {
        this.maxHealth = maxHealth;
        this.currentHealth = maxHealth;
    }

    public void setCurrentHealth(int health) {
        this.currentHealth = health;
        if (this.currentHealth > maxHealth) {
            this.currentHealth = maxHealth;
        }
        if (this.currentHealth < 0) {
            this.currentHealth = 0;
        }
    }

    public int getCurrentHealth() {
        return currentHealth;
    }

    public int getMaxHealth() {
        return maxHealth;
    }
}

