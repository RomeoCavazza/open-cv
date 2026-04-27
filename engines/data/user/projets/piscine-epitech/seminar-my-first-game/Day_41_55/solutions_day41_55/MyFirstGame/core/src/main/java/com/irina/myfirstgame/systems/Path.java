package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.entities.valueobjects.Vector2;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

/**
 * Représente un chemin composé de waypoints (points de passage).
 * <p>
 * Utilisé par le système de pathfinding pour stocker et parcourir
 * une séquence de points menant d'un point de départ à une destination.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Path {
    private final Queue<Vector2> waypoints = new LinkedList<>();

    public Path() {
    }

    public Path(List<Vector2> points) {
        if (points != null) {
            for (Vector2 point : points) {
                // Cloner pour éviter les mutations externes
                waypoints.add(new Vector2(point));
            }
        }
    }

    public void addWaypoint(Vector2 point) {
        if (point != null) {
            waypoints.add(new Vector2(point));
        }
    }

    public boolean isEmpty() {
        return waypoints.isEmpty();
    }

    public Vector2 peek() {
        return waypoints.peek();
    }

    public Vector2 next() {
        return waypoints.poll();
    }

    public void clear() {
        waypoints.clear();
    }
    
    public int size() {
        return waypoints.size();
    }
}

