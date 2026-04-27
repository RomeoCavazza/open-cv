package com.irina.myfirstgame.entities;

import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.enemy.Ant;
import com.irina.myfirstgame.entities.enemy.Enemy;
import com.irina.myfirstgame.entities.enemy.Pie;
import com.irina.myfirstgame.entities.enemy.Spider;
import com.irina.myfirstgame.entities.enemy.SurfaceEnemy;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.world.Map;
import com.irina.myfirstgame.world.World;
import com.badlogic.gdx.maps.tiled.TiledMapTileLayer;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Random;

/**
 * Gère le spawn (création) des ennemis dans le monde.
 * <p>
 * Cette classe fournit des méthodes pour créer différents types d'ennemis
 * (Pie, Ant, Spider) à des positions appropriées dans le monde.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Spawner {

    private static final float BIRD_SKY_OFFSET_MIN = 200f;
    private static final float BIRD_SKY_OFFSET_MAX = 320f;

    private static final float ANT_HITBOX_SCALE = 0.6f;
    private static final float SPIDER_HITBOX_SCALE = 0.6f;

    private final Random random = new Random();

    public Enemy spawnPie(World world, Assets assets, Player target) {
        float surfaceLevel = world.getSurfaceLevel();
        float skyOffset = lerp(BIRD_SKY_OFFSET_MIN, BIRD_SKY_OFFSET_MAX, random.nextFloat());
        float spawnX = target != null && target.getPosition() != null
            ? target.getPosition().getX()
            : world.getMap().getMapWidthPixels() * 0.5f;

        Vector2 spawnPosition = new Vector2(spawnX, surfaceLevel + skyOffset);
        return new Pie(assets, spawnPosition, target, surfaceLevel);
    }

    public Enemy spawnAnt(World world, Assets assets, Player target) {
        return spawnAntAt(world, assets, target, null);
    }

    public Enemy spawnAntAt(World world, Assets assets, Player target, Vector2 spawnCell) {
        float surfaceLevel = world.getSurfaceLevel();
        Vector2 spawn = spawnCell != null ? new Vector2(spawnCell) : pickFallbackUndergroundPosition(world, target, surfaceLevel);
        return new Ant(assets, spawn, target, surfaceLevel, world.getMap(), world.getCollisionLayerName());
    }

    public Enemy spawnSpider(World world, Assets assets, Player target) {
        return spawnSpiderAt(world, assets, target, null);
    }

    public Enemy spawnSpiderAt(World world, Assets assets, Player target, Vector2 spawnCell) {
        float surfaceLevel = world.getSurfaceLevel();
        Vector2 spawn = spawnCell != null ? new Vector2(spawnCell) : pickFallbackUndergroundPosition(world, target, surfaceLevel);
        return new Spider(assets, spawn, target, surfaceLevel, world.getMap(), world.getCollisionLayerName());
    }


    private float lerp(float a, float b, float t) {
        return a + (b - a) * t;
    }

    public List<Vector2> collectDeepDirtCells(World world, float entityWidth, float entityHeight) {
        List<Vector2> cells = new ArrayList<>();
        Map map = world.getMap();
        String layerName = world.getCollisionLayerName();
        TiledMapTileLayer layer = map.getLayer(layerName);
        if (layer == null) {
            return cells;
        }
        int width = layer.getWidth();
        int height = layer.getHeight();
        int tileWidth = map.getTileWidth();
        int tileHeight = map.getTileHeight();
        float surfaceLevel = world.getSurfaceLevel();
        if (entityWidth <= 0f) {
            entityWidth = tileWidth * 0.6f;
        }
        if (entityHeight <= 0f) {
            entityHeight = tileHeight * 0.6f;
        }
        float halfW = entityWidth * 0.5f;
        float halfH = entityHeight * 0.5f;
        float mapWidthPixels = map.getMapWidthPixels();
        float mapHeightPixels = map.getMapHeightPixels();

        // surfaceLevel est le haut de l'herbe, donc le bas de l'herbe est à surfaceLevel - tileHeight
        // Ne collecter QUE les cellules bien en dessous du bas de l'herbe (dans les souterrains)
        float grassBottom = surfaceLevel - tileHeight;
        for (int y = 0; y < height; y++) {
            float centerY = (y + 0.5f) * tileHeight;
            // Exclure la surface et les tunnels proches de la surface
            // Ne garder que les cellules au moins 1 tuile en dessous du bas de l'herbe
            if (centerY >= grassBottom - tileHeight) {
                continue;
            }
            for (int x = 0; x < width; x++) {
                float centerX = (x + 0.5f) * tileWidth;
                if (centerX - halfW < 0 || centerX + halfW > mapWidthPixels ||
                    centerY - halfH < 0 || centerY + halfH > mapHeightPixels) {
                    continue;
                }
                if (!map.isSolidAt(centerX, centerY, layerName)
                        && isAreaFree(map, layerName, centerX, centerY, halfW, halfH)
                        && touchesSolid(map, layerName, centerX, centerY, tileWidth, tileHeight)) {
                    cells.add(new Vector2(centerX, centerY));
                }
            }
        }
        Collections.shuffle(cells, random);
        return cells;
    }

    public Vector2 takeRandomDeepCell(List<Vector2> cells) {
        if (cells == null || cells.isEmpty()) {
            return null;
        }
        return cells.remove(cells.size() - 1);
    }

    private Vector2 pickFallbackUndergroundPosition(World world, Player target, float surfaceLevel) {
        Map map = world.getMap();
        float fallbackX = target != null && target.getPosition() != null
            ? target.getPosition().getX()
            : map.getMapWidthPixels() * 0.5f;
        // Spawner dans les souterrains, bien en dessous de la surface
        // surfaceLevel est le haut de l'herbe, donc le bas de l'herbe est à surfaceLevel - tileHeight
        // Spawner au moins 2 tuiles en dessous du bas de l'herbe pour être sûr d'être dans les souterrains
        float tileHeight = map.getTileHeight();
        float grassBottom = surfaceLevel - tileHeight;
        float undergroundY = grassBottom - (tileHeight * 2f);
        return new Vector2(fallbackX, undergroundY);
    }

    private boolean isAreaFree(Map map, String layer, float centerX, float centerY, float halfWidth, float halfHeight) {
        return !map.isSolidAt(centerX - halfWidth, centerY - halfHeight, layer)
            && !map.isSolidAt(centerX + halfWidth, centerY - halfHeight, layer)
            && !map.isSolidAt(centerX - halfWidth, centerY + halfHeight, layer)
            && !map.isSolidAt(centerX + halfWidth, centerY + halfHeight, layer)
            && !map.isSolidAt(centerX, centerY, layer);
    }

    private boolean touchesSolid(Map map, String layer, float centerX, float centerY, float tileWidth, float tileHeight) {
        float halfTileWidth = tileWidth * 0.5f;
        float halfTileHeight = tileHeight * 0.5f;
        return map.isSolidAt(centerX - halfTileWidth, centerY, layer)
            || map.isSolidAt(centerX + halfTileWidth, centerY, layer)
            || map.isSolidAt(centerX, centerY - halfTileHeight, layer)
            || map.isSolidAt(centerX, centerY + halfTileHeight, layer);
    }

    public float computeAntHitboxWidth(Assets assets) {
        return computeHitboxWidth(assets, "ant.png", ANT_HITBOX_SCALE);
    }

    public float computeAntHitboxHeight(Assets assets) {
        return computeHitboxHeight(assets, "ant.png", ANT_HITBOX_SCALE);
    }

    public float computeSpiderHitboxWidth(Assets assets) {
        return computeHitboxWidth(assets, "spider.png", SPIDER_HITBOX_SCALE);
    }

    public float computeSpiderHitboxHeight(Assets assets) {
        return computeHitboxHeight(assets, "spider.png", SPIDER_HITBOX_SCALE);
    }
    
    private float computeHitboxWidth(Assets assets, String textureName, float scale) {
        return (assets.getTexture(textureName).getWidth() / (float) SurfaceEnemy.SHEET_COLUMNS) * scale;
    }
    
    private float computeHitboxHeight(Assets assets, String textureName, float scale) {
        return (assets.getTexture(textureName).getHeight() / (float) SurfaceEnemy.SHEET_ROWS) * scale;
    }
}

