package com.irina.myfirstgame.core;

import java.util.HashMap;
import java.util.Map;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.TextureRegion;

import com.irina.myfirstgame.systems.Animation;

public class Assets {
    private Map<String, Texture> textures;
    private Map<String, com.badlogic.gdx.audio.Sound> sounds;

    public Assets() {
        this.textures = new HashMap<>();
        this.sounds = new HashMap<>();
    }

    public void loadAll() {
        Gdx.app.log("Assets", "Starting to load all textures...");

        // Baby Wormy
        loadTexture("worm_0.png");
        loadTexture("worm_1.png");
        loadTexture("worm_2.png");
        loadTexture("worm_3.png");
        loadTexture("worm_up_0.png");
        loadTexture("worm_up_1.png");
        loadTexture("worm_up_2.png");
        loadTexture("worm_up_3.png");
        loadTexture("worm_turn_up_1.png");
        loadTexture("worm_turn_up_2.png");

        // Adult Wormy
        loadTexture("adult_wormy_1.png");
        loadTexture("adult_wormy_2.png");
        loadTexture("adult_wormy_3.png");
        loadTexture("adult_wormy_4.png");
        loadTexture("adult_wormy_5.png");
        loadTexture("adult_wormy_up_1.png");
        loadTexture("adult_wormy_up_2.png");
        loadTexture("adult_wormy_up_3.png");
        loadTexture("adult_wormy_up_4.png");
        loadTexture("adult_wormy_up_5.png");
        loadTexture("adult_wormy_turn_up_1.png");
        loadTexture("adult_wormy_turn_up_2.png");

        // Super Wormy
        Gdx.app.log("Assets", "Loading Super Wormy textures...");
        loadTexture("super-wormy(1).png");
        loadTexture("super-wormy(2).png");
        loadTexture("super-wormy(3).png");
        loadTexture("super-wormy(4).png");
        loadTexture("super-wormy(5).png");
        loadTexture("super-wormy-up(1).png");
        loadTexture("super-wormy-up(2).png");
        loadTexture("super-wormy-up(3).png");
        loadTexture("super-wormy-up(4).png");
        loadTexture("super-wormy-up(5).png");
        loadTexture("super-wormy-turn-up(1).png");
        loadTexture("super-wormy-turn-up(2).png");

        // Ennemis
        loadTexture("bird.png");
        loadTexture("ant.png");
        loadTexture("spider.png");

        // Items
        loadTexture("burger.png");
        loadTexture("diamond.png");
        loadTexture("frites.png");
        loadTexture("soda.png");

        // Gun & Projectiles
        loadTexture("gun.png");
        loadTexture("bullet.png");
        // Essayer de charger gun-HUD.png si disponible (conversion depuis gun-HUD.avif
        // nécessaire)
        // Si gun-HUD.png n'existe pas, on utilisera gun.png comme fallback
        loadTexture("gun-HUD.png");
        // Essayer de charger gun-flashes.avif directement (LibGDX peut le gérer selon
        // la version)
        loadTexture("gun-flashes.avif");
        // Fallback: essayer gun-flashes.png si l'AVIF n'existe pas
        loadTexture("gun-flashes.png");

        // Health Sprites (0 to 7)
        for (int i = 0; i <= 7; i++) {
            loadTexture("health_" + i + ".png");
        }

        // Energy Sprites (0 to 5)
        for (int i = 0; i <= 5; i++) {
            loadTexture("energy_" + i + ".png");
        }

        Gdx.app.log("Assets", "Finished loading all textures. Total loaded: " + textures.size());
    }

