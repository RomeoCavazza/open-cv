package com.irina.myfirstgame.lwjgl3;

import com.badlogic.gdx.backends.lwjgl3.Lwjgl3Application;
import com.badlogic.gdx.backends.lwjgl3.Lwjgl3ApplicationConfiguration;
import com.irina.myfirstgame.Main;

/**
 * Launcher pour le jeu sur desktop (LWJGL3).
 * <p>
 * Point d'entrée pour lancer le jeu sur desktop avec le backend LWJGL3.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Lwjgl3Launcher {
    public static void main(String[] args) {
        Lwjgl3ApplicationConfiguration config = new Lwjgl3ApplicationConfiguration();
        config.setTitle("Wormy");
        config.setWindowedMode(1024, 1024);
        config.setResizable(false);
        config.useVsync(true);
        config.setForegroundFPS(60);
        new Lwjgl3Application(new Main(), config);
    }
}


