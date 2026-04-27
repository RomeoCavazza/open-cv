package com.irina.myfirstgame.systems;

import com.badlogic.gdx.maps.tiled.TiledMapTileLayer;
import com.irina.myfirstgame.entities.Entity;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.interfaces.Collidable;
import com.irina.myfirstgame.world.Map;

public class CollisionHandler {
    private Map map;
    private String collisionLayerName;
    
    public CollisionHandler(Map map, String collisionLayerName) {
        this.map = map;
        this.collisionLayerName = collisionLayerName;
    }
    
    public void resolve(Entity entity, Collidable other) {
        if (entity != null && other != null) {
            entity.onCollision(other);
        }
    }
    
    /**
     * Force un ennemi au sol s'il est dans les airs à la surface (comme pour le joueur)
     * surfaceLevel est le HAUT de l'herbe, donc le BAS de l'herbe est à surfaceLevel - tileHeight
     * Force exactement au niveau du BAS de l'herbe pour que Ant et Spider rampent comme Wormy
     * MAIS seulement si l'ennemi est sur l'herbe horizontale, PAS sur un tuyau vertical (pour permettre l'escalade)
     * S'applique TOUJOURS si l'ennemi est au-dessus de la surface (même très haut)
     * Ne s'applique PAS si l'ennemi est en dessous ou au niveau de la surface (dans les tunnels)
     */
    public void constrainToGround(Entity entity, float surfaceLevel) {
        if (map == null || entity == null || collisionLayerName == null) return;
        
        Vector2 pos = entity.getPosition();
        float halfHeight = entity.getHeight() * 0.5f;
        float bottomY = pos.getY() - halfHeight;
        
        // Calculer le bas de l'herbe (surfaceLevel est le haut de l'herbe)
        float grassBottom = surfaceLevel - map.getTileHeight();
        
        // Réutiliser SurfaceMovementHelper pour détecter les tuyaux
        boolean inPipe = SurfaceMovementHelper.isInPipe(
            pos.getX(), pos.getY(), entity.getWidth(), entity.getHeight(),
            surfaceLevel, map, collisionLayerName);
        
        // FORCER au sol si on est sur l'herbe horizontale (pas dans le tunnel, pas dans le tuyau)
        // Logique : tolérance seulement EN DESSOUS pour permettre la sortie du tunnel
        // Mais si on est AU-DESSUS de l'herbe, on force immédiatement au sol (pas de flottement)
        float tolerance = map.getTileHeight() * 0.5f; // Tolérance pour permettre la sortie du tunnel (en dessous seulement)
        
        // On est dans la zone de surface si :
        // - On est au niveau de l'herbe ou légèrement en dessous (pour sortir du tunnel)
        // - OU on est au-dessus de l'herbe (doit être forcé au sol)
        boolean inSurfaceZone = bottomY >= grassBottom - tolerance;
        
        // Si on est dans la zone de surface ET qu'on n'est PAS dans un tuyau, FORCER au niveau du bas de l'herbe
        // Cela permet aux insectes de monter depuis le tunnel, mais les force au sol une fois sur l'herbe
        if (inSurfaceZone && !inPipe) {
            // Forcer exactement au niveau du bas de l'herbe (comme le joueur)
            // Pas de recherche de sol réel, on force directement au niveau de l'herbe
            float groundY = grassBottom + halfHeight;
            pos.setY(groundY);
        }
    }
    
