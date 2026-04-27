package com.irina.myfirstgame.core.ui;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.OrthographicCamera;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Matrix4;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.wormy.Adult;
import com.irina.myfirstgame.entities.wormy.Baby;
import com.irina.myfirstgame.entities.wormy.Super;
import com.irina.myfirstgame.entities.wormy.WormState;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.objects.Scoreboard;
import com.irina.myfirstgame.objects.Timer;

public class HUD {
    private Wormy boundWormy;
    private Scoreboard scoreboard;
    private Timer time;
    private Assets assets;
    private SpriteBatch batch;
    private OrthographicCamera hudCamera;
    private int currentScore = 0; // Current game score
    
    // Messages d'aide contextuels
    private String helpMessage = null;
    private float helpMessageTimer = 0f;
    private static final float HELP_MESSAGE_DURATION = 5.0f; // Durée d'affichage en secondes
    private boolean helpMessageShownBaby = false;
    private boolean helpMessageShownAdult = false;
    private boolean helpMessageShownSuper = false;

    // Tailles fixes pour garantir un ratio 1:1 non écrasé (comme une div/box
    // invisible)
    private static final float HUD_VIEWPORT_WIDTH = 1920f; // Largeur fixe du viewport HUD
    private static final float HUD_VIEWPORT_HEIGHT = 1080f; // Hauteur fixe du viewport HUD
    private static final float ICON_SIZE = 48f; // Taille fixe des icônes (ratio 1:1 garanti)
    private static final float PADDING = 18f; // Padding fixe (augmenté pour espacer plus vers la droite)
    private static final float START_X = 30f; // Position de départ X fixe
    private static final float START_Y = 30f; // Position de départ Y fixe

    public HUD(Assets assets, SpriteBatch batch) {
        this.assets = assets;
        this.batch = batch;
        // Créer une caméra orthographique avec viewport fixe pour garantir les ratios
        this.hudCamera = new OrthographicCamera();
        // Utiliser un viewport fixe pour que les tailles soient toujours identiques
        this.hudCamera.setToOrtho(false, HUD_VIEWPORT_WIDTH, HUD_VIEWPORT_HEIGHT);
        this.hudCamera.update();
    }
    
    public void bind(Wormy worm, Scoreboard scores) {
        this.boundWormy = worm;
        this.scoreboard = scores;
    }

    /**
     * Update the current score displayed in the HUD
     */
    public void updateScore(int score) {
        this.currentScore = score;
    }

    public void resize(int width, int height) {
        // Toujours utiliser le viewport fixe pour garantir les ratios
        // Le viewport fixe fait office de "box invisible" avec dimensions stables
        hudCamera.setToOrtho(false, HUD_VIEWPORT_WIDTH, HUD_VIEWPORT_HEIGHT);
        hudCamera.update();
    }
    
