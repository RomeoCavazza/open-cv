package com.irina.myfirstgame.world;

import com.badlogic.gdx.maps.MapLayer;
import com.badlogic.gdx.maps.MapProperties;
import com.badlogic.gdx.maps.tiled.TiledMap;
import com.badlogic.gdx.maps.tiled.TiledMapTile;
import com.badlogic.gdx.maps.tiled.TiledMapTileLayer;
import com.badlogic.gdx.maps.tiled.TmxMapLoader;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.world.valueobjects.TileCollection;

public class Map {
    private TiledMap tiledMap;
    private TileCollection tiles;
    private int tileWidth;
    private int tileHeight;
    private int mapWidth;
    private int mapHeight;
    
    public Map(String tmxFile) {
        load(tmxFile);
    }
    
    public void load(String tmxFile) {
        tiledMap = new TmxMapLoader().load(tmxFile);
        MapProperties props = tiledMap.getProperties();
        tileWidth = props.get("tilewidth", Integer.class);
        tileHeight = props.get("tileheight", Integer.class);
        mapWidth = props.get("width", Integer.class);
        mapHeight = props.get("height", Integer.class);
    }
    
    public TiledMap getTiledMap() {
        return tiledMap;
    }
    
    public TiledMapTileLayer getLayer(String layerName) {
        MapLayer layer = tiledMap.getLayers().get(layerName);
        return layer instanceof TiledMapTileLayer ? (TiledMapTileLayer) layer : null;
    }
    
    public Tile getTileAt(Vector2 position) {
        return null; // À implémenter selon les besoins
    }
    
    public boolean isSolidAt(int tileX, int tileY, String layerName) {
        TiledMapTileLayer layer = getLayer(layerName);
        if (layer == null) return false;
        if (tileX < 0 || tileY < 0 || tileX >= layer.getWidth() || tileY >= layer.getHeight()) {
            return false;
        }
        TiledMapTileLayer.Cell cell = layer.getCell(tileX, tileY);
        if (cell == null) return false;
        TiledMapTile tile = cell.getTile();
        if (tile == null) return false;
        Object solid = tile.getProperties().get("solid");
        return isSolidProperty(solid);
    }
    
    private boolean isSolidProperty(Object solid) {
        return (solid instanceof Boolean && (Boolean) solid) ||
               (solid instanceof String && Boolean.parseBoolean((String) solid));
    }
    
    public boolean isSolidAt(float worldX, float worldY, String layerName) {
        int tileX = (int) Math.floor(worldX / tileWidth);
        int tileY = (int) Math.floor(worldY / tileHeight);
        return isSolidAt(tileX, tileY, layerName);
    }

    public float calculateSurfaceLevel(String layerName) {
        TiledMapTileLayer layer = getLayer(layerName);
        if (layer == null) {
            return getMapHeightPixels() * 0.5f;
        }
        for (int y = layer.getHeight() - 1; y >= 0; y--) {
            for (int x = 0; x < layer.getWidth(); x++) {
                TiledMapTileLayer.Cell cell = layer.getCell(x, y);
                if (cell == null) {
                    continue;
                }
                TiledMapTile tile = cell.getTile();
                if (tile == null) {
                    continue;
                }
                Object solid = tile.getProperties().get("solid");
                if (isSolidProperty(solid)) {
                    return (y + 1) * tileHeight;
                }
            }
        }
        return getMapHeightPixels() * 0.5f;
    }
    
    public int getTileWidth() {
        return tileWidth;
    }
    
    public int getTileHeight() {
        return tileHeight;
    }
    
    public int getMapWidthPixels() {
        return mapWidth * tileWidth;
    }
    
    public int getMapHeightPixels() {
        return mapHeight * tileHeight;
    }
    
    public void dispose() {
        if (tiledMap != null) {
            tiledMap.dispose();
        }
    }
}

