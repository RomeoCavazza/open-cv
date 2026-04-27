package com.irina.myfirstgame.world;

import com.badlogic.gdx.graphics.OrthographicCamera;
import com.badlogic.gdx.utils.viewport.Viewport;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.valueobjects.Vector2;

/**
 * Gère la caméra qui suit le joueur.
 * <p>
 * La caméra suit automatiquement le joueur et reste dans les limites de la carte.
 * Utilise une caméra orthographique de LibGDX.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Camera {
    private OrthographicCamera libgdxCamera;
    private Viewport viewport;
    private Player target;
    private float viewportWidth;
    private float viewportHeight;
    
    public Camera(float viewportWidth, float viewportHeight) {
        this.viewportWidth = viewportWidth;
        this.viewportHeight = viewportHeight;
        this.libgdxCamera = new OrthographicCamera();
        libgdxCamera.setToOrtho(false, viewportWidth, viewportHeight);
    }
    
    public void setViewport(Viewport viewport) {
        this.viewport = viewport;
    }
    
    public void follow(Player player) {
        this.target = player;
    }
    
    public void update(float delta) {
        if (target != null) {
            Vector2 pos = target.getPosition();
            if (pos != null) {
                libgdxCamera.position.set(
                    Math.round(pos.getX()),
                    Math.round(pos.getY()),
                    0
                );
            }
        }
        libgdxCamera.update();
    }
    
    public void clampTo(Map map) {
        if (map == null) return;
        
        float halfW = viewportWidth / 2f;
        float halfH = viewportHeight / 2f;
        float maxX = map.getMapWidthPixels() - halfW;
        float maxY = map.getMapHeightPixels() - halfH;
        
        libgdxCamera.position.x = Math.max(halfW, Math.min(libgdxCamera.position.x, maxX));
        libgdxCamera.position.y = Math.max(halfH, Math.min(libgdxCamera.position.y, maxY));
        libgdxCamera.update();
    }
    
    public OrthographicCamera getLibgdxCamera() {
        return libgdxCamera;
    }
    
    public void setPosition(float x, float y) {
        libgdxCamera.position.set(x, y, 0);
        libgdxCamera.update();
    }
    
    public void updateViewport(int width, int height) {
        if (viewport != null) {
            viewport.update(width, height);
        }
    }
    
    public Viewport getViewport() {
        return viewport;
    }
}