    public void render() {
        if (boundWormy == null || assets == null || batch == null) {
            return;
        }

        WormState wormState = boundWormy.getEvolutionState();
        if (wormState == null) {
            return;
        }

        // Sauvegarder la matrice actuelle
        Matrix4 oldMatrix = batch.getProjectionMatrix();

        // Utiliser le viewport fixe de la caméra pour garantir les ratios
        // Cette matrice fixe fait office de "box invisible" avec dimensions stables
        // IMPORTANT: Utiliser la matrice combinée de la caméra pour le viewport fixe
        batch.setProjectionMatrix(hudCamera.combined);
        batch.begin();

        // Utiliser les coordonnées du viewport fixe (0,0 en bas à gauche)
        float y = HUD_VIEWPORT_HEIGHT - START_Y; // En haut à gauche du viewport fixe
        float maxItemHeight = 0f; // Pour garder la hauteur maximale des items pour le gun en dessous

        // Taille de base pour tous les items (BEAUCOUP plus petit)
        // IMPORTANT: Cette taille est utilisée comme RÉFÉRENCE pour la hauteur
        // uniquement
        // La largeur sera calculée PROPORTIONNELLEMENT au ratio réel de la texture
        float itemBaseSize = ICON_SIZE * 1.5f; // 72 pixels de référence (beaucoup plus petit)

        // --- HEALTH BAR (Top Right) ---
        if (boundWormy != null && !(boundWormy instanceof Super)) {
            int currentHealth = boundWormy.getHealth().getCurrentHealth();
            int maxHealth = boundWormy.getHealth().getMaxHealth();

            // Calculate health percentage (0.0 to 1.0)
            float healthPercent = (float) currentHealth / maxHealth;

            // Map percentage to sprite index (0 to 7)
            // 100% -> index 0 (Full)
            // 0% -> index 7 (Empty)
            int rawIndex = (int) Math.ceil(healthPercent * 7);
            rawIndex = Math.max(0, Math.min(7, rawIndex));
            int spriteIndex = 7 - rawIndex; // Invert: 0 is full, 7 is empty

            TextureRegion healthRegion = assets.getTextureRegion("health_" + spriteIndex + ".png");

            if (healthRegion != null) {
                // Fixed size for both health and energy bars
                float barWidth = 200f; // Fixed width for consistency
                float barHeight = 40f; // Fixed height for consistency

                // Position at top right with padding
                float healthX = HUD_VIEWPORT_WIDTH - barWidth - START_X;
                float healthY = HUD_VIEWPORT_HEIGHT - START_Y - barHeight;

                batch.draw(healthRegion, healthX, healthY, barWidth, barHeight);
            }
        }

        // --- ENERGY BAR (Top Right, below health bar) ---
        if (boundWormy != null && !(boundWormy instanceof Super)) {
            int currentHunger = boundWormy.getHunger().getCurrentHunger();
            int maxHunger = boundWormy.getHunger().getMaxHunger();

            // Only show energy bar if maxHunger > 0 (not in Super state)
            if (maxHunger > 0) {
                // Calculate hunger percentage (0.0 to 1.0)
                float hungerPercent = (float) currentHunger / maxHunger;

                // Map percentage to sprite index (0 to 5)
                // 100% -> index 0 (Full)
                // 0% -> index 5 (Empty)
                int rawIndex = (int) Math.ceil(hungerPercent * 5);
                rawIndex = Math.max(0, Math.min(5, rawIndex));
                int spriteIndex = 5 - rawIndex; // Invert: 0 is full, 5 is empty

                TextureRegion energyRegion = assets.getTextureRegion("energy_" + spriteIndex + ".png");

                if (energyRegion != null) {
                    // Fixed size for energy bar - agrandie de 5-10% par rapport à la barre de vie
                    float barWidth = 210f; // Largeur augmentée de 5% (200 * 1.05)
                    float barHeight = 44f; // Hauteur augmentée de 10% (40 * 1.1)

                    // Position at top right, below the health bar
                    float energyX = HUD_VIEWPORT_WIDTH - barWidth - START_X;
                    float energyY = HUD_VIEWPORT_HEIGHT - START_Y - barHeight - PADDING - barHeight;

                    batch.draw(energyRegion, energyX, energyY, barWidth, barHeight);
                }
            }
        }

        // Première passe : calculer les dimensions de chaque item en préservant
        // ABSOLUMENT le ratio
        float totalWidth = 0f; // Largeur totale des items + paddings
        float[] displayWidths = new float[3];
        float[] displayHeights = new float[3];
        int itemCount = 0;

        if (wormState.hasAteBurger()) {
            TextureRegion burgerRegion = assets.getTextureRegion("burger.png");
            if (burgerRegion != null) {
                // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion (en pixels de
                // texture)
                float originalWidth = (float) burgerRegion.getRegionWidth();
                float originalHeight = (float) burgerRegion.getRegionHeight();

                // CRITICAL: Calculer le ratio RÉEL (largeur / hauteur)
                float realRatio = originalWidth / originalHeight;

                // CRITICAL: Utiliser itemBaseSize comme hauteur de référence
                // La largeur sera calculée EXACTEMENT selon le ratio réel
                displayHeights[itemCount] = itemBaseSize;
                displayWidths[itemCount] = itemBaseSize * realRatio; // Ratio RÉEL préservé à 100%

                if (displayHeights[itemCount] > maxItemHeight) {
                    maxItemHeight = displayHeights[itemCount];
                }
                totalWidth += displayWidths[itemCount];
                itemCount++;
            }
        }

        if (wormState.hasAteFrites()) {
            TextureRegion fritesRegion = assets.getTextureRegion("frites.png");
            if (fritesRegion != null) {
                if (itemCount > 0)
                    totalWidth += PADDING; // Ajouter padding entre items

                // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion
                float originalWidth = (float) fritesRegion.getRegionWidth();
                float originalHeight = (float) fritesRegion.getRegionHeight();

                // CRITICAL: Calculer le ratio RÉEL
                float realRatio = originalWidth / originalHeight;

                // CRITICAL: Utiliser itemBaseSize comme hauteur, largeur selon ratio réel
                displayHeights[itemCount] = itemBaseSize;
                displayWidths[itemCount] = itemBaseSize * realRatio; // Ratio RÉEL préservé à 100%

                if (displayHeights[itemCount] > maxItemHeight) {
                    maxItemHeight = displayHeights[itemCount];
                }
                totalWidth += displayWidths[itemCount];
                itemCount++;
            }
        }

        if (wormState.hasAteSoda()) {
            TextureRegion sodaRegion = assets.getTextureRegion("soda.png");
            if (sodaRegion != null) {
                if (itemCount > 0)
                    totalWidth += PADDING; // Ajouter padding entre items

                // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion
                float originalWidth = (float) sodaRegion.getRegionWidth();
                float originalHeight = (float) sodaRegion.getRegionHeight();

                // CRITICAL: Calculer le ratio RÉEL
                float realRatio = originalWidth / originalHeight;

                // CRITICAL: Utiliser itemBaseSize comme hauteur, largeur selon ratio réel
                displayHeights[itemCount] = itemBaseSize;
                displayWidths[itemCount] = itemBaseSize * realRatio; // Ratio RÉEL préservé à 100%

                if (displayHeights[itemCount] > maxItemHeight) {
                    maxItemHeight = displayHeights[itemCount];
                }
                totalWidth += displayWidths[itemCount];
                itemCount++;
            }
        }

        // Position X de départ : coin gauche (START_X)
        float startX = START_X; // Coin gauche
        float currentX = startX;
        itemCount = 0; // Réinitialiser pour la deuxième passe

        // Deuxième passe : afficher les items alignés à gauche
        if (wormState.hasAteBurger()) {
            TextureRegion burgerRegion = assets.getTextureRegion("burger.png");
            if (burgerRegion != null) {
                batch.draw(burgerRegion, currentX, y - displayHeights[itemCount],
                        displayWidths[itemCount], displayHeights[itemCount]);
                currentX += displayWidths[itemCount] + PADDING;
                itemCount++;
            }
        }

        if (wormState.hasAteFrites()) {
            TextureRegion fritesRegion = assets.getTextureRegion("frites.png");
            if (fritesRegion != null) {
                batch.draw(fritesRegion, currentX, y - displayHeights[itemCount],
                        displayWidths[itemCount], displayHeights[itemCount]);
                currentX += displayWidths[itemCount] + PADDING;
                itemCount++;
            }
        }

        if (wormState.hasAteSoda()) {
            TextureRegion sodaRegion = assets.getTextureRegion("soda.png");
            if (sodaRegion != null) {
                batch.draw(sodaRegion, currentX, y - displayHeights[itemCount],
                        displayWidths[itemCount], displayHeights[itemCount]);
            }
        }

        // Afficher le pistolet JUSTE EN DESSOUS des items food, aligné à gauche
        if (boundWormy instanceof Super) {
            Super superWorm = (Super) boundWormy;
            if (superWorm.hasGun()) {
                // Positionner juste en dessous des items food
                float gunY = maxItemHeight > 0 ? (y - maxItemHeight - PADDING) : (y - itemBaseSize - PADDING);

                TextureRegion gunRegion = assets.getTextureRegion("gun.png");
                if (gunRegion != null) {
                    // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion
                    float originalGunWidth = (float) gunRegion.getRegionWidth();
                    float originalGunHeight = (float) gunRegion.getRegionHeight();

                    // CRITICAL: Calculer le ratio RÉEL
                    float realGunRatio = originalGunWidth / originalGunHeight;

                    // CRITICAL: Utiliser itemBaseSize comme hauteur de référence
                    // La largeur sera calculée EXACTEMENT selon le ratio réel
                    float displayHeight = itemBaseSize;
                    float displayWidth = itemBaseSize * realGunRatio; // Ratio RÉEL préservé à 100%

                    // Position X du gun : aligné à gauche (START_X) comme les items food
                    float gunX = START_X;

                    // Afficher le gun avec son ratio réel (pas écrasé), aligné à gauche sous les
                    // items food
                    batch.draw(gunRegion, gunX, gunY - displayHeight, displayWidth, displayHeight);

                    // Mettre à jour la position Y pour les munitions (juste en dessous du gun)
                    float currentY = gunY - displayHeight - PADDING;

                    // Afficher les munitions du pistolet (en dessous du gun, alignées à gauche
                    // comme le gun)
                    float bulletsX = gunX; // Alignées à gauche comme le gun

                    // Essayer de charger gun-HUD.png (frame pour les munitions)
                    TextureRegion gunHudFrame = assets.getTextureRegion("gun-HUD.png");

                    // Afficher le frame du HUD des munitions si disponible avec ratio réel
                    if (gunHudFrame != null) {
                        // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion
                        float originalFrameWidth = (float) gunHudFrame.getRegionWidth();
                        float originalFrameHeight = (float) gunHudFrame.getRegionHeight();

                        // CRITICAL: Calculer le ratio RÉEL
                        float realFrameRatio = originalFrameWidth / originalFrameHeight;

                        // Taille appropriée pour le frame (légèrement plus petit que le gun)
                        float frameBaseSize = itemBaseSize * 0.9f;

                        // CRITICAL: Utiliser frameBaseSize comme hauteur, largeur selon ratio réel
                        float displayFrameHeight = frameBaseSize;
                        float displayFrameWidth = frameBaseSize * realFrameRatio; // Ratio RÉEL préservé à 100%

                        // Afficher le frame avec son ratio réel (pas écrasé), juste en dessous du gun
                        batch.draw(gunHudFrame, bulletsX, currentY - displayFrameHeight, displayFrameWidth,
                                displayFrameHeight);
                        // Mettre à jour currentY pour les balles
                        currentY -= displayFrameHeight + PADDING;
                    }

                    // Afficher les munitions restantes (2 colonnes x 3 rangs = 6 munitions max)
                    // Affichage : 6 balles -> 5 balles -> 4 balles -> 3 balles -> 2 balles -> 1
                    // balle
                    // Positionner les munitions en dessous du gun, alignées à la marge gauche
                    TextureRegion bulletRegion = assets.getTextureRegion("bullet.png");
                    if (bulletRegion != null) {
                        // CRITICAL: Obtenir les dimensions RÉELLES de la TextureRegion
                        float originalBulletWidth = (float) bulletRegion.getRegionWidth();
                        float originalBulletHeight = (float) bulletRegion.getRegionHeight();

                        // CRITICAL: Calculer le ratio RÉEL
                        float realBulletRatio = originalBulletWidth / originalBulletHeight;

                        // Taille des munitions : environ 40% de itemBaseSize pour cohérence
                        float bulletSize = itemBaseSize * 0.4f;

                        // CRITICAL: Utiliser bulletSize comme hauteur, largeur selon ratio réel
                        float displayBulletHeight = bulletSize;
                        float displayBulletWidth = bulletSize * realBulletRatio; // Ratio RÉEL préservé à 100%

                        float bulletPadding = 6f; // Padding fixe entre les munitions
                        int ammo = superWorm.getAmmo();

                        // Position de départ de la grille de munitions (alignée à la marge gauche
                        // START_X)
                        // Positionner les munitions juste en dessous du gun/frame
                        float bulletsStartX = bulletsX; // Aligné à la marge gauche
                        float bulletsStartY = currentY; // Juste en dessous du frame

                        // Afficher les munitions dans une grille 2 colonnes x 3 rangs
                        // Ordre : de gauche à droite, de haut en bas (6, 5, 4, 3, 2, 1)
                        for (int row = 0; row < 3; row++) {
                            for (int col = 0; col < 2; col++) {
                                int index = row * 2 + col; // Index de 0 à 5 (6 positions)
                                // Calculer la position de la balle dans la grille (ratio préservé)
                                float bulletX = bulletsStartX + col * (displayBulletWidth + bulletPadding);
                                // Inverser l'ordre des rangs pour afficher du haut vers le bas
                                float bulletY = bulletsStartY
                                        - (row * (displayBulletHeight + bulletPadding) + displayBulletHeight);

                                // Afficher la balle seulement si elle existe (index < ammo)
                                // ammo va de 0 à 6, donc on affiche les munitions de 6 à 1
                                if (index < ammo) {
                                    // Dessiner avec dimensions réelles pour garantir ratio (pas d'écrasement)
                                    batch.draw(bulletRegion, bulletX, bulletY, displayBulletWidth, displayBulletHeight);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Mettre à jour et afficher le message d'aide contextuel en haut, centré, dans une box marron
        float delta = com.badlogic.gdx.Gdx.graphics.getDeltaTime();
        updateHelpMessage(delta);
        
        // Créer un ShapeRenderer et une font pour le rendu
        com.badlogic.gdx.graphics.glutils.ShapeRenderer shapeRenderer = new com.badlogic.gdx.graphics.glutils.ShapeRenderer();
        com.badlogic.gdx.graphics.g2d.BitmapFont font = new com.badlogic.gdx.graphics.g2d.BitmapFont();
        
        // Afficher le message d'aide si présent
        if (helpMessage != null && helpMessageTimer > 0f) {
            font.getData().setScale(1.3f); // Taille du message d'aide
            com.badlogic.gdx.graphics.g2d.GlyphLayout helpLayout = new com.badlogic.gdx.graphics.g2d.GlyphLayout(font,
                    helpMessage);
            float padding = 25f; // Padding augmenté pour une box plus large mais pas envahissante
            float boxWidth = helpLayout.width + padding * 2;
            float boxHeight = helpLayout.height + padding * 2;
            float helpX = (HUD_VIEWPORT_WIDTH - boxWidth) / 2; // Centré horizontalement
            float helpY = HUD_VIEWPORT_HEIGHT - boxHeight - 20f; // En haut avec padding
            
            // Dessiner la box avec ShapeRenderer (doit être après batch.end() ou avant batch.begin())
            // On va fermer batch temporairement pour dessiner la box
            batch.end();
            
            shapeRenderer.setProjectionMatrix(hudCamera.combined);
            
            // Dessiner le fond marron plus foncé
            shapeRenderer.begin(com.badlogic.gdx.graphics.glutils.ShapeRenderer.ShapeType.Filled);
            com.badlogic.gdx.graphics.Color brown = new com.badlogic.gdx.graphics.Color(0.5f, 0.35f, 0.2f, 0.95f); // Marron plus foncé
            shapeRenderer.setColor(brown);
            shapeRenderer.rect(helpX, helpY, boxWidth, boxHeight);
            shapeRenderer.end();
            
            // Dessiner les bords marron très foncé et plus épais (en dessinant plusieurs lignes pour l'épaisseur)
            shapeRenderer.begin(com.badlogic.gdx.graphics.glutils.ShapeRenderer.ShapeType.Line);
            com.badlogic.gdx.graphics.Color veryDarkBrownBorder = new com.badlogic.gdx.graphics.Color(0.2f, 0.1f, 0.05f, 1.0f); // Marron très foncé
            shapeRenderer.setColor(veryDarkBrownBorder);
            float borderThickness = 4f; // Épaisseur de la bordure
            
            // Dessiner les 4 bords avec plusieurs lignes pour créer une bordure épaisse
            // Bord du haut (plusieurs lignes pour l'épaisseur)
            for (float i = 0; i < borderThickness; i++) {
                shapeRenderer.line(helpX, helpY + boxHeight - i, helpX + boxWidth, helpY + boxHeight - i);
            }
            // Bord du bas
            for (float i = 0; i < borderThickness; i++) {
                shapeRenderer.line(helpX, helpY + i, helpX + boxWidth, helpY + i);
            }
            // Bord de gauche
            for (float i = 0; i < borderThickness; i++) {
                shapeRenderer.line(helpX + i, helpY, helpX + i, helpY + boxHeight);
            }
            // Bord de droite
            for (float i = 0; i < borderThickness; i++) {
                shapeRenderer.line(helpX + boxWidth - i, helpY, helpX + boxWidth - i, helpY + boxHeight);
            }
            shapeRenderer.end();
            
            // Reprendre le batch pour dessiner le texte
            batch.begin();
            
            // Texte du message d'aide en blanc
            font.setColor(com.badlogic.gdx.graphics.Color.WHITE);
            font.draw(batch, helpMessage, helpX + padding, helpY + padding + helpLayout.height);
        }
        
        // Display score sous les barres de vie/faim, collé à la marge droite
        font.getData().setScale(1.5f); // Taille du score
        String scoreText = "Score: " + currentScore;
        com.badlogic.gdx.graphics.g2d.GlyphLayout layout = new com.badlogic.gdx.graphics.g2d.GlyphLayout(font,
                scoreText);
        
        // Position du score : sous les barres de vie/faim, collé à droite
        // Les barres ont une hauteur de 40f et sont espacées de PADDING
        float barHeight = 40f;
        float scoreY = HUD_VIEWPORT_HEIGHT - START_Y - barHeight - PADDING - barHeight - PADDING - layout.height; // Sous les deux barres
        float scoreX = HUD_VIEWPORT_WIDTH - START_X - layout.width; // Aligné à droite
        
        font.setColor(com.badlogic.gdx.graphics.Color.WHITE);
        font.draw(batch, scoreText, scoreX, scoreY);
        
        batch.end();
        
        // Nettoyer le ShapeRenderer (créé même si le message n'est pas affiché)
        shapeRenderer.dispose();

        // Restaurer l'ancienne matrice de projection
        batch.setProjectionMatrix(oldMatrix);
    }
    
    /**
     * Met à jour le message d'aide en fonction de l'état actuel du joueur
     * 
     * @param delta Temps écoulé depuis la dernière frame
     */
    private void updateHelpMessage(float delta) {
        if (boundWormy == null) {
            return;
        }
        
        // Détecter l'état actuel du joueur et afficher le message correspondant
        if (boundWormy instanceof Baby && !helpMessageShownBaby) {
            // Phase 1 : Baby - Afficher le message une seule fois au début
            helpMessage = "Ramassez les 3 types d'aliments pour évoluer vers l'état Adulte";
            helpMessageTimer = HELP_MESSAGE_DURATION;
            helpMessageShownBaby = true;
        } else if (boundWormy instanceof Adult && !helpMessageShownAdult) {
            // Phase 2 : Adult - Afficher quand le joueur devient Adult
            helpMessage = "Montez à la surface et ramassez le Diamant pour activer le mode Super";
            helpMessageTimer = HELP_MESSAGE_DURATION;
            helpMessageShownAdult = true;
        } else if (boundWormy instanceof Super && !helpMessageShownSuper) {
            // Phase 3 : Super - Afficher quand le joueur devient Super
            helpMessage = "Tirez sur la Pie pour remporter la partie !";
            helpMessageTimer = HELP_MESSAGE_DURATION;
            helpMessageShownSuper = true;
        }
        
        // Décrémenter le timer
        if (helpMessageTimer > 0f) {
            helpMessageTimer -= delta;
            if (helpMessageTimer <= 0f) {
                helpMessage = null;
                helpMessageTimer = 0f;
            }
        }
    }
    
    /**
     * Réinitialise les messages d'aide (appelé au début d'une nouvelle partie)
     */
    public void resetHelpMessages() {
        helpMessage = null;
        helpMessageTimer = 0f;
        helpMessageShownBaby = false;
        helpMessageShownAdult = false;
        helpMessageShownSuper = false;
    }
}
