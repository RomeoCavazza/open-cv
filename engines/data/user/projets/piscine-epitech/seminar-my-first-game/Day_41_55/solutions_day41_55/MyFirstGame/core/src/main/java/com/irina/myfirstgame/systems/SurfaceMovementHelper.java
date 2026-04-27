package com.irina.myfirstgame.systems;

import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.world.Map;

/**
 * Classe utilitaire pour centraliser la logique de mouvement et de gestion de surface
 * partagée entre Ant et Spider.
 */
public class SurfaceMovementHelper {
    
    /**
     * Calcule le niveau du bas de l'herbe (surfaceLevel - tileHeight)
     */
    public static float getGrassBottom(float surfaceLevel, Map map) {
        return surfaceLevel - map.getTileHeight();
    }
    
    /**
     * Vérifie si une position est à la surface (Y >= surfaceLevel - tileHeight)
     */
    public static boolean isAtSurface(float y, float height, float surfaceLevel, Map map) {
        float grassBottom = getGrassBottom(surfaceLevel, map);
        float bottomY = y - height * 0.5f;
        return bottomY >= grassBottom - map.getTileHeight() * 0.5f;
    }
    
    /**
     * Détecte si une position est dans ou près d'un tuyau vertical.
     * Ne détecte que les tuyaux VERTICAUX près de la surface, PAS les tunnels horizontaux.
     */
    public static boolean isInPipe(float x, float y, float width, float height, 
                                   float surfaceLevel, Map map, String collisionLayer) {
        float grassBottom = getGrassBottom(surfaceLevel, map);
        float halfWidth = width * 0.5f;
        float halfHeight = height * 0.5f;
        float bottomY = y - halfHeight;
        
        // IMPORTANT : On est dans un tuyau seulement si on est près de la surface
        float distanceFromSurface = bottomY - grassBottom;
        
        // Si on est trop bas (plus de 2 tuiles sous la surface), on est dans le tunnel, pas dans le tuyau
        if (distanceFromSurface < -map.getTileHeight() * 2f) {
            return false;
        }
        
        // Zone plus large pour détecter la zone autour du tuyau (comme Wormy)
        float pipeDetectionWidth = halfWidth + 8f;
        
        // Vérifier à plusieurs hauteurs pour détecter les tuyaux verticaux
        float checkY1 = y; // Centre
        float checkY2 = y - halfHeight + 5f; // Bas
        float checkY3 = y + halfHeight - 5f; // Haut
        
        boolean leftWall1 = map.isSolidAt(x - pipeDetectionWidth, checkY1, collisionLayer);
        boolean rightWall1 = map.isSolidAt(x + pipeDetectionWidth, checkY1, collisionLayer);
        boolean leftWall2 = map.isSolidAt(x - pipeDetectionWidth, checkY2, collisionLayer);
        boolean rightWall2 = map.isSolidAt(x + pipeDetectionWidth, checkY2, collisionLayer);
        boolean leftWall3 = map.isSolidAt(x - pipeDetectionWidth, checkY3, collisionLayer);
        boolean rightWall3 = map.isSolidAt(x + pipeDetectionWidth, checkY3, collisionLayer);
        
        boolean hasLeftWall = leftWall1 || leftWall2 || leftWall3;
        boolean hasRightWall = rightWall1 || rightWall2 || rightWall3;
        
        // Si on a un mur à gauche OU à droite ET qu'on est près de la surface, on est dans la zone du tuyau vertical
        return hasLeftWall || hasRightWall;
    }
    
    /**
     * Vérifie si une position est dans la zone de surface (tolérance seulement en dessous pour sortir du tunnel).
     * Si on est au-dessus de l'herbe, on doit être forcé au sol (pas de flottement).
     */
    public static boolean isInSurfaceZone(float y, float height, float surfaceLevel, Map map) {
        float grassBottom = getGrassBottom(surfaceLevel, map);
        float bottomY = y - height * 0.5f;
        float tolerance = map.getTileHeight() * 0.5f;
        return bottomY >= grassBottom - tolerance;
    }
    
    /**
     * Calcule la position Y au niveau du sol (grassBottom + halfHeight)
     */
    public static float getGroundY(float height, float surfaceLevel, Map map) {
        float grassBottom = getGrassBottom(surfaceLevel, map);
        float halfHeight = height * 0.5f;
        return grassBottom + halfHeight;
    }
    
    /**
     * Ajuste un waypoint pour qu'il soit au niveau du sol si on est dans la zone de surface et pas dans un tuyau.
     * Modifie le waypoint en place si nécessaire.
     * @param waypoint Le waypoint à ajuster (modifié en place si nécessaire)
     * @param x Position X actuelle
     * @param y Position Y actuelle
     * @param width Largeur de l'entité
     * @param height Hauteur de l'entité
     * @param surfaceLevel Niveau de la surface
     * @param map La carte
     * @param collisionLayer Le calque de collision
     */
    public static void adjustWaypointForSurface(Vector2 waypoint, float x, float y, 
                                                float width, float height, 
                                                float surfaceLevel, Map map, String collisionLayer) {
        if (!isAtSurface(y, height, surfaceLevel, map)) {
            return;
        }
        
        boolean inPipe = isInPipe(waypoint.getX(), waypoint.getY(), width, height, surfaceLevel, map, collisionLayer);
        boolean inSurfaceZone = isInSurfaceZone(waypoint.getY(), height, surfaceLevel, map);
        
        if (!inPipe && inSurfaceZone) {
            float groundY = getGroundY(height, surfaceLevel, map);
            waypoint.setY(groundY);
        }
    }
    
    /**
     * Ajuste une position Y pour qu'elle soit au niveau du sol si on est dans la zone de surface et pas dans un tuyau.
     * Retourne la position Y ajustée ou la position Y originale si aucun ajustement n'est nécessaire.
     */
    public static float adjustYForSurface(float x, float y, float width, float height,
                                         float surfaceLevel, Map map, String collisionLayer) {
        if (!isAtSurface(y, height, surfaceLevel, map)) {
            return y;
        }
        
        boolean inPipe = isInPipe(x, y, width, height, surfaceLevel, map, collisionLayer);
        boolean inSurfaceZone = isInSurfaceZone(y, height, surfaceLevel, map);
        
        if (!inPipe && inSurfaceZone) {
            return getGroundY(height, surfaceLevel, map);
        }
        
        return y;
    }
    
    /**
     * Ajuste le mouvement vertical (dy) pour bloquer le mouvement vertical si on est sur l'herbe horizontale.
     * Retourne dy ajusté (0f si sur l'herbe, dy original sinon).
     */
    public static float adjustVerticalMovement(float dy, float x, float y,
                                              float width, float height,
                                              float surfaceLevel, Map map, String collisionLayer) {
        if (!isAtSurface(y, height, surfaceLevel, map)) {
            return dy;
        }
        
        boolean inPipe = isInPipe(x, y, width, height, surfaceLevel, map, collisionLayer);
        boolean inSurfaceZone = isInSurfaceZone(y, height, surfaceLevel, map);
        
        if (!inPipe && inSurfaceZone) {
            // Sur l'herbe horizontale : bloquer le mouvement vertical
            return 0f;
        }
        
        // Si dans un tuyau OU en train de sortir du tunnel, on laisse dy tel quel
        return dy;
    }
}

