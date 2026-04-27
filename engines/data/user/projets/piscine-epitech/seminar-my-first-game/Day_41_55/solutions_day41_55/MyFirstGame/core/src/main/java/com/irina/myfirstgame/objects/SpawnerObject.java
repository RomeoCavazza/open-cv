package com.irina.myfirstgame.objects;

import java.util.List;
import java.util.Random;

import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.world.World;

/**
 * Gère le spawn (création) des objets dans le monde.
 * <p>
 * Cette classe fournit des méthodes pour créer différents types d'items
 * (Burger, Frites, Soda, Diamond) à des positions appropriées dans le monde.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class SpawnerObject {

    private final Random random = new Random();

    public Item spawnBurgerInDeepTunnel(World world, Assets assets) {
        float w = 16f;
        float h = 16f;

        List<com.irina.myfirstgame.entities.valueobjects.Vector2> cells = world.getSpawner().collectDeepDirtCells(world,
                w, h);

        com.irina.myfirstgame.entities.valueobjects.Vector2 cell = world.getSpawner().takeRandomDeepCell(cells);

        if (cell == null)
            return null;

        Vector2 spawnPos = new Vector2(cell.getX(), cell.getY());

        return new Burger(assets, spawnPos);
    }

    public Item spawnFritesInDeepTunnel(World world, Assets assets) {
        float w = 16f;
        float h = 16f;

        List<com.irina.myfirstgame.entities.valueobjects.Vector2> cells = world.getSpawner().collectDeepDirtCells(world,
                w, h);

        com.irina.myfirstgame.entities.valueobjects.Vector2 cell = world.getSpawner().takeRandomDeepCell(cells);

        if (cell == null)
            return null;

        Vector2 spawnPos = new Vector2(cell.getX(), cell.getY());

        return new Frites(assets, spawnPos);
    }
    
    public Item spawnSodaInDeepTunnel(World world, Assets assets) {
        float w = 16f;
        float h = 16f;

        List<com.irina.myfirstgame.entities.valueobjects.Vector2> cells = world.getSpawner().collectDeepDirtCells(world,
                w, h);

        com.irina.myfirstgame.entities.valueobjects.Vector2 cell = world.getSpawner().takeRandomDeepCell(cells);

        if (cell == null)
            return null;

        Vector2 spawnPos = new Vector2(cell.getX(), cell.getY());

        return new Soda(assets, spawnPos);
    }

    public Item spawnDiamondAtSurface(World world, Assets assets) {
        float h = 16f;
        float surfaceLevel = world.getSurfaceLevel();
        float tileHeight = world.getMap().getTileHeight();
        
        // Spawner le diamant à la surface (sur l'herbe)
        // Utiliser une position X aléatoire sur la largeur de la map
        float mapWidth = world.getMap().getMapWidthPixels();
        float spawnX = random.nextFloat() * mapWidth;
        
        // Trouver la vraie hauteur de la surface à cette position X
        // Chercher la première tuile solide à partir du haut à cette position X
        float actualSurfaceY = findSurfaceHeightAt(world, spawnX, surfaceLevel);
        
        // surfaceLevel est le haut de l'herbe
        // Pour coller le diamant à l'herbe : le bas du diamant doit toucher le haut de l'herbe
        // Item.spawn() centre le sprite (position - width/2, position - height/2),
        // donc si je veux que le bas soit à actualSurfaceY, je dois placer le centre à actualSurfaceY + h * 0.5f
        // Mais on le place directement sur la surface pour qu'il soit bien collé
        float spawnY = actualSurfaceY + h * 0.5f;
        
        Vector2 spawnPos = new Vector2(spawnX, spawnY);
        
        return new Diamond(assets, spawnPos);
    }
    
    private float findSurfaceHeightAt(World world, float x, float defaultSurfaceLevel) {
        com.irina.myfirstgame.world.Map map = world.getMap();
        String layerName = world.getCollisionLayerName();
        if (layerName == null) {
            return defaultSurfaceLevel;
        }
        
        float tileHeight = map.getTileHeight();
        float tileWidth = map.getTileWidth();
        int tileX = (int)(x / tileWidth);
        
        com.badlogic.gdx.maps.tiled.TiledMapTileLayer layer = map.getLayer(layerName);
        if (layer == null) {
            return defaultSurfaceLevel;
        }
        
        // Chercher la première tuile solide en partant du haut à cette position X
        for (int y = layer.getHeight() - 1; y >= 0; y--) {
            com.badlogic.gdx.maps.tiled.TiledMapTileLayer.Cell cell = layer.getCell(tileX, y);
            if (cell == null) {
                continue;
            }
            com.badlogic.gdx.maps.tiled.TiledMapTile tile = cell.getTile();
            if (tile == null) {
                continue;
            }
            Object solid = tile.getProperties().get("solid");
            if (solid != null && (solid instanceof Boolean && (Boolean)solid || "true".equals(solid.toString()))) {
                // Trouvé une tuile solide, retourner le haut de cette tuile
                return (y + 1) * tileHeight;
            }
        }
        
        // Si aucune tuile solide trouvée, utiliser le niveau de surface par défaut
        return defaultSurfaceLevel;
    }
}