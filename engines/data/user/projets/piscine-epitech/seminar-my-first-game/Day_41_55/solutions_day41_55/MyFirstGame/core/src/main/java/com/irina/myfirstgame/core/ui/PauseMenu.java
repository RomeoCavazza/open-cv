package com.irina.myfirstgame.core.ui;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.Label;
import com.badlogic.gdx.scenes.scene2d.ui.Skin;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.ui.TextButton;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.badlogic.gdx.utils.viewport.Viewport;

/**
 * Menu de pause harmonisé avec le menu principal.
 */
public class PauseMenu {
    
    public enum PauseAction {
        NONE,
        RESUME,
        RESTART,
        SETTINGS
    }
    
    private final Stage stage;
    private final Skin skin;
    private boolean inputCaptured = false;
    private PauseAction pendingAction = PauseAction.NONE;
    private boolean paused;
    
    public PauseMenu(Viewport referenceViewport) {
        float worldWidth = referenceViewport.getWorldWidth() > 0 ? referenceViewport.getWorldWidth() : 1280f;
        float worldHeight = referenceViewport.getWorldHeight() > 0 ? referenceViewport.getWorldHeight() : 720f;
        this.stage = new Stage(new FitViewport(worldWidth, worldHeight));
        this.skin = UiSkinFactory.createDefaultSkin();
        buildLayout();
    }
    
    private void buildLayout() {
        Table background = new Table();
        background.setFillParent(true);
        background.setBackground(skin.newDrawable("white", new Color(0f, 0f, 0f, 0.7f)));
        stage.addActor(background);
        
        Table panel = new Table();
        panel.setBackground(skin.newDrawable("white", new Color(0.15f, 0.18f, 0.25f, 0.95f)));
        panel.pad(30f).defaults().pad(12f).width(280f);
        
        Label title = new Label("Pause", skin, "title");
        TextButton resumeBtn = new TextButton("Resume", skin);
        TextButton restartBtn = new TextButton("Restart", skin);
        TextButton settingsBtn = new TextButton("Settings", skin);
        
        ChangeListener listener = new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (actor == resumeBtn) {
                    pendingAction = PauseAction.RESUME;
                } else if (actor == restartBtn) {
                    pendingAction = PauseAction.RESTART;
                } else if (actor == settingsBtn) {
                    pendingAction = PauseAction.SETTINGS;
                }
            }
        };
        resumeBtn.addListener(listener);
        restartBtn.addListener(listener);
        settingsBtn.addListener(listener);
        
        panel.add(title).padBottom(10f);
        panel.row();
        panel.add(resumeBtn);
        panel.row();
        panel.add(restartBtn);
        panel.row();
        panel.add(settingsBtn);
        
        background.add(panel).center();
    }
    
    /**
     * Met à jour le menu.
     */
    public PauseAction update(boolean isPaused) {
        this.paused = isPaused;
        if (isPaused) {
            attachInput();
            stage.act(Gdx.graphics.getDeltaTime());
        } else {
            detachInput();
            pendingAction = PauseAction.NONE;
        }
        PauseAction action = pendingAction;
        pendingAction = PauseAction.NONE;
        return action;
    }
    
    public void render(SpriteBatch ignored) {
        if (!paused) return;
        stage.draw();
    }
    
    public void resize(int width, int height) {
        stage.getViewport().update(width, height, true);
    }
    
    public void forceClose() {
        paused = false;
        detachInput();
        pendingAction = PauseAction.NONE;
    }
    
    private void attachInput() {
        if (!inputCaptured) {
            Gdx.input.setInputProcessor(stage);
            inputCaptured = true;
        }
    }
    
    private void detachInput() {
        if (inputCaptured) {
            Gdx.input.setInputProcessor(null);
            inputCaptured = false;
        }
    }
    
    public void dispose() {
        stage.dispose();
        skin.dispose();
    }
}

