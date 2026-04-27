package com.irina.myfirstgame.systems;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Preferences;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Manages the score system with persistent high scores per player.
 */
public class ScoreManager {
    private static final String PREFS_NAME = "highscores";
    private static final int MAX_TOP_SCORES = 5;

    private int currentScore;
    private final Preferences preferences;

    public ScoreManager() {
        this.currentScore = 0;
        this.preferences = Gdx.app.getPreferences(PREFS_NAME);
    }

    /**
     * Add points to the current score
     */
    public void addPoints(int points) {
        if (points > 0) {
            currentScore += points;
            Gdx.app.log("ScoreManager", "Added " + points + " points. Total: " + currentScore);
        }
    }

    /**
     * Get the current score for this game session
     */
    public int getCurrentScore() {
        return currentScore;
    }

    /**
     * Reset the current score (for new game)
     */
    public void resetScore() {
        currentScore = 0;
    }

    /**
     * Save the current score for a player if it's a new high score
     * 
     * @param playerName The player's name
     * @return true if it's a new high score, false otherwise
     */
    public boolean saveScore(String playerName) {
        if (playerName == null || playerName.trim().isEmpty()) {
            playerName = "Player";
        }

        int previousHighScore = getPlayerHighScore(playerName);

        if (currentScore > previousHighScore) {
            preferences.putInteger(playerName, currentScore);
            preferences.flush();
            Gdx.app.log("ScoreManager", "New high score for " + playerName + ": " + currentScore);
            return true;
        }

        return false;
    }

    /**
     * Get the high score for a specific player
     * 
     * @param playerName The player's name
     * @return The player's high score, or 0 if they haven't played before
     */
    public int getPlayerHighScore(String playerName) {
        if (playerName == null || playerName.trim().isEmpty()) {
            playerName = "Player";
        }
        return preferences.getInteger(playerName, 0);
    }

    /**
     * Get the top N scores across all players
     * 
     * @return List of PlayerScore objects sorted by score (highest first)
     */
    public List<PlayerScore> getTopScores() {
        Map<String, ?> allScores = preferences.get();
        List<PlayerScore> scores = new ArrayList<>();

        for (Map.Entry<String, ?> entry : allScores.entrySet()) {
            if (entry.getValue() instanceof Integer) {
                scores.add(new PlayerScore(entry.getKey(), (Integer) entry.getValue()));
            }
        }

        // Sort by score descending
        Collections.sort(scores, new Comparator<PlayerScore>() {
            @Override
            public int compare(PlayerScore a, PlayerScore b) {
                return Integer.compare(b.score, a.score);
            }
        });

        // Return only top N
        if (scores.size() > MAX_TOP_SCORES) {
            return scores.subList(0, MAX_TOP_SCORES);
        }

        return scores;
    }

    /**
     * Clear all saved scores (for testing or reset)
     */
    public void clearAllScores() {
        preferences.clear();
        preferences.flush();
        Gdx.app.log("ScoreManager", "All scores cleared");
    }

    /**
     * Simple data class to hold player name and score
     */
    public static class PlayerScore {
        public final String playerName;
        public final int score;

        public PlayerScore(String playerName, int score) {
            this.playerName = playerName;
            this.score = score;
        }
    }
}
