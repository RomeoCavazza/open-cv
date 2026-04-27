package com.irina.myfirstgame.objects;

/**
 * Représente une entrée de score dans le tableau des meilleurs scores.
 * <p>
 * Contient les informations sur un score : nom du joueur, points, source,
 * timestamp, temps de survie et évolution maximale atteinte.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class ScoreEntry {
    public String player;
    public int points;
    public String source;
    public float timestamp;
    public int timeSurvived;
    public String maxEvolution;
}

