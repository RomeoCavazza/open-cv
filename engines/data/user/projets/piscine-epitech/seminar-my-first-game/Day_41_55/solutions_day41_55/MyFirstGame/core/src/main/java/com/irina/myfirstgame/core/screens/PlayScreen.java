package com.irina.myfirstgame.core.screens;

import java.lang.reflect.Method;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.Preferences;
import com.badlogic.gdx.Screen;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.maps.MapLayers;
import com.badlogic.gdx.maps.tiled.TiledMap;
import com.badlogic.gdx.maps.tiled.TiledMapTileLayer;
import com.badlogic.gdx.maps.tiled.renderers.OrthogonalTiledMapRenderer;
import com.badlogic.gdx.math.Vector3;
import com.badlogic.gdx.utils.viewport.FitViewport;
import com.irina.myfirstgame.Main;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.core.input.InputController;
import com.irina.myfirstgame.core.ui.HUD;
import com.irina.myfirstgame.core.ui.PauseMenu;
import com.irina.myfirstgame.core.screens.GameOverScreen;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.entities.wormy.Adult;
import com.irina.myfirstgame.entities.wormy.Baby;
import com.irina.myfirstgame.entities.wormy.Super;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.systems.Animation;
import com.irina.myfirstgame.systems.CollisionHandler;
import com.irina.myfirstgame.systems.Sprite;
import com.irina.myfirstgame.systems.SpawnManager;
import com.irina.myfirstgame.world.World;

public class PlayScreen implements Screen {

    private static final String TMX_FILE = "mapVeryNew.tmx";
    private static final String COLLISION_LAYER = "Calque de Tuiles 2";
    private static final String PIPE_LAYER = "Calque de Tuiles 3";
    private static final String PREFS_NAME = "revolvr_settings";
    private static final String PREF_OVERLAY = "overlayEnabled";
    private static final String PREF_PLAYER_NAME = "playerName";

    private static final int V_WIDTH = 1024;
    private static final int V_HEIGHT = 1024;
    private static final float FPS_H = 8f;
    private static final float FPS_UP = 8f;
    private static final float FPS_TURN = 12f;

    private final Main game;
    private final String requestedPlayerName;
    private String activePlayerName;
    private Preferences preferences;

    private SpriteBatch batch;
    private World world;
    private boolean pieKilled = false;
    private String deathCause = null; // Track how the player died
    private Player player;
    private InputController inputController;
    private CollisionHandler collisionHandler;
    private Assets assets;

    private OrthogonalTiledMapRenderer mapRenderer;
    private TiledMap tiledMap;
    private TiledMapTileLayer pipeLayer;

    private boolean overlayEnabled = true;
    private float wobbleTime;

    // Menu de pause
    private boolean isPaused = false;
    private PauseMenu pauseMenu;
    private boolean spaceWasPressed = false;

    // HUD
    private HUD hud;

    // Score system
    private com.irina.myfirstgame.systems.ScoreManager scoreManager;
    private float survivalTimer = 0f; // Track seconds survived for +1 point/second

    // Diamant spawné
    private boolean diamondSpawned = false;

    private Class<?> lastWormClass;

    // Animation de flash de canon
    private Animation gunFlashAnimation;
    private boolean gunFlashPlaying;
    private float gunFlashAngle;
    private com.irina.myfirstgame.entities.valueobjects.Vector2 gunFlashPosition;

    public PlayScreen() {
        this(null, null);
    }

    public PlayScreen(Main game, String playerName) {
        this.game = game;
        this.requestedPlayerName = sanitizeName(playerName);
    }

    @Override
    public void show() {
        batch = new SpriteBatch();
        assets = new Assets();
        assets.loadAll();
        preferences = Gdx.app.getPreferences(PREFS_NAME);
        overlayEnabled = preferences.getBoolean(PREF_OVERLAY, true);

        // Initialiser l'animation de flash de canon (10 FPS = 0.1s par frame pour être
        // plus visible)
        // 4 frames * 0.1s = 0.4s d'animation totale (au lieu de 0.2s avec 0.05s/frame)
        Gdx.app.log("PlayScreen", "🔍 show() - Initialisation de gunFlashAnimation...");
        gunFlashAnimation = assets.getGunFlashAnimation(0.1f);
        if (gunFlashAnimation == null) {
            Gdx.app.error("PlayScreen", "❌ ERREUR CRITIQUE: gunFlashAnimation est NULL !");
            Gdx.app.error("PlayScreen", "❌ La texture gun-flashes.avif/.png n'a pas pu être chargée.");
            Gdx.app.error("PlayScreen", "❌ ACTION REQUISE: Convertir gun-flashes.avif en gun-flashes.png");
            Gdx.app.error("PlayScreen", "❌ LibGDX ne supporte généralement PAS les fichiers AVIF nativement");
        } else {
            Gdx.app.log("PlayScreen", "✅ gunFlashAnimation initialisée avec succès !");
        }
        gunFlashPlaying = false;
        gunFlashPosition = new com.irina.myfirstgame.entities.valueobjects.Vector2(0, 0);
        gunFlashAngle = 0f;

        initializeGame();
    }