    public Texture loadTexture(String name) {
        if (!textures.containsKey(name)) {
            try {
                com.badlogic.gdx.files.FileHandle fileHandle = Gdx.files.internal(name);
                if (!fileHandle.exists()) {
                    Gdx.app.log("Assets", "⚠️ Texture file does not exist: " + name);
                    return null;
                }
                Texture texture = new Texture(fileHandle);
                texture.setFilter(Texture.TextureFilter.Nearest, Texture.TextureFilter.Nearest);
                textures.put(name, texture);
                Gdx.app.log("Assets",
                        "✅ Loaded texture: " + name + " (" + texture.getWidth() + "x" + texture.getHeight() + ")");
            } catch (Exception e) {
                Gdx.app.error("Assets", "❌ ERREUR lors du chargement de la texture: " + name, e);
                Gdx.app.error("Assets", "   Cause: " + e.getMessage());
                return null;
            }
        }
        return textures.get(name);
    }

    public Texture getTexture(String name) {
        return textures.get(name);
    }

    public TextureRegion getTextureRegion(String name) {
        Texture texture = getTexture(name);
        if (texture == null) {
            Gdx.app.error("Assets", "Texture is null when getting TextureRegion for: " + name);
            return null;
        }
        return new TextureRegion(texture);
    }

    public com.badlogic.gdx.audio.Sound getSound(String name) {
        return sounds.get(name);
    }

    /**
     * Crée une animation de flash de canon à partir de la texture gun-flashes.
     * Essaie d'abord gun-flashes.avif, puis gun-flashes.png en fallback.
     * La texture contient 4 frames sur une seule ligne (1 ligne, 4 colonnes).
     * 
     * @param frameDuration Durée de chaque frame en secondes (ex: 0.05f pour 20
     *                      FPS)
     * @return Animation de flash de canon, ou null si la texture n'existe pas
     */
    public Animation getGunFlashAnimation(float frameDuration) {
        Gdx.app.log("Assets", "🔍 Début de getGunFlashAnimation() - Recherche de la texture...");

        // Essayer d'abord gun-flashes.avif (support selon version LibGDX)
        Texture texture = getTexture("gun-flashes.avif");
        String loadedFile = "gun-flashes.avif";

        if (texture == null) {
            Gdx.app.log("Assets", "⚠️ gun-flashes.avif n'a pas pu être chargé, tentative avec gun-flashes.png...");
            // Si l'AVIF n'existe pas, essayer le PNG en fallback
            texture = getTexture("gun-flashes.png");
            loadedFile = "gun-flashes.png";
        }

        if (texture == null) {
            Gdx.app.error("Assets", "❌ ERREUR CRITIQUE: Ni gun-flashes.avif ni gun-flashes.png n'ont été trouvés !");
            Gdx.app.error("Assets", "❌ Vérifiez que le fichier existe dans MyFirstGame/assets/");
            Gdx.app.error("Assets", "❌ LibGDX ne supporte généralement PAS les fichiers AVIF");
            Gdx.app.error("Assets", "❌ SOLUTION: Convertir gun-flashes.avif en gun-flashes.png");
            return null;
        }

        Gdx.app.log("Assets",
                "✅ " + loadedFile + " chargé avec succès (" + texture.getWidth() + "x" + texture.getHeight() + ")");

        // La texture contient 4 frames sur une ligne : découper en 4 régions égales
        int textureWidth = texture.getWidth();
        int textureHeight = texture.getHeight();
        int frameWidth = textureWidth / 4; // 4 colonnes
        int frameHeight = textureHeight; // 1 ligne

        // Créer les 4 régions de texture (frames)
        TextureRegion[] frames = new TextureRegion[4];
        for (int i = 0; i < 4; i++) {
            frames[i] = new TextureRegion(texture, i * frameWidth, 0, frameWidth, frameHeight);
        }

        // Créer l'animation avec le mode NORMAL (ne boucle pas automatiquement)
        Animation animation = new Animation(frameDuration, frames);
        animation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.NORMAL);

        return animation;
    }

    public void dispose() {
        for (Texture texture : textures.values()) {
            texture.dispose();
        }
        textures.clear();
        for (com.badlogic.gdx.audio.Sound sound : sounds.values()) {
            sound.dispose();
        }
        sounds.clear();
    }
}
