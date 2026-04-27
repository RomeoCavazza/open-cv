package com.irina.myfirstgame.core.screens;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.Label;
import com.badlogic.gdx.scenes.scene2d.ui.Skin;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.ui.TextButton;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.irina.myfirstgame.Main;
import com.irina.myfirstgame.core.ui.UiSkinFactory;

/**
 * End game screen displayed when the player wins (kills the Pie) or loses
 * (dies).
 * Provides two actions: Restart the game or Quit the application.
 */
public class GameOverScreen implements Screen {
    private final Main game;
    private final boolean isVictory;
    private final String deathCause; // How the player died (enemy name or "Starvation")
    private final String playerName; // Player's name for personalized messages
    private final com.irina.myfirstgame.systems.ScoreManager scoreManager; // Score manager with final score
    private Stage stage;
    private Skin skin;

    /**
     * @param game         The main game instance
     * @param isVictory    true if player won (killed Pie), false if player died
     * @param deathCause   How the player died (e.g., "Ant", "Spider", "Pie",
     *                     "Starvation"), null if victory
     * @param playerName   The player's name for personalized messages
     * @param scoreManager The score manager containing the final score
     */
    public GameOverScreen(Main game, boolean isVictory, String deathCause, String playerName,
            com.irina.myfirstgame.systems.ScoreManager scoreManager) {
        this.game = game;
        this.isVictory = isVictory;
        this.deathCause = deathCause;
        this.playerName = playerName != null ? playerName : "Player";
        this.scoreManager = scoreManager;
    }

    @Override
    public void show() {
        stage = new Stage(new FitViewport(1280f, 720f));
        Gdx.input.setInputProcessor(stage);
        skin = UiSkinFactory.createDefaultSkin();
        buildLayout();
    }

    private void buildLayout() {
        // Semi-transparent black background overlay (like PauseMenu)
        Table background = new Table();
        background.setFillParent(true);
        background.setBackground(skin.newDrawable("white", new com.badlogic.gdx.graphics.Color(0f, 0f, 0f, 0.7f)));
        stage.addActor(background);

        // Centered blue panel (like PauseMenu) - simplifié
        Table panel = new Table();
        panel.setBackground(skin.newDrawable("white", new com.badlogic.gdx.graphics.Color(0.12f, 0.15f, 0.22f, 0.98f)));
        panel.pad(40f).defaults().pad(8f).width(400f); // Panel plus compact

        // Different title based on victory or defeat
        String titleText = isVictory ? "Congratulations!" : "Game Over";
        Label title = new Label(titleText, skin, "title");

        // Personalized subtitle based on death cause
        String subtitleText = getPersonalizedMessage();
        Label subtitle = new Label(subtitleText, skin);

        // Score display - toujours afficher Score actuel et Best score
        int finalScore = scoreManager != null ? scoreManager.getCurrentScore() : 0;
        // Récupérer le meilleur score AVANT de sauvegarder (sinon il sera mis à jour)
        int playerHighScoreBeforeSave = scoreManager != null ? scoreManager.getPlayerHighScore(playerName) : 0;
        // Sauvegarder le score et vérifier si c'est un nouveau record
        boolean isNewRecord = scoreManager != null && scoreManager.saveScore(playerName);
        // Récupérer le meilleur score après sauvegarde (peut avoir changé si nouveau record)
        int playerHighScore = scoreManager != null ? scoreManager.getPlayerHighScore(playerName) : 0;

        // Toujours afficher le score actuel
        Label scoreLabel = new Label("Score: " + finalScore, skin);
        scoreLabel.setColor(com.badlogic.gdx.graphics.Color.WHITE);
        scoreLabel.setFontScale(1.2f);

        // Toujours afficher le meilleur score du joueur
        Label highScoreLabel;
        if (isNewRecord) {
            highScoreLabel = new Label("NEW RECORD: " + playerHighScore, skin);
            highScoreLabel.setColor(com.badlogic.gdx.graphics.Color.GOLD);
            highScoreLabel.setFontScale(1.2f);
        } else {
            highScoreLabel = new Label("Best: " + playerHighScore, skin);
            highScoreLabel.setColor(com.badlogic.gdx.graphics.Color.LIGHT_GRAY);
            highScoreLabel.setFontScale(1.1f);
        }

        TextButton restartBtn = new TextButton("Restart", skin);
        TextButton quitBtn = new TextButton("Quit", skin);

        restartBtn.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, com.badlogic.gdx.scenes.scene2d.Actor actor) {
                // Restart the game using the cached player name
                game.startGame(null);
            }
        });

        quitBtn.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, com.badlogic.gdx.scenes.scene2d.Actor actor) {
                Gdx.app.exit();
            }
        });

        panel.add(title).padBottom(15f);
        panel.row();
        panel.add(subtitle).padBottom(30f);
        panel.row();
        
        // Score actuel (toujours affiché)
        panel.add(scoreLabel).padBottom(12f);
        panel.row();
        
        // Meilleur score (toujours affiché)
        panel.add(highScoreLabel).padBottom(30f);
        panel.row();

        // Boutons avec meilleur espacement
        panel.add(restartBtn).padTop(10f).width(200f).height(45f);
        panel.row();
        panel.add(quitBtn).padTop(8f).width(200f).height(45f);

        // Centrer verticalement et horizontalement le panel dans la fenêtre
        background.add(panel).center();
    }

    /**
     * Generate a personalized message based on victory/defeat and death cause
     */
    private String getPersonalizedMessage() {
        if (isVictory) {
            return "You defeated the Pie!";
        }

        // Defeat messages - personalized by death cause
        if (deathCause == null) {
            return playerName + " was defeated";
        }

        switch (deathCause) {
            case "Ant":
                return playerName + " was devoured by an Ant!";
            case "Spider":
                return playerName + " was caught by a Spider!";
            case "Pie":
                return playerName + " was killed by the Pie!";
            case "Starvation":
                return playerName + " died of starvation...";
            default:
                return playerName + " was defeated by " + deathCause;
        }
    }

    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(0f, 0f, 0f, 1f);
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
