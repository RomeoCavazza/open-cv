package com.irina.myfirstgame.systems;

import com.badlogic.gdx.graphics.g2d.TextureRegion;

/**
 * Gère l'animation de sprites à partir d'une séquence de TextureRegion.
 * <p>
 * Encapsule l'animation de LibGDX et fournit des méthodes pour contrôler
 * la lecture, la pause et la réinitialisation de l'animation.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Animation {
    private com.badlogic.gdx.graphics.g2d.Animation<TextureRegion> libgdxAnimation;
    private float stateTime;
    private boolean playing;
    
    public Animation(float frameDuration, TextureRegion[] frames) {
        this.libgdxAnimation = new com.badlogic.gdx.graphics.g2d.Animation<>(frameDuration, frames);
        this.stateTime = 0f;
        this.playing = true;
    }
    
    public void setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode mode) {
        if (libgdxAnimation != null) {
            libgdxAnimation.setPlayMode(mode);
        }
    }
    
    public void play() {
        playing = true;
    }
    
    public void pause() {
        playing = false;
    }
    
    public void reset() {
        stateTime = 0f;
    }
    
    public TextureRegion getKeyFrame(float deltaTime) {
        if (playing) {
            stateTime += deltaTime;
        }
        return libgdxAnimation != null ? libgdxAnimation.getKeyFrame(stateTime) : null;
    }
    
    public boolean isAnimationFinished(float deltaTime) {
        return libgdxAnimation != null && libgdxAnimation.isAnimationFinished(stateTime);
    }
    
    public float getStateTime() {
        return stateTime;
    }
    
    public void setStateTime(float time) {
        this.stateTime = time;
    }
}

