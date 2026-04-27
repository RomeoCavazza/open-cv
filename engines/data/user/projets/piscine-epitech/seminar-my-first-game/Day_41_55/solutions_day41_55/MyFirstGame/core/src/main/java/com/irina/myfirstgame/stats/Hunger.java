package com.irina.myfirstgame.stats;

public class Hunger {
    private int currentHunger;
    private int maxHunger;
    
    public void decrease(int amount) {
        currentHunger -= amount;
        if (currentHunger < 0) {
            currentHunger = 0;
        }
    }
    
    public void increase(int amount) {
        currentHunger += amount;
        if (currentHunger > maxHunger) {
            currentHunger = maxHunger;
        }
    }
    
    public boolean isStarving() {
        return currentHunger <= 0;
    }

    public void setMaxHunger(int maxHunger) {
        this.maxHunger = maxHunger;
        this.currentHunger = maxHunger;
    }   
    
    public void setCurrentHunger(int hunger) {
        this.currentHunger = hunger;
        if (this.currentHunger > maxHunger) {
            this.currentHunger = maxHunger;
        }
        if (this.currentHunger < 0) {
            this.currentHunger = 0;
        }
    }

    public int getCurrentHunger() {
        return currentHunger;
    }

    public int getMaxHunger() {
        return maxHunger;
    }
}

