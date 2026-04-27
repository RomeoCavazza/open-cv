package com.irina.myfirstgame.core.screens;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Preferences;
import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.Label;
import com.badlogic.gdx.scenes.scene2d.ui.Skin;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.ui.TextButton;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.Align;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.irina.myfirstgame.Main;
import com.irina.myfirstgame.core.ui.UiSkinFactory;

/**
 * Menu principal : permet de saisir le nom du joueur, de démarrer la partie
 * ou d'ouvrir les réglages.
 */
public class MenuScreen implements Screen {

    private static final float VIRTUAL_WIDTH = 1280f;
    private static final float VIRTUAL_HEIGHT = 720f;
    private static final String PREFS_NAME = "revolvr_settings";
    private static final String PREF_PLAYER_NAME = "playerName";

    private final Main game;

    private Stage stage;
    private Skin skin;
    private TextField nameField;
    private Label errorLabel;
    private Preferences preferences;

    public MenuScreen(Main game) {
        this.game = game;
    }

    @Override
    public void show() {
        stage = new Stage(new FitViewport(VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
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

        Label title = new Label("Wormy", skin, "title");
        title.setAlignment(Align.center);

        nameField = new TextField(loadStoredName(), skin);
        nameField.setMessageText("Nom du joueur");

        errorLabel = new Label("", skin, "error");
        errorLabel.setColor(Color.SCARLET);

        TextButton startButton = new TextButton("Start", skin);
        TextButton settingsButton = new TextButton("Settings", skin);
        TextButton quitButton = new TextButton("Quit", skin);

        startButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                launchGame();
            }
        });

        settingsButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                openSettings();
            }
        });

        quitButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                Gdx.app.exit();
            }
        });

        root.add(title).colspan(2).padBottom(20f);
        root.row();
        root.add(new Label("Nom du joueur", skin)).width(300f);
        root.add(nameField).width(400f);
        root.row();
        root.add(errorLabel).colspan(2).height(24f);
        root.row();
        root.add(startButton).width(250f).colspan(2).padTop(10f);
        root.row();
        root.add(settingsButton).width(250f).colspan(2);
        root.row();
        root.add(quitButton).width(250f).colspan(2);
    }

    private void launchGame() {
        String playerName = sanitizeName(nameField.getText());
        if (playerName == null) {
            errorLabel.setText("Choisis un nom valide (3+ caractères).");
            return;
        }
        errorLabel.setText("");
        preferences.putString(PREF_PLAYER_NAME, playerName);
        preferences.flush();
        if (game != null) {
            game.startGame(playerName);
        }
    }

    private void openSettings() {
        if (game != null) {
            game.openSettingsScreen(this);
        }
    }

    private String loadStoredName() {
        String stored = preferences != null ? preferences.getString(PREF_PLAYER_NAME, "") : "";
        if (stored == null || stored.trim().isEmpty()) {
            if (game != null) {
                stored = game.getCachedPlayerName();
            }
        }
        return stored == null ? "" : stored;
    }

    private String sanitizeName(String value) {
        if (value == null) return null;
        String trimmed = value.trim();
        return trimmed.length() < 3 ? null : trimmed;
    }

    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(0.05f, 0.05f, 0.08f, 1f);
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
