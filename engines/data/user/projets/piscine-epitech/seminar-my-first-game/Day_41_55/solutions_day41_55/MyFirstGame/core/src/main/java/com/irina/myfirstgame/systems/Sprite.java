package com.irina.myfirstgame.systems;

import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.irina.myfirstgame.entities.valueobjects.Vector2;

/**
 * Sprite personnalisé avec support pour position, rotation, scale et flip.
 * <p>
 * Encapsule un TextureRegion de LibGDX et ajoute des fonctionnalités
 * de transformation (position, rotation, scale, flip) avec mise en cache
 * pour optimiser les performances.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Sprite {
    private TextureRegion textureRegion;
    private Vector2 position;
    private Vector2 origin;
    private float width;
    private float height;
    private boolean flipX;
    private boolean flipY;
    private float scaleX;
    private float scaleY;
    private float rotation; // Rotation en degrés
    private TextureRegion cachedFlippedRegion;
    private boolean cachedFlipX;
    private boolean cachedFlipY;
    
    public Sprite(TextureRegion textureRegion) {
        this.textureRegion = textureRegion;
        this.position = new Vector2();
        this.origin = new Vector2();
        this.width = textureRegion != null ? textureRegion.getRegionWidth() : 0;
        this.height = textureRegion != null ? textureRegion.getRegionHeight() : 0;
        this.scaleX = 1f;
        this.scaleY = 1f;
        this.rotation = 0f;
        setOriginCenter();
    }
    
    public void setOriginCenter() {
        if (textureRegion != null) {
            origin.set(width / 2f, height / 2f);
        }
    }
    
    public void setPosition(float x, float y) {
        position.set(x, y);
    }
    
    public void setPosition(Vector2 pos) {
        position.set(pos);
    }
    
    public Vector2 getPosition() {
        return position;
    }
    
    public void setRegion(TextureRegion region) {
        this.textureRegion = region;
        if (region != null) {
            this.width = region.getRegionWidth();
            this.height = region.getRegionHeight();
            // IMPORTANT: Mettre à jour l'origine pour qu'elle reste au centre
            // Sinon le sprite se décale quand on change de frame d'animation
            setOriginCenter();
            // Invalider le cache de flip car la texture a changé
            cachedFlippedRegion = null;
        }
    }
    
    public void flip(boolean x, boolean y) {
        if (x) flipX = !flipX;
        if (y) flipY = !flipY;
        // Invalider le cache si le flip change
        if (x || y) {
            cachedFlippedRegion = null;
        }
    }
    
    public boolean isFlipX() {
        return flipX;
    }
    
    public boolean isFlipY() {
        return flipY;
    }
    
    public void setScale(float scaleX, float scaleY) {
        this.scaleX = scaleX;
        this.scaleY = scaleY;
    }
    
    public void setRotation(float degrees) {
        this.rotation = degrees;
    }
    
    public float getRotation() {
        return rotation;
    }

    public float getOriginX() {
        return origin.getX();
    }

    public float getOriginY() {
        return origin.getY();
    }

    public float getScaleX() {
        return scaleX;
    }

    public float getScaleY() {
        return scaleY;
    }
    
    public float getWidth() {
        return width * scaleX;
    }
    
    public float getHeight() {
        return height * scaleY;
    }
    
    public void draw(SpriteBatch batch) {
        if (textureRegion != null && batch != null) {
            // CRITIQUE : Dans libGDX, l'origine est en coordonnées de la texture (non scalée)
            // Le scale est appliqué par libGDX autour de l'origine
            // Donc on calcule drawX/Y comme position - origine (sans multiplier par scale)
            float drawX = position.getX() - origin.getX();
            float drawY = position.getY() - origin.getY();
            
            // Réutiliser le TextureRegion mis en cache si le flip n'a pas changé
            if (flipX || flipY) {
                // Vérifier si le cache est valide
                if (cachedFlippedRegion == null || cachedFlipX != flipX || cachedFlipY != flipY) {
                    cachedFlippedRegion = new TextureRegion(textureRegion);
                    if (flipX) cachedFlippedRegion.flip(true, false);
                    if (flipY) cachedFlippedRegion.flip(false, true);
                    cachedFlipX = flipX;
                    cachedFlipY = flipY;
                }
                batch.draw(cachedFlippedRegion, 
                    drawX, drawY,
                    origin.getX(), origin.getY(),
                    width, height,
                    scaleX, scaleY, rotation);
            } else {
                // Pas de flip, utiliser directement le textureRegion original
                batch.draw(textureRegion, 
                    drawX, drawY,
                    origin.getX(), origin.getY(),
                    width, height,
                    scaleX, scaleY, rotation);
                // Invalider le cache si on n'a plus besoin de flip
                cachedFlippedRegion = null;
            }
        }
    }
}