    private void initializeGame() {
        world = new World(TMX_FILE, V_WIDTH, V_HEIGHT);
        world.setCollisionLayer(COLLISION_LAYER);
        collisionHandler = new CollisionHandler(world.getMap(), COLLISION_LAYER);

        inputController = new InputController();
        player = new Player();

        // Réinitialiser le flag du diamant
        diamondSpawned = false;
        pieKilled = false; // Reset pieKilled on game restart
        deathCause = null; // Reset death cause
        survivalTimer = 0f; // Reset survival timer
        lastWormClass = null; // Reset last worm class for evolution detection

        // Initialize score manager
        scoreManager = new com.irina.myfirstgame.systems.ScoreManager();

        player.setInputController(inputController);
        activePlayerName = resolvePlayerName();
        player.setName(activePlayerName);

        world.bindPlayer(player);
        // Passer le scoreManager à World pour qu'il puisse ajouter des points
        world.setScoreManager(scoreManager);

        setupBabyAnimations();
        placePlayerInTunnel();
        
        // Initialiser lastWormClass avec le type de worm initial (Baby)
        if (player != null && player.getWorm().getCurrent() != null) {
            lastWormClass = player.getWorm().getCurrent().getClass();
        }

        // Initialiser et utiliser le SpawnManager pour le spawn dynamique
        com.irina.myfirstgame.systems.SpawnManager spawnManager = new com.irina.myfirstgame.systems.SpawnManager();
        world.setSpawnManager(spawnManager);
        // Register callback for Pie death
        world.setPieKilledCallback(() -> pieKilled = true);
        // Register callback for death cause tracking
        world.setDeathCauseCallback(cause -> deathCause = cause);
        spawnManager.initialize(world, assets, player);

        world.getCamera().follow(player);
        world.getCamera().setViewport(new FitViewport(V_WIDTH, V_HEIGHT, world.getCamera().getLibgdxCamera()));
        pauseMenu = new PauseMenu(world.getCamera().getViewport());

        // Initialiser le HUD
        hud = new HUD(assets, batch);
        if (player.getWorm().getCurrent() != null) {
            hud.bind(player.getWorm().getCurrent(), null);
        }
        // Réinitialiser les messages d'aide au début de la partie
        if (hud != null) {
            hud.resetHelpMessages();
        }

        tiledMap = extractTiledMap(world.getMap());
        if (tiledMap != null) {
            if (mapRenderer != null) {
                mapRenderer.dispose();
            }
            mapRenderer = new OrthogonalTiledMapRenderer(tiledMap, 1f, batch);
            pipeLayer = getTileLayerByName(tiledMap, PIPE_LAYER);
            if (pipeLayer == null) {
                Gdx.app.log("PlayScreen", "Calque introuvable: \"" + PIPE_LAYER + "\". L'overlay sera ignoré.");
            }
        } else {
            Gdx.app.log("PlayScreen", "Aucun TiledMap accessible. Fallback sur world.render().");
        }

        wobbleTime = 0f;
        isPaused = false;
    }

    private TiledMap extractTiledMap(Object mapObj) {
        if (mapObj == null) {
            return null;
        }
        if (mapObj instanceof TiledMap) {
            return (TiledMap) mapObj;
        }
        try {
            Method m = mapObj.getClass().getMethod("getTiledMap");
            Object tm = m.invoke(mapObj);
            if (tm instanceof TiledMap) {
                return (TiledMap) tm;
            }
        } catch (Exception e) {
            Gdx.app.error("PlayScreen", "Erreur lors de l'extraction du TiledMap", e);
        }
        return null;
    }

    private TiledMapTileLayer getTileLayerByName(TiledMap map, String name) {
        MapLayers layers = map.getLayers();
        for (int i = 0; i < layers.size(); i++) {
            if (layers.get(i) instanceof TiledMapTileLayer && name.equals(layers.get(i).getName())) {
                return (TiledMapTileLayer) layers.get(i);
            }
        }
        return null;
    }

    private void setupBabyAnimations() {
        Baby baby = (Baby) player.getWorm().getCurrent();
        if (baby == null) {
            return;
        }

        TextureRegion[] walkH = new TextureRegion[4];
        for (int i = 0; i < 4; i++) {
            walkH[i] = assets.getTextureRegion("worm_" + i + ".png");
        }
        Animation walkAnimation = new Animation(1f / FPS_H, walkH);
        walkAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        baby.setWalkAnimation(walkAnimation);

        TextureRegion[] walkUp = new TextureRegion[4];
        for (int i = 0; i < 4; i++) {
            walkUp[i] = assets.getTextureRegion("worm_up_" + i + ".png");
        }
        Animation upAnimation = new Animation(1f / FPS_UP, walkUp);
        upAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        baby.setUpAnimation(upAnimation);

        TextureRegion[] turnUp = new TextureRegion[] {
                assets.getTextureRegion("worm_turn_up_1.png"),
                assets.getTextureRegion("worm_turn_up_2.png")
        };
        Animation turnAnimation = new Animation(1f / FPS_TURN, turnUp);
        turnAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.NORMAL);
        baby.setTurnAnimation(turnAnimation);

        Sprite sprite = new Sprite(walkH[0]);
        sprite.setOriginCenter();
        baby.setSprite(sprite);

