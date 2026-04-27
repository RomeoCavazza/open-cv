package com.irina.myfirstgame;

import com.badlogic.gdx.Game;
import com.badlogic.gdx.assets.AssetManager;
import com.irina.myfirstgame.core.screens.MenuScreen;
import com.irina.myfirstgame.core.screens.PlayScreen;
import com.irina.myfirstgame.core.screens.SettingsScreen;

public class Main extends Game {
    public AssetManager assets;

    private static final String DEFAULT_PLAYER_NAME = "Wormy";
    private String cachedPlayerName = DEFAULT_PLAYER_NAME;

    @Override
    public void create() {
        setScreen(new MenuScreen(this));
    }

    @Override
    public void render() {
        super.render();
    }

    @Override
    public void dispose() {
        if (assets != null) {
        assets.dispose();
        }
        super.dispose();
    }

    public void startGame(String playerName) {
        this.cachedPlayerName = (playerName == null || playerName.trim().isEmpty())
                ? DEFAULT_PLAYER_NAME
                : playerName.trim();
        setScreen(new PlayScreen(this, cachedPlayerName));
    }

    public void openSettingsScreen(com.badlogic.gdx.Screen returnScreen) {
        setScreen(new SettingsScreen(this, returnScreen));
    }

    public void returnToMenu() {
        setScreen(new MenuScreen(this));
    }

    public String getCachedPlayerName() {
        return cachedPlayerName;
    }
}