    public void moveAndCollide(com.irina.myfirstgame.entities.Player player, float delta) {
        if (map == null || player == null) return;
        
        Vector2 pos = player.getPosition();
        Vector2 vel = player.getVelocity();
        float width = player.getWidth();
        float height = player.getHeight();
        
        // IMPORTANT: pos est le CENTRE de la hitbox
        // Hitbox va de (pos.x - halfWidth, pos.y - halfHeight) à (pos.x + halfWidth, pos.y + halfHeight)
        float halfWidth = width * 0.5f;
        float halfHeight = height * 0.5f;
        
        // Calculer la position suivante (toujours le centre)
        float nextX = pos.getX() + vel.getX() * delta;
        float nextY = pos.getY() + vel.getY() * delta;
        
        // === COLLISION X (horizontal) ===
        // Vérifier plusieurs points pour les entités plus grandes (comme Adult)
        // Pour permettre le mouvement horizontal dans les tunnels même avec une hitbox plus grande
        if (vel.getX() != 0) {
            float sign = Math.signum(vel.getX());
            float frontX = nextX + (sign * halfWidth);
            
            // Vérifier un seul point au centre pour les hitboxes fines (comme Baby et Adult réduit)
            // Utiliser le centre uniquement pour permettre le passage dans les tunnels d'1 bloc
            float centerY = nextY;
            
            // Pour les hitboxes fines, vérifier seulement le centre pour éviter les faux positifs
            // Cela permet de passer dans les tunnels étroits même si les coins touchent légèrement
            boolean collisionCenter = map.isSolidAt(frontX, centerY, collisionLayerName);
            
            // Collision seulement si le centre touche vraiment un mur
            // Avec une hitbox ultra-réduite, on vérifie seulement le centre pour plus de fluidité
            if (collisionCenter) {
                // Blocage détecté : aligner proprement
                int tileX = (int) (frontX / map.getTileWidth());
                if (sign > 0) {
                    nextX = tileX * map.getTileWidth() - halfWidth;
                } else {
                    nextX = (tileX + 1) * map.getTileWidth() + halfWidth;
                }
                vel.setX(0);
            }
        }
        pos.setX(nextX);
        
        // === COLLISION Y (vertical) ===
        // IMPORTANT: Ne JAMAIS bloquer le mouvement horizontal à la surface
        // Vérifier la collision Y si on monte/descend (seuil bas pour permettre le mouvement dans les tunnels)
        if (Math.abs(vel.getY()) > 0.1f) { // Seuil bas pour permettre le mouvement vertical libre dans les tunnels
            float sign = Math.signum(vel.getY());
            float leftX = nextX - halfWidth;
            float rightX = nextX + halfWidth;
            float centerX = nextX;
            
            // Sweep : vérifier la collision de manière continue pendant le mouvement
            float stepY = vel.getY() * delta;
            float steps = Math.max(1, (int) Math.ceil(Math.abs(stepY) / (map.getTileHeight() * 0.5f)));
            float stepSize = stepY / steps;
            
            float testY = pos.getY();
            
            for (int i = 0; i < steps; i++) {
                testY += stepSize;
                
                if (sign > 0 && vel.getY() > 0.1f) { // Seuil bas pour permettre la montée
                    // On monte vraiment : vérifier le haut de la hitbox + une petite marge de sécurité
                    float topY = testY + halfHeight;
                    float safetyMargin = 1f; // Marge réduite pour permettre plus de liberté
                    float topYWithMargin = topY + safetyMargin;
                    
                    // Pour les entités plus grandes, vérifier seulement le centre pour éviter les faux positifs
                    // Si on vérifie les bords, on peut bloquer dans les tunnels étroits
                    boolean collisionCenter = map.isSolidAt(centerX, topYWithMargin, collisionLayerName);
                    
                    // Collision seulement si le centre touche vraiment un plafond
                    if (collisionCenter) {
                        // Trouver la tuile qui bloque
                        int tileY = (int) (topYWithMargin / map.getTileHeight());
                        nextY = tileY * map.getTileHeight() - halfHeight - safetyMargin;
                        vel.setY(0);
                        break;
                    }
                } else if (sign < 0 && vel.getY() < -0.1f) { // Seuil bas pour permettre la descente
                    // On descend vraiment : vérifier le bas de la hitbox + une petite marge de sécurité
                    float bottomY = testY - halfHeight;
                    float safetyMargin = 1f; // Marge réduite pour permettre plus de liberté
                    float bottomYWithMargin = bottomY - safetyMargin;
                    
                    // Pour les entités plus grandes, vérifier seulement le centre pour éviter les faux positifs
                    boolean collisionCenter = map.isSolidAt(centerX, bottomYWithMargin, collisionLayerName);
                    
                    // Collision seulement si le centre touche vraiment un sol
                    if (collisionCenter) {
                        int tileY = (int) (bottomYWithMargin / map.getTileHeight());
                        nextY = (tileY + 1) * map.getTileHeight() + halfHeight + safetyMargin;
                        vel.setY(0);
                        break;
                    }
                }
            }
        }
        
        // Si vélocité Y très faible, l'annuler pour éviter les micro-mouvements qui bloquent
        // Seuil très bas pour ne pas bloquer le mouvement intentionnel
        if (vel.getY() != 0 && Math.abs(vel.getY()) <= 0.01f) {
            vel.setY(0);
        }
        pos.setY(nextY);
        
        // Clamp dans les limites de la carte
        float minX = halfWidth;
        float maxX = map.getMapWidthPixels() - halfWidth;
        float minY = halfHeight;
        float maxY = map.getMapHeightPixels() - halfHeight;
        pos.setX(Math.max(minX, Math.min(pos.getX(), maxX)));
        pos.setY(Math.max(minY, Math.min(pos.getY(), maxY)));
    }
    
    public boolean isTunnel(int tileX, int tileY) {
        if (map == null) return false;
        if (map.isSolidAt(tileX, tileY, collisionLayerName)) return false;
        
        return map.isSolidAt(tileX, tileY + 1, collisionLayerName) ||
               map.isSolidAt(tileX, tileY - 1, collisionLayerName) ||
               map.isSolidAt(tileX - 1, tileY, collisionLayerName) ||
               map.isSolidAt(tileX + 1, tileY, collisionLayerName);
    }
    
    private static final Vector2 DEFAULT_SPAWN = new Vector2(40, 40);
    
    public Vector2 findTunnelSpawn() {
        if (map == null) return DEFAULT_SPAWN;
        
        TiledMapTileLayer layer = map.getLayer(collisionLayerName);
        if (layer == null) return DEFAULT_SPAWN;
        
        int w = layer.getWidth();
        int h = layer.getHeight();
        int startY = Math.max(0, h / 3);
        float tileWidth = map.getTileWidth();
        float tileHeight = map.getTileHeight();
        float halfTileWidth = tileWidth * 0.5f;
        float halfTileHeight = tileHeight * 0.5f;
        
        for (int y = startY; y < h; y++) {
            for (int x = 0; x < w; x++) {
                if (isTunnel(x, y)) {
                    float px = x * tileWidth + halfTileWidth;
                    float py = y * tileHeight + halfTileHeight;
                    return new Vector2(px, py);
                }
            }
        }
        
        return DEFAULT_SPAWN;
    }
}