        if (walkH[0] != null) {
            float spriteWidth = walkH[0].getRegionWidth();
            float spriteHeight = walkH[0].getRegionHeight();
            player.setWidth(spriteWidth);
            player.setHeight(spriteHeight * 1.25f);
        }

    }

    private void setupAdultAnimations(Adult adult) {
        if (adult == null) {
            return;
        }

        TextureRegion[] walkH = new TextureRegion[5];
        for (int i = 0; i < 5; i++) {
            walkH[i] = assets.getTextureRegion("adult_wormy_" + (i + 1) + ".png");
        }
        Animation walkAnimation = new Animation(1f / FPS_H, walkH);
        walkAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        adult.setWalkAnimation(walkAnimation);

        TextureRegion[] walkUp = new TextureRegion[5];
        for (int i = 0; i < 5; i++) {
            walkUp[i] = assets.getTextureRegion("adult_wormy_up_" + (i + 1) + ".png");
        }
        Animation upAnimation = new Animation(1f / FPS_UP, walkUp);
        upAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        adult.setUpAnimation(upAnimation);

        TextureRegion[] turnUp = new TextureRegion[] {
                assets.getTextureRegion("adult_wormy_turn_up_1.png"),
                assets.getTextureRegion("adult_wormy_turn_up_2.png")
        };
        Animation turnAnimation = new Animation(1f / FPS_TURN, turnUp);
        turnAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.NORMAL);
        adult.setTurnAnimation(turnAnimation);

        Sprite sprite = new Sprite(walkH[0]);
        sprite.setOriginCenter();
        adult.setSprite(sprite);

        if (walkH[0] != null) {
            float spriteWidth = walkH[0].getRegionWidth();
            float spriteHeight = walkH[0].getRegionHeight();

            // Utiliser une hitbox aussi fine que Baby pour passer dans les tunnels d'1 bloc
            // Baby utilise les dimensions exactes du sprite : spriteWidth et spriteHeight *
            // 1.25f
            // Pour Adult, réduire significativement la largeur pour passer dans les tunnels
            // étroits
            // Utiliser la largeur de Baby (32px typiquement) pour garantir le passage dans
            // les tunnels d'1 bloc
            float adultWidth = spriteWidth * 0.7f; // 70% de la largeur pour passer dans les tunnels d'1 bloc (plus fin
                                                   // que Baby)
            float adultHeight = spriteHeight * 1.1f; // 110% de la hauteur (proche de Baby qui utilise 125%)

            // S'assurer que la largeur ne dépasse pas 32 pixels (largeur typique d'un
            // tunnel)
            if (adultWidth > 32f) {
                adultWidth = 32f * 0.9f; // 90% de 32px pour passer facilement dans les tunnels d'1 bloc
            }

            player.setWidth(adultWidth);
            player.setHeight(adultHeight);

            Gdx.app.debug("PlayScreen", "Adult hitbox ultra-réduite: width=" + adultWidth + ", height=" + adultHeight
                    + " (sprite: " + spriteWidth + "x" + spriteHeight + ")");
        }
    }

    private void setupSuperAnimations(Super superWorm) {
        if (superWorm == null) {
            return;
        }

        TextureRegion[] walkH = new TextureRegion[5];
        for (int i = 0; i < 5; i++) {
            walkH[i] = assets.getTextureRegion("super-wormy(" + (i + 1) + ").png");
        }
        Animation walkAnimation = new Animation(1f / FPS_H, walkH);
        walkAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        superWorm.setWalkAnimation(walkAnimation);

        TextureRegion[] walkUp = new TextureRegion[5];
        for (int i = 0; i < 5; i++) {
            walkUp[i] = assets.getTextureRegion("super-wormy-up(" + (i + 1) + ").png");
        }
        Animation upAnimation = new Animation(1f / FPS_UP, walkUp);
        upAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.LOOP);
        superWorm.setUpAnimation(upAnimation);

        TextureRegion[] turnUp = new TextureRegion[] {
                assets.getTextureRegion("super-wormy-turn-up(1).png"),
                assets.getTextureRegion("super-wormy-turn-up(2).png")
        };
        Animation turnAnimation = new Animation(1f / FPS_TURN, turnUp);
        turnAnimation.setPlayMode(com.badlogic.gdx.graphics.g2d.Animation.PlayMode.NORMAL);
        superWorm.setTurnAnimation(turnAnimation);

        Sprite sprite = new Sprite(walkH[0]);
        sprite.setOriginCenter();
        superWorm.setSprite(sprite);

        if (walkH[0] != null) {
            // Utiliser les MÊMES dimensions qu'Adult pour Super afin d'avoir le même
            // comportement
            // Récupérer les dimensions d'Adult depuis le Player (déjà configurées)
            float adultWidth = player.getWidth();
            float adultHeight = player.getHeight();

            // Si les dimensions du Player ne sont pas encore définies (première fois),
            // utiliser les dimensions des textures de Super
            if (adultWidth <= 0 || adultHeight <= 0) {
                float spriteWidth = walkH[0].getRegionWidth();
                float spriteHeight = walkH[0].getRegionHeight();
                player.setWidth(spriteWidth);
                player.setHeight(spriteHeight * 1.25f);
                adultWidth = spriteWidth;
                adultHeight = spriteHeight * 1.25f;
            }

            // FORCER Super à utiliser les mêmes dimensions qu'Adult
            player.setWidth(adultWidth);
            player.setHeight(adultHeight);

            // Synchroniser aussi les dimensions du worm pour les collisions
            if (superWorm != null) {
                superWorm.setWidth(adultWidth);
                superWorm.setHeight(adultHeight);
            }
        }
    }

    private void placePlayerInTunnel() {
        com.irina.myfirstgame.entities.valueobjects.Vector2 spawnPos = collisionHandler.findTunnelSpawn();
        player.setPosition(spawnPos.getX(), spawnPos.getY());

        Baby baby = (Baby) player.getWorm().getCurrent();
        if (baby != null && baby.getSprite() != null) {
            baby.getSprite().setPosition(spawnPos.getX(), spawnPos.getY());
        }

        world.getCamera().setPosition(
                spawnPos.getX() + player.getWidth() / 2f,
                spawnPos.getY() + player.getHeight() / 2f);
    }

    @Override
    public void render(float delta) {
        if (pauseMenu == null) {
            if (world != null && world.getCamera() != null && world.getCamera().getViewport() != null) {
                pauseMenu = new PauseMenu(world.getCamera().getViewport());
            }
        }

        boolean spacePressed = Gdx.input.isKeyPressed(Input.Keys.SPACE);
        if (spacePressed && !spaceWasPressed) {
            isPaused = !isPaused;
        }
        spaceWasPressed = spacePressed;

        if (isPaused && pauseMenu != null) {
            PauseMenu.PauseAction action = pauseMenu.update(true);
            handlePauseAction(action);
        } else {
            if (pauseMenu != null) {
                pauseMenu.update(false);
            }
            update(delta);
        }

        Gdx.gl.glClearColor(0, 0, 0, 1);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        if (mapRenderer != null && tiledMap != null) {
            mapRenderer.setView(world.getCamera().getLibgdxCamera());

            if (pipeLayer != null) {
                pipeLayer.setVisible(false);
            }
            mapRenderer.render();

            renderEntities();

            if (pipeLayer != null && overlayEnabled) {
                pipeLayer.setVisible(true);
                int idx = tiledMap.getLayers().getIndex(PIPE_LAYER);
                if (idx >= 0) {
                    mapRenderer.render(new int[] { idx });
                }
            }

        } else {
            world.render();
            renderEntities();
        }

        // Rendre le HUD (items collectés) - toujours à l'écran, indépendant de la
        // caméra du monde
        if (batch != null && hud != null && player != null && player.getWorm().getCurrent() != null) {
            hud.bind(player.getWorm().getCurrent(), null);
            // Update HUD with current score
            if (scoreManager != null) {
                hud.updateScore(scoreManager.getCurrentScore());
            }
            hud.render();
        }

        if (isPaused && pauseMenu != null && batch != null) {
            pauseMenu.render(batch);
        }
    }

    private void handlePauseAction(PauseMenu.PauseAction action) {
        switch (action) {
            case RESUME:
                isPaused = false;
                if (pauseMenu != null) {
                    pauseMenu.forceClose();
                }
                break;
            case RESTART:
                restartGame();
                break;
            case SETTINGS:
                if (game != null) {
                    isPaused = false;
                    if (pauseMenu != null) {
                        pauseMenu.forceClose();
                    }
                    game.openSettingsScreen(this);
                }
                break;
            case NONE:
                break;
        }
    }

    private void restartGame() {
        if (pauseMenu != null) {
            pauseMenu.forceClose();
        }
        if (world != null) {
            world.dispose();
        }
        isPaused = false;
        initializeGame();
    }

    private void renderEntities() {
        batch.setProjectionMatrix(world.getCamera().getLibgdxCamera().combined);
        batch.begin();

        Wormy worm = player.getWorm().getCurrent();
        if (worm != null && worm.getSprite() != null) {
            worm.getSprite().draw(batch);
        }

        // Afficher le pistolet si Super en possède un
        if (worm instanceof Super) {
            Super superWorm = (Super) worm;
            if (superWorm.hasGun() && superWorm.getGunSprite() != null) {
                superWorm.getGunSprite().draw(batch);
            }
        }

        // Afficher l'animation de flash de canon si elle est en cours
        if (gunFlashPlaying && gunFlashAnimation != null) {
            // Récupérer le frame actuel sans incrémenter le stateTime (deltaTime=0)
            // L'animation a déjà été mise à jour dans update() avec dt
            com.badlogic.gdx.graphics.g2d.TextureRegion currentFrame = gunFlashAnimation.getKeyFrame(0f);
            if (currentFrame != null) {
                // Calculer la taille du flash (visible mais pas trop gros)
                float flashScale = 0.15f; // 15% de la taille originale (réduit de 30% à 15%)
                float flashWidth = currentFrame.getRegionWidth() * flashScale;
                float flashHeight = currentFrame.getRegionHeight() * flashScale;

                // Positionner et dessiner le flash au bout du canon
                float flashX = gunFlashPosition.getX() - flashWidth * 0.5f;
                float flashY = gunFlashPosition.getY() - flashHeight * 0.5f;

                // Log toutes les 0.1s pour debug (sans spammer)
                float stateTime = gunFlashAnimation.getStateTime();
                if (stateTime < 0.3f && (int) (stateTime * 10) % 1 == 0) {
                    Gdx.app.debug("PlayScreen", "🎨 DESSIN du flash:");
                    Gdx.app.debug("PlayScreen", "   📍 Position: (" + flashX + ", " + flashY + ")");
                    Gdx.app.debug("PlayScreen", "   📏 Taille: " + flashWidth + "x" + flashHeight);
                    Gdx.app.debug("PlayScreen", "   📐 Angle: " + gunFlashAngle + "°");
                    Gdx.app.debug("PlayScreen", "   🖼️ Frame size: " + currentFrame.getRegionWidth() + "x"
                            + currentFrame.getRegionHeight());
                }

                // Dessiner avec rotation pour correspondre à l'angle du canon
                batch.draw(
                        currentFrame,
                        flashX, flashY, // Position
                        flashWidth * 0.5f, flashHeight * 0.5f, // Origine de rotation (centre)
                        flashWidth, flashHeight, // Taille
                        1f, 1f, // Scale
                        gunFlashAngle // Rotation en degrés
                );
            } else {
                Gdx.app.error("PlayScreen", "❌ ERREUR: currentFrame est NULL dans renderEntities() !");
            }
        } else {
            // Log seulement la première fois pour ne pas spammer
            if (Gdx.graphics.getFrameId() % 300 == 0) {
                Gdx.app.debug("PlayScreen", "⚠️ Animation NON active: gunFlashPlaying=" + gunFlashPlaying
                        + ", gunFlashAnimation=" + (gunFlashAnimation != null ? "OK" : "NULL"));
            }
        }

        world.renderEnemies(batch);
        world.renderItems(batch);
        world.renderProjectiles(batch, assets);
        batch.end();
    }

    private void update(float dt) {
        if (isPaused) {
            return;
        }

        // Survival timer - add 1 point per second
        if (scoreManager != null) {
            survivalTimer += dt;
            if (survivalTimer >= 1.0f) {
                scoreManager.addPoints(1); // 1 seconde de survie : +1 point (cumulatif)
                survivalTimer -= 1.0f; // Keep fractional part for precision
            }
        }

        // Détecter l'évolution vers Adult et ajouter les points
        if (player != null && player.getWorm().getCurrent() != null && scoreManager != null) {
            Class<?> currentWormClass = player.getWorm().getCurrent().getClass();
            // Si c'est la première fois qu'on devient Adult (évolution depuis Baby)
            if (currentWormClass == Adult.class && lastWormClass == Baby.class) {
                scoreManager.addPoints(100); // Évolution en Adult : +100 points
                Gdx.app.log("PlayScreen", "Evolution vers Adult détectée ! +100 points ajoutés.");
            }
            lastWormClass = currentWormClass;
        }

        // Check for death
        if (player != null && player.getWorm().getCurrent() != null) {
            Wormy currentWorm = player.getWorm().getCurrent();
            boolean isSuperMode = currentWorm instanceof Super;
            
            if (currentWorm.getHealth().isDead()) {
                // Check if death was from starvation (health at 0 but no enemy cause)
                // IMPORTANT: Super mode cannot die from starvation - it's invincible to hunger
                if (deathCause == null) {
                    // Only set starvation death cause if NOT in Super mode
                    if (!isSuperMode) {
                        deathCause = "Starvation";
                    } else {
                        // In Super mode, if health is 0 without enemy cause, it's a bug - prevent death
                        Gdx.app.log("PlayScreen", "Super mode: health at 0 without enemy cause - preventing death");
                        return; // Don't trigger game over
                    }
                }
                
                // Trigger game over only if we have a valid death cause (enemy killed player)
                if (deathCause != null) {
                    game.setScreen(new GameOverScreen(game, false, deathCause, activePlayerName, scoreManager));
                    // Do not dispose here; GameOverScreen will handle cleanup when appropriate
                    return;
                }
            }
        }

        // Check if Pie was killed
        if (pieKilled) {
            Gdx.app.log("PlayScreen", "Pie was killed! Transitioning to GameOverScreen.");
            // Victory! Add bonus points for killing the Pie
            if (scoreManager != null) {
                scoreManager.addPoints(500); // Tuer la pie : +500 points (Win condition)
            }
            game.setScreen(new GameOverScreen(game, true, null, activePlayerName, scoreManager)); // Victory!
            // Do not dispose here; assets are still needed for the GameOverScreen UI
            return;
        }

        wobbleTime += dt;

        if (Gdx.input.isKeyJustPressed(Input.Keys.F1)) {
            overlayEnabled = !overlayEnabled;
            persistOverlayPreference();
        }

        inputController.update();
        player.update(dt);

        Wormy worm = player.getWorm().getCurrent();

        // Bloquer la descente à la surface pour Adult et Super
        // Ils ne peuvent descendre que dans les tunnels/puits, pas à travers la terre
        // pleine
        if (worm instanceof Adult || worm instanceof Super) {
            blockDownwardMovementAtSurface();
        }

        collisionHandler.moveAndCollide(player, dt);

        // Mettre à jour l'animation de flash de canon et vérifier si elle est terminée
        if (gunFlashPlaying && gunFlashAnimation != null) {
            // Mettre à jour l'animation (incrémente le stateTime)
            com.badlogic.gdx.graphics.g2d.TextureRegion currentFrame = gunFlashAnimation.getKeyFrame(dt);
            float stateTime = gunFlashAnimation.getStateTime();

            // Log toutes les 0.1s pour ne pas spammer
            if (stateTime < 0.3f && (int) (stateTime * 10) % 1 == 0) {
                Gdx.app.debug("PlayScreen", "⏱️ Animation update - stateTime: " + stateTime + "s, frame: "
                        + (currentFrame != null ? "OK" : "NULL"));
            }

            // Vérifier si l'animation est terminée
            if (gunFlashAnimation.isAnimationFinished(dt)) {
                Gdx.app.log("PlayScreen", "✅ Animation de flash TERMINÉE (stateTime: " + stateTime + "s)");
                gunFlashPlaying = false;
            }
        }

        // Contraindre Y pour Baby, Adult et Super à la surface (ne pas voler au-dessus)
        // Ils peuvent descendre librement dans les tunnels mais doivent ramper à la
        // surface
        if (worm instanceof Baby || worm instanceof Adult || worm instanceof Super) {
            constrainPlayerY();
        }

        if (worm instanceof Baby) {
            Baby baby = (Baby) worm;

            if (baby.getSprite() == null) {
                setupBabyAnimations();
            }

            if (baby.getSprite() != null) {
                baby.getSprite().setPosition(player.getPosition().getX(), player.getPosition().getY());
            }
            baby.applyWobble(wobbleTime);
        } else if (worm instanceof Adult) {
            Adult adult = (Adult) worm;

            if (adult.getSprite() == null) {
                setupAdultAnimations(adult);
            }

            // Position du sprite mise à jour automatiquement dans updateAnimation()
            // Identique au comportement de Baby pour avoir le même mouvement libre
            adult.applyWobble(wobbleTime);
            // Note: Le diamant est maintenant géré par le SpawnManager
        } else if (worm instanceof Super) {
            Super superWorm = (Super) worm;

            if (superWorm.getSprite() == null) {
                setupSuperAnimations(superWorm);
                // S'assurer que Super a les mêmes dimensions qu'Adult pour le comportement
                // identique
                // Les dimensions sont déjà définies dans setupSuperAnimations, mais on s'assure
                // ici aussi
            }

            // Utiliser player.getPosition() pour le sprite du worm (comme pour Baby et
            // Adult)
            // Cela garantit que tous les sprites utilisent la même source de position
            com.irina.myfirstgame.entities.valueobjects.Vector2 playerPos = player.getPosition();

            if (superWorm.getSprite() != null) {
                superWorm.getSprite().setPosition(playerPos.getX(), playerPos.getY());
            }
            // Synchroniser les dimensions avec le Player pour éviter les collisions
            // incorrectes
            // Cela assure que Super utilise les mêmes dimensions qu'Adult
            superWorm.applyWobble(wobbleTime);

            // Gérer le pistolet si Super en possède un
            if (superWorm.hasGun()) {
                // Initialiser le sprite du pistolet s'il n'existe pas
                if (superWorm.getGunSprite() == null) {
                    com.badlogic.gdx.graphics.g2d.TextureRegion gunRegion = assets.getTextureRegion("gun.png");
                    if (gunRegion != null) {
                        com.irina.myfirstgame.systems.Sprite gunSprite = new com.irina.myfirstgame.systems.Sprite(
                                gunRegion);
                        // Rétrécir le gun légèrement pour qu'il soit bien proportionné sur le wormy
                        float gunScale = 0.14f; // 14% de la taille originale (réduit légèrement)
                        gunSprite.setScale(gunScale, gunScale);
                        // CRITIQUE : Recalculer l'origine au centre APRÈS le scale pour garantir un
                        // centrage parfait
                        gunSprite.setOriginCenter();
                        superWorm.setGunSprite(gunSprite);
                    }
                }

                // Calculer l'angle du pistolet vers le curseur
                // Utiliser playerPos pour être cohérent avec la position du sprite du worm
                float mouseX = inputController.getMouseX();
                float mouseY = inputController.getMouseY();

                // Convertir les coordonnées de la souris (écran) en coordonnées du monde
                Vector3 mouseWorld = world.getCamera().getLibgdxCamera().unproject(
                        new Vector3(mouseX, mouseY, 0));

                float dx = mouseWorld.x - playerPos.getX();
                float dy = mouseWorld.y - playerPos.getY();
                float angle = (float) Math.atan2(dy, dx);
                superWorm.setGunAngle(angle);

                // Mettre à jour la position du sprite du pistolet (centré exactement sur le
                // wormy)
                if (superWorm.getGunSprite() != null && superWorm.getSprite() != null) {
                    // CRITIQUE : Utiliser exactement la même position que le sprite du worm
                    // Les deux sprites (worm et gun) ont leur origine au centre grâce à
                    // setOriginCenter()
                    // Donc setPosition() avec la même position les centre parfaitement l'un sur
                    // l'autre
                    // Utiliser la position réelle du sprite du worm pour garantir une
                    // synchronisation parfaite
                    com.irina.myfirstgame.entities.valueobjects.Vector2 wormSpritePos = superWorm.getSprite()
                            .getPosition();
                    superWorm.getGunSprite().setPosition(
                            wormSpritePos.getX(), // Exactement la même position X que le sprite du worm
                            wormSpritePos.getY()); // Exactement la même position Y que le sprite du worm

                    // Détecter si le curseur est à gauche ou à droite
                    boolean pointingLeft = dx < 0;

                    // CRITIQUE : Gérer le flip du sprite uniquement quand nécessaire pour éviter le
                    // clignotement
                    boolean isCurrentlyFlipped = superWorm.getGunSprite().isFlipX();
                    if (isCurrentlyFlipped != pointingLeft) {
                        // État différent : changer le flip
                        superWorm.getGunSprite().flip(true, false);
                    }

                    // Calculer l'angle de rotation avec mobilité complète de 180° (-90° à +90°)
                    float angleDegrees = (float) Math.toDegrees(angle);
                    float limitedAngle;

                    if (pointingLeft) {
                        // Curseur à gauche : le sprite est retourné horizontalement
                        // Angle original de atan2 : ~135° (haut-gauche) à ~-135° (bas-gauche)
                        // Normaliser l'angle dans la plage 0° à 360°
                        float normalizedAngle = angleDegrees;
                        if (normalizedAngle < 0) {
                            normalizedAngle += 360f;
                        }

                        // Convertir en angle relatif depuis la gauche : angle - 180°
                        // Cela donne : 135°→-45° (haut), 180°→0° (horizontal), 225°→+45° (bas)
                        float relativeAngle = normalizedAngle - 180f;

                        // Normaliser relativeAngle dans la plage -180° à +180°
                        if (relativeAngle > 180f) {
                            relativeAngle -= 360f;
                        } else if (relativeAngle < -180f) {
                            relativeAngle += 360f;
                        }

                        // CRITIQUE : Utiliser directement relativeAngle (sans inversion)
                        // Car le flip du sprite inverse déjà horizontalement
                        // Test : si ça ne marche pas, on inversera verticalement
                        limitedAngle = relativeAngle;
                    } else {
                        // Curseur à droite : angle normal, mobilité complète de 180° (-90° à +90°)
                        limitedAngle = angleDegrees;
                    }

                    // Limiter entre -90° et +90° pour éviter le retournement à l'envers
                    // Cela conserve la mobilité complète de 180° dans les deux directions
                    limitedAngle = Math.max(-90f, Math.min(90f, limitedAngle));
                    superWorm.getGunSprite().setRotation(limitedAngle);
                }

                // Gérer le tir au clic
                if (inputController.isJustClicked() && superWorm.canShoot()) {
                    handleShoot(superWorm, angle);
                }
            }
        }

        // Mettre à jour le SpawnManager pour gérer les respawns
        if (world.getSpawnManager() != null) {
            world.getSpawnManager().update(dt, world, assets, player);
        }

        world.update(dt);
    }

    private void handleShoot(Super superWorm, float angle) {
        if (superWorm == null || !superWorm.canShoot()) {
            return;
        }

        // Consommer une balle (canShoot() a déjà vérifié qu'il y a des munitions)
        // shoot() décrémente ammo et retourne true si succès
        if (!superWorm.shoot()) {
            // Ne devrait jamais arriver si canShoot() retourne true, mais par sécurité
            Gdx.app.debug("PlayScreen", "Échec du tir - pas de munitions (ne devrait pas arriver)");
            return;
        }

        // Vérifier si c'était la dernière balle
        if (superWorm.getAmmo() == 0) {
            Gdx.app.log("PlayScreen", "Dernière balle tirée ! Perte du pouvoir Super.");
            if (world.getSpawnManager() != null) {
                world.getSpawnManager().onSuperPowerLost();
            }

            // Revenir à la forme Adulte
            com.irina.myfirstgame.entities.wormy.Adult adult = new com.irina.myfirstgame.entities.wormy.Adult();
            setupAdultAnimations(adult);
            player.getWorm().evolveTo(adult);

            // Réinitialiser stats
            adult.getHealth().setCurrentHealth(100);
            adult.getHunger().setCurrentHunger(50);

            Gdx.app.log("PlayScreen", "Joueur redevenu Adult (invincibilité perdue)");
        }

        // Créer un projectile
        com.irina.myfirstgame.objects.Projectile projectile = new com.irina.myfirstgame.objects.Projectile();

        // Position de départ du projectile (au bout du canon)
        // Utiliser player.getPosition() pour être cohérent avec la position du sprite
        // du gun
        com.irina.myfirstgame.entities.valueobjects.Vector2 wormPos = player.getPosition();

        // Calculer la position du bout du canon
        // Le gun sprite est centré sur le wormy avec un scale de 0.16f
        // La longueur du canon = environ la moitié de la largeur du gun sprite
        com.badlogic.gdx.graphics.g2d.TextureRegion gunRegion = assets.getTextureRegion("gun.png");
        float gunTextureWidth = gunRegion != null ? gunRegion.getRegionWidth() : 32f;
        float gunScale = 0.16f; // Même scale que le gun affiché
        float gunWidth = gunTextureWidth * gunScale; // Largeur réelle du gun affiché
        float gunLength = gunWidth * 0.6f; // Longueur du canon (60% de la largeur du gun)

        // Position du bout du canon = centre du wormy + offset selon l'angle
        float startX = wormPos.getX() + (float) Math.cos(angle) * gunLength;
        float startY = wormPos.getY() + (float) Math.sin(angle) * gunLength;
        projectile.setPosition(startX, startY);

        // Démarrer l'animation de flash au bout du canon
        Gdx.app.log("PlayScreen", "🔫 handleShoot() appelé - angle: " + Math.toDegrees(angle) + "°");
        if (gunFlashAnimation != null) {
            gunFlashPosition.setX(startX);
            gunFlashPosition.setY(startY);
            gunFlashAngle = (float) Math.toDegrees(angle);
            gunFlashAnimation.reset();
            gunFlashAnimation.play();
            gunFlashPlaying = true;
            Gdx.app.log("PlayScreen", "🎬 Animation de flash DÉMARRÉE !");
            Gdx.app.log("PlayScreen", "   📍 Position: (" + startX + ", " + startY + ")");
            Gdx.app.log("PlayScreen", "   📐 Angle: " + gunFlashAngle + "°");
            Gdx.app.log("PlayScreen", "   ▶️ gunFlashPlaying = " + gunFlashPlaying);
        } else {
            Gdx.app.error("PlayScreen", "❌ CRITIQUE: gunFlashAnimation est NULL ! L'animation ne peut pas démarrer.");
        }

        // Direction du projectile (normalisée pour une trajectoire droite)
        com.irina.myfirstgame.entities.valueobjects.Vector2 direction = new com.irina.myfirstgame.entities.valueobjects.Vector2(
                (float) Math.cos(angle), (float) Math.sin(angle));
        direction.nor(); // Normaliser pour garantir une vitesse constante
        projectile.setDirection(direction);

        // Définir les dimensions du projectile (petites mais visibles)
        // Les balles doivent être petites mais pas trop, pas la taille complète de
        // bullet.png
        float bulletDisplaySize = 6f; // Taille d'affichage (6x6 pixels, légèrement agrandie)
        projectile.setWidth(bulletDisplaySize);
        projectile.setHeight(bulletDisplaySize);

        // Lancer le projectile
        projectile.launch(new com.irina.myfirstgame.entities.valueobjects.Vector2(startX, startY));

        // Ajouter le projectile au monde
        world.addProjectile(projectile);

        Gdx.app.debug("PlayScreen", "Tir effectué ! Balles restantes: " + superWorm.getAmmo());
    }

    private void blockDownwardMovementAtSurface() {
        Wormy worm = player.getWorm().getCurrent();
        if (worm == null)
            return;

        float x = player.getPosition().getX();
        float y = player.getPosition().getY();
        float maxY = getMaxYForX(x);

        Vector2 vel = player.getVelocity();
        if (vel == null || vel.getY() >= 0)
            return; // Pas de descente, pas besoin de vérifier

        // Si on est à la surface (y proche de maxY = niveau de l'herbe)
        // Tolérance de 5 pixels pour considérer qu'on est à la surface
        float surfaceTolerance = 5f;
        boolean isAtSurface = y >= maxY - surfaceTolerance && y <= maxY + surfaceTolerance;

        if (!isAtSurface) {
            // Pas à la surface, on peut descendre librement
            return;
        }

        // On est à la surface : vérifier s'il y a un tunnel/puit directement en dessous
        // Si oui, on peut descendre. Si non (terre pleine), on bloque la descente
        float tileHeight = world.getMap().getTileHeight();
        float playerHeight = player.getHeight();
        float halfHeight = playerHeight * 0.5f;

        // Vérifier s'il y a de la terre pleine directement sous le joueur
        // Vérifier plusieurs points horizontalement pour être sûr
        float checkY = y - halfHeight - tileHeight * 0.5f; // Position en dessous du joueur
        float leftX = x - player.getWidth() * 0.3f;
        float centerX = x;
        float rightX = x + player.getWidth() * 0.3f;

        // Vérifier si les points en dessous sont solides (terre pleine)
        boolean solidBelowLeft = world.getMap().isSolidAt(leftX, checkY, COLLISION_LAYER);
        boolean solidBelowCenter = world.getMap().isSolidAt(centerX, checkY, COLLISION_LAYER);
        boolean solidBelowRight = world.getMap().isSolidAt(rightX, checkY, COLLISION_LAYER);

        // Si TOUS les points en dessous sont solides, c'est de la terre pleine :
        // bloquer la descente
        // Si au moins un point est libre, c'est un tunnel/puit : permettre la descente
        boolean allSolidBelow = solidBelowLeft && solidBelowCenter && solidBelowRight;

        if (allSolidBelow) {
            // Terre pleine en dessous : bloquer la descente
            vel.setY(0);
            Gdx.app.debug("PlayScreen", "Descente bloquée : terre pleine en dessous (y=" + y + ", maxY=" + maxY + ")");
        } else {
            // Tunnel/puit en dessous : permettre la descente
            Gdx.app.debug("PlayScreen", "Descente autorisée : tunnel/puit détecté en dessous");
        }
    }

    private void constrainPlayerY() {
        Wormy worm = player.getWorm().getCurrent();
        if (worm == null)
            return;

        float x = player.getPosition().getX();
        float y = player.getPosition().getY();
        float maxY = getMaxYForX(x);

        // Logique spécifique pour Baby : ne peut pas sortir du tunnel
        if (worm instanceof com.irina.myfirstgame.entities.wormy.Baby) {
            // Baby doit rester SOUS la surface (dans le tunnel)
            // maxY est le niveau de l'herbe (surface)
            // On soustrait la hauteur d'une tuile pour obtenir le plafond du tunnel
            float tunnelCeiling = maxY - world.getMap().getTileHeight();

            // Ajouter une petite marge pour éviter de bloquer pile au plafond
            float babyMaxY = tunnelCeiling - 2f;

            if (y > babyMaxY) {
                player.setPosition(x, babyMaxY);
                Vector2 vel = player.getVelocity();
                if (vel != null && vel.getY() > 0) {
                    vel.setY(0);
                }
            }
            return; // Baby géré, on sort
        }

        // For Adult and Super : existing behavior (can go to the surface but not fly)
        // Ne contraindre QUE si on est AU-DESSUS de la surface (pour empêcher de voler)
        // Si on est en dessous ou au niveau de la surface (dans un tunnel ou sur
        // l'herbe),
        // on NE DOIT PAS être contraint pour permettre le mouvement vertical libre dans
        // les tunnels
        // Tolérance de 2 pixels pour éviter les problèmes de précision flottante
        if (y > maxY + 2f) {
            // On est vraiment au-dessus de la surface : contraindre pour empêcher de voler
            player.setPosition(x, maxY);
            // Annuler la vélocité verticale vers le haut pour éviter de continuer à voler
            Vector2 vel = player.getVelocity();
            if (vel != null && vel.getY() > 0) {
                vel.setY(0);
            }
        }
        // Si y <= maxY + 2f, on est dans un tunnel ou à la surface : PAS DE CONTRAINTE
        // Le mouvement vertical dans les tunnels est géré par moveAndCollide() qui
        // vérifie les collisions
    }

    private float getMaxYForX(float x) {
        if (x < 921f) {
            return 678f;
        } else if (x < 1179f) {
            return 750f;
        } else if (x < 1741f) {
            return 678f;
        } else if (x < 1952f) {
            return 750f;
        } else {
            return 678f;
        }
    }

    private void persistOverlayPreference() {
        if (preferences != null) {
            preferences.putBoolean(PREF_OVERLAY, overlayEnabled);
            preferences.flush();
        }
    }

    private String resolvePlayerName() {
        if (requestedPlayerName != null) {
            return requestedPlayerName;
        }
        if (game != null) {
            String cached = sanitizeName(game.getCachedPlayerName());
            if (cached != null) {
                return cached;
            }
        }
        if (preferences != null) {
            String stored = sanitizeName(preferences.getString(PREF_PLAYER_NAME, ""));
            if (stored != null) {
                return stored;
            }
        }
        return "Wormy";
    }

    private String sanitizeName(String candidate) {
        if (candidate == null) {
            return null;
        }
        String trimmed = candidate.trim();
        return trimmed.isEmpty() ? null : trimmed;
    }

    @Override
    public void resize(int width, int height) {
        world.getCamera().updateViewport(width, height);
        if (pauseMenu != null) {
            pauseMenu.resize(width, height);
        }
        if (hud != null) {
            hud.resize(width, height);
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
    }

    @Override
    public void dispose() {
        if (batch != null) {
            batch.dispose();
        }
        if (world != null) {
            world.dispose();
        }
        if (assets != null) {
            assets.dispose();
        }
        if (mapRenderer != null) {
            mapRenderer.dispose();
        }
        if (pauseMenu != null) {
            pauseMenu.dispose();
        }
    }
}
