package com.irina.myfirstgame.core.input;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import java.util.HashSet;
import java.util.Set;

/**
 * Contrôleur d'entrée pour gérer le clavier et la souris.
 * <p>
 * Détecte les touches pressées et les clics de souris, et fournit
 * des méthodes pour vérifier l'état des entrées utilisateur.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class InputController {
    private Set<Integer> keysDown;
    private float mouseX;
    private float mouseY;
    private boolean mousePressed;
    private boolean mouseJustClicked;
    private boolean lastMousePressed;
    
    public InputController() {
        this.keysDown = new HashSet<>();
        this.lastMousePressed = false;
        this.mouseJustClicked = false;
    }
    
    public void update() {
        mouseX = Gdx.input.getX();
        mouseY = Gdx.input.getY();
        mousePressed = Gdx.input.isButtonPressed(Input.Buttons.LEFT);
        
        // Détecter un clic unique (justPressed)
        mouseJustClicked = mousePressed && !lastMousePressed;
        lastMousePressed = mousePressed;
        
        keysDown.clear();
        if (Gdx.input.isKeyPressed(Input.Keys.LEFT)) keysDown.add(Input.Keys.LEFT);
        if (Gdx.input.isKeyPressed(Input.Keys.RIGHT)) keysDown.add(Input.Keys.RIGHT);
        if (Gdx.input.isKeyPressed(Input.Keys.UP)) keysDown.add(Input.Keys.UP);
        if (Gdx.input.isKeyPressed(Input.Keys.DOWN)) keysDown.add(Input.Keys.DOWN);
    }
    
    public boolean isKeyPressed(int keyCode) {
        return Gdx.input.isKeyPressed(keyCode);
    }
    
    public boolean isDown(int keyCode) {
        return keysDown.contains(keyCode);
    }
    
    public boolean isClicked() {
        return mousePressed;
    }
    
    public boolean isJustClicked() {
        return mouseJustClicked;
    }
    
    public float getMouseX() {
        return mouseX;
    }
    
    public float getMouseY() {
        return mouseY;
    }
    
    public boolean isLeftPressed() {
        return isKeyPressed(Input.Keys.LEFT);
    }
    
    public boolean isRightPressed() {
        return isKeyPressed(Input.Keys.RIGHT);
    }
    
    public boolean isUpPressed() {
        return isKeyPressed(Input.Keys.UP);
    }
    
    public boolean isDownPressed() {
        return isKeyPressed(Input.Keys.DOWN);
    }
}

