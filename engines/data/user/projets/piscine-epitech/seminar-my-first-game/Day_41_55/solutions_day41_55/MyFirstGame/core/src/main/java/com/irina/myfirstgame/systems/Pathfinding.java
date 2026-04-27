package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.world.Map;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.List;

/**
 * Système de pathfinding utilisant l'algorithme BFS (Breadth-First Search).
 * <p>
 * Calcule le chemin le plus court entre deux points sur la carte en évitant
 * les obstacles solides.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Pathfinding {

    private final Map map;
    private final String collisionLayer;

    public Pathfinding(Map map, String collisionLayer) {
        this.map = map;
        this.collisionLayer = collisionLayer;
    }

    public Path compute(Vector2 from, Vector2 to) {
        if (map == null || from == null || to == null) {
            return new Path();
        }

        int tileWidth = map.getTileWidth();
        int tileHeight = map.getTileHeight();

        int startX = clampToGrid((int) (from.getX() / tileWidth), map.getMapWidthPixels() / tileWidth);
        int startY = clampToGrid((int) (from.getY() / tileHeight), map.getMapHeightPixels() / tileHeight);
        int goalX = clampToGrid((int) (to.getX() / tileWidth), map.getMapWidthPixels() / tileWidth);
        int goalY = clampToGrid((int) (to.getY() / tileHeight), map.getMapHeightPixels() / tileHeight);

        if (!isWalkable(goalX, goalY)) {
            return new Path();
        }

        int gridWidth = map.getMapWidthPixels() / tileWidth;
        int gridHeight = map.getMapHeightPixels() / tileHeight;

        boolean[][] visited = new boolean[gridWidth][gridHeight];
        Node[][] parents = new Node[gridWidth][gridHeight];

        Deque<Node> queue = new ArrayDeque<>();
        queue.add(new Node(startX, startY));
        visited[startX][startY] = true;

        int[] dx = {1, -1, 0, 0};
        int[] dy = {0, 0, 1, -1};

        boolean found = false;
        while (!queue.isEmpty()) {
            Node current = queue.poll();
            if (current.x == goalX && current.y == goalY) {
                found = true;
                break;
            }
            for (int dir = 0; dir < 4; dir++) {
                int nx = current.x + dx[dir];
                int ny = current.y + dy[dir];
                if (nx < 0 || ny < 0 || nx >= gridWidth || ny >= gridHeight) {
                    continue;
                }
                if (visited[nx][ny] || !isWalkable(nx, ny)) {
                    continue;
                }
                visited[nx][ny] = true;
                parents[nx][ny] = current;
                queue.add(new Node(nx, ny));
            }
        }

        if (!found) {
            return new Path();
        }

        List<Vector2> points = new ArrayList<>();
        Node current = new Node(goalX, goalY);
        while (current != null) {
            float cx = (current.x + 0.5f) * tileWidth;
            float cy = (current.y + 0.5f) * tileHeight;
            points.add(0, new Vector2(cx, cy));
            current = parents[current.x][current.y];
        }

        return new Path(points);
    }

    private boolean isWalkable(int tileX, int tileY) {
        float worldX = (tileX + 0.5f) * map.getTileWidth();
        float worldY = (tileY + 0.5f) * map.getTileHeight();
        return !map.isSolidAt(worldX, worldY, collisionLayer);
    }

    private int clampToGrid(int value, int max) {
        return Math.max(0, Math.min(value, max - 1));
    }

    private static class Node {
        final int x;
        final int y;

        Node(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }
}

