package com.irina.myfirstgame.core.screens;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Preferences;
import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.CheckBox;
import com.badlogic.gdx.scenes.scene2d.ui.Label;
import com.badlogic.gdx.scenes.scene2d.ui.Skin;
import com.badlogic.gdx.scenes.scene2d.ui.Slider;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.ui.TextButton;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.Align;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.irina.myfirstgame.Main;
import com.irina.myfirstgame.core.ui.UiSkinFactory;

/**
 * Écran de paramètres : volume global et préférences d'affichage.
 */
public class SettingsScreen implements Screen {

    private static final String PREFS_NAME = "revolvr_settings";
    private static final String PREF_VOLUME = "masterVolume";
    private static final String PREF_OVERLAY = "overlayEnabled";

    private final Main game;
    private final Screen previousScreen;

    private Stage stage;
    private Skin skin;
    private Preferences preferences;
    private Slider volumeSlider;
    private CheckBox overlayCheckbox;
    private Label feedbackLabel;

    public SettingsScreen(Main game, Screen previousScreen) {
        this.game = game;
        this.previousScreen = previousScreen;
    }

    @Override
    public void show() {
        stage = new Stage(new FitViewport(1280f, 720f));
        Gdx.input.setInputProcessor(stage);
        preferences = Gdx.app.getPreferences(PREFS_NAME);
        skin = UiSkinFactory.createDefaultSkin();
        buildLayout();
    }

    private void buildLayout() {
        Table root = new Table();
        root.setFillParent(true);
        root.defaults().pad(12f);
        stage.addActor(root);

        Label title = new Label("Settings", skin, "title");
        title.setAlignment(Align.center);

        volumeSlider = new Slider(0f, 1f, 0.01f, false, skin);
        volumeSlider.setValue(preferences.getFloat(PREF_VOLUME, 0.7f));

        overlayCheckbox = new CheckBox(" Afficher l'overlay des tuyaux (F1)", skin);
        overlayCheckbox.setChecked(preferences.getBoolean(PREF_OVERLAY, true));

        feedbackLabel = new Label("", skin, "feedback");

        TextButton applyButton = new TextButton("Sauvegarder", skin);
        applyButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                saveSettings();
            }
        });

        TextButton backButton = new TextButton("Retour", skin);
        backButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                goBack();
            }
        });

        root.add(title).colspan(2).padBottom(20f);
        root.row();
        root.add(new Label("Volume général", skin)).left();
        root.add(volumeSlider).width(400f);
        root.row();
        root.add(new Label("Affichage", skin)).left();
        root.add(overlayCheckbox).left();
        root.row();
        root.add(feedbackLabel).colspan(2).height(30f);
        root.row();
        root.add(applyButton).width(220f);
        root.add(backButton).width(220f);
    }

    private void saveSettings() {
        preferences.putFloat(PREF_VOLUME, volumeSlider.getValue());
        preferences.putBoolean(PREF_OVERLAY, overlayCheckbox.isChecked());
        preferences.flush();
        feedbackLabel.setText("Paramètres sauvegardés.");
    }

    private void goBack() {
        if (game == null) return;
        Screen target = previousScreen != null ? previousScreen : new MenuScreen(game);
        game.setScreen(target);
    }

    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(0.04f, 0.04f, 0.06f, 1f);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);
        if (stage != null) {
            stage.act(delta);
            stage.draw();
        }
    }

    @Override
    public void resize(int width, int height) {
        if (stage != null) {
            stage.getViewport().update(width, height, true);
        }
    }

    @Override
    public void pause() {
    }

    @Override
    public void resume() {
    }

    @Override
    public void hide() {
        if (Gdx.input.getInputProcessor() == stage) {
            Gdx.input.setInputProcessor(null);
        }
    }

    @Override
    public void dispose() {
        if (stage != null) {
            stage.dispose();
        }
        if (skin != null) {
            skin.dispose();
        }
    }
}
