package com.irina.myfirstgame.objects;

import com.irina.myfirstgame.interfaces.ScoreStorage;
import com.irina.myfirstgame.entities.Player;
import java.util.List;

/**
 * Gère le score et les meilleurs scores du joueur.
 * <p>
 * Stub pour une fonctionnalité future de gestion des scores.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Scoreboard {
    private int currentScore;
    private int highScore;
    private int maxCombo;
    private float comboMultiplier;
    private float lastUpdateTime;
    private List<ScoreEntry> entries;
    private Player boundPlayer;
    
    public void add(ScoreEntry entry) {
    }
    
    public void increase(int points, String source) {
    }
    
    public List<ScoreEntry> top(int n) {
        return null;
    }
    
    public void reset() {
    }
    
    public void persist(ScoreStorage storage) {
    }
}

