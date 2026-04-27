package com.irina.myfirstgame.entities.enemy;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.Animation.PlayMode;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.systems.Animation;
import com.irina.myfirstgame.systems.Sprite;

/**
 * Ennemi oiseau (Pie) qui patrouille dans le ciel et plonge pour attaquer le joueur.
 * <p>
 * La Pie a trois états : PATROL (patrouille), DIVING (plongeon) et RECOVERING (récupération).
 * Elle suit le joueur horizontalement et plonge lorsqu'il est à portée.
 * </p>
 *
 * @author Irina
 * @version 1.0
 * @since 1.0
 */
public class Pie extends Enemy {

    private static final int SHEET_COLUMNS = 9;
    private static final int SHEET_ROWS = 6;
    private static final int FRAME_SIZE = 64;

    private static final float PATROL_WIDTH = 240f;
    private static final float PATROL_SPEED_X = 100f; // Reduced from 160f to give worm more time
    private static final float ORIGIN_FOLLOW_SMOOTH = 2.0f;

    private static final float DIVE_SPEED = 90f; // Reduced from 140f to make it much slower
    private static final float DIVE_COOLDOWN = 2.5f;
    private static final float DIVE_TRIGGER_DISTANCE = 220f;
    private static final float DIVE_IMPACT_RADIUS = 32f;
    private static final int DIVE_DAMAGE = 7;

    private static final float RECOVER_SPEED = 150f;
    private static final float MIN_ALTITUDE_BUFFER = 48f;
    private static final float SKY_OFFSET = 220f;

    private enum State {
        PATROL,
        DIVING,
        RECOVERING
    }

    private final Player target;
    private final Vector2 origin;
    private final Animation flyAnimation;
    private final Animation diveAnimation;
    private final Sprite sprite;
    private final float surfaceLevel;

    private final Vector2 tempVec = new Vector2();
    private final Vector2 diveTarget = new Vector2();

    private State state = State.PATROL;
    private float cooldownTimer = 0f;
    private float timeAtGrassLevel = 0f; // Timer pour forcer la récupération si on reste trop longtemps au niveau de
                                         // l'herbe

    private float lastX;
    private final float minAltitude;
    private final float hoverAltitude;
    private int patrolDirection = 1;

    public Pie(Assets assets, Vector2 spawnPosition, Player target, float surfaceLevel) {
        super(spawnPosition.getX(), spawnPosition.getY());
        this.target = target;
        this.surfaceLevel = surfaceLevel;
        float baseHover = surfaceLevel + SKY_OFFSET;
        float spawnY = Math.max(spawnPosition.getY(), baseHover);
        this.minAltitude = surfaceLevel + MIN_ALTITUDE_BUFFER;
        this.hoverAltitude = Math.max(baseHover, minAltitude + 16f);
        this.origin = new Vector2(spawnPosition.getX(), hoverAltitude);
        getPosition().set(origin.getX(), hoverAltitude);

        TextureRegion[][] regions = TextureRegion.split(
                assets.getTexture("bird.png"),
                FRAME_SIZE,
                FRAME_SIZE);

        this.flyAnimation = new Animation(1f / 12f, sliceRow(regions, 1, 0, SHEET_COLUMNS));
        this.flyAnimation.setPlayMode(PlayMode.LOOP);

        this.diveAnimation = new Animation(1f / 18f, sliceRow(regions, 4, 0, SHEET_COLUMNS));
        this.diveAnimation.setPlayMode(PlayMode.NORMAL);

        TextureRegion initialFrame = flyAnimation.getKeyFrame(0f);
        this.sprite = new Sprite(initialFrame);
        this.sprite.setOriginCenter();
        this.sprite.setPosition(getPosition());

        setWidth(sprite.getWidth());
        setHeight(sprite.getHeight());
        lastX = origin.getX();
    }

    @Override
    public void update(float delta) {
        cooldownTimer = Math.max(0f, cooldownTimer - delta);

        switch (state) {
            case PATROL:
                updatePatrol(delta);
                break;
            case DIVING:
                updateDive(delta);
                break;
            case RECOVERING:
                updateRecover(delta);
                break;
            default:
                break;
        }

        updateSprite(delta);
    }

    public void draw(SpriteBatch batch) {
        sprite.draw(batch);
    }

    private void updatePatrol(float delta) {
        float desiredOriginX = origin.getX();
        if (target != null && target.getPosition() != null) {
            desiredOriginX = target.getPosition().getX();
        }
        float followFactor = Math.min(1f, ORIGIN_FOLLOW_SMOOTH * delta);
        origin.setX(origin.getX() + (desiredOriginX - origin.getX()) * followFactor);

        float leftBound = origin.getX() - PATROL_WIDTH * 0.5f;
        float rightBound = origin.getX() + PATROL_WIDTH * 0.5f;

        float newX = getPosition().getX() + patrolDirection * PATROL_SPEED_X * delta;
        if (newX <= leftBound) {
            newX = leftBound;
            patrolDirection = 1;
        } else if (newX >= rightBound) {
            newX = rightBound;
            patrolDirection = -1;
        }

        getPosition().set(newX, hoverAltitude);

        if (target != null && target.getPosition() != null && cooldownTimer <= 0f) {
            Vector2 targetPos = target.getPosition();
            float dx = targetPos.getX() - newX;
            float horizontalDistance = Math.abs(dx);

            // Vérifier la distance horizontalement (pas la distance 2D qui pénalise les
            // grandes différences verticales)
            // Permettre le plongeon si le joueur est proche horizontalement ET en dessous
            // de hoverAltitude
            // IMPORTANT : Ne pas attaquer si le joueur est SOUS la surface (souterrain
            // profond)
            // Le joueur est visible si il est entre surfaceLevel - 100f (herbe) et
            // hoverAltitude
            // Cela inclut : herbe (surfaceLevel - 100f à surfaceLevel), tuyau (minAltitude
            // = surfaceLevel + 48f)
            boolean horizontalClose = horizontalDistance <= DIVE_TRIGGER_DISTANCE;
            boolean belowHover = targetPos.getY() < hoverAltitude + 150f;
            boolean isVisible = targetPos.getY() >= surfaceLevel - 100f; // Le joueur est visible si il est à la surface
                                                                         // (herbe ou tuyau) ou proche
            boolean canDive = horizontalClose && belowHover && isVisible;

            if (canDive) {
                // SIMPLE : Si le joueur est entre surfaceLevel - 100f et minAltitude, c'est
                // qu'il est sur l'herbe
                // On vise EXACTEMENT surfaceLevel (niveau de l'herbe)
                // Sinon, on vise minAltitude (niveau du tuyau)
                float playerY = targetPos.getY();
                Gdx.app.log("Pie",
                        "CHECKING DIVE - player.y=" + playerY + ", surfaceLevel=" + surfaceLevel + ", minAltitude="
                                + minAltitude + ", condition="
                                + (playerY >= surfaceLevel - 100f && playerY < minAltitude));

                if (playerY >= surfaceLevel - 100f && playerY < minAltitude) {
                    // Joueur sur l'herbe, viser EXACTEMENT surfaceLevel pour pouvoir le toucher
                    diveTarget.set(targetPos.getX(), surfaceLevel);
                    Gdx.app.log("Pie", "TARGETING GRASS - player.y=" + playerY + ", diveTarget SET TO surfaceLevel: "
                            + surfaceLevel);
                } else {
                    // Joueur sur tuyau/sous-sol ou au-dessus, viser minAltitude
                    diveTarget.set(targetPos.getX(), minAltitude);
                    Gdx.app.log("Pie", "TARGETING TUNNEL - player.y=" + playerY + ", diveTarget SET TO minAltitude: "
                            + minAltitude);
                }
                Gdx.app.log("Pie", "DIVE START - diveTarget.y=" + diveTarget.getY() + ", player.y=" + playerY
                        + ", surfaceLevel=" + surfaceLevel + ", minAltitude=" + minAltitude);
                cooldownTimer = DIVE_COOLDOWN;
                diveAnimation.reset();
                state = State.DIVING;
                timeAtGrassLevel = 0f; // Réinitialiser le timer quand on commence un nouveau plongeon
                return;
            }
        }
    }

    private void updateDive(float delta) {
        Vector2 pos = getPosition();
        float dx = diveTarget.getX() - pos.getX();
        float dy = diveTarget.getY() - pos.getY();
        float distanceSquared = dx * dx + dy * dy;
        float impactRadiusSquared = DIVE_IMPACT_RADIUS * DIVE_IMPACT_RADIUS;

        boolean targetingGrass = diveTarget.getY() < minAltitude;

        // Si on vise l'herbe, continuer la descente jusqu'à atteindre vraiment
        // surfaceLevel
        // Ne pas s'arrêter trop tôt avec le rayon d'impact
        if (targetingGrass && pos.getY() > diveTarget.getY() + 5f) {
            // On est encore trop haut, continuer à descendre
            float distance = (float) Math.sqrt(distanceSquared);
            float step = DIVE_SPEED * delta;
            if (step > distance) {
                step = distance;
            }
            float nx = dx / distance;
            float ny = dy / distance;
            pos.add(nx * step, ny * step);

            // Forcer la descente verticale si on est très proche de surfaceLevel
            // horizontalement
            if (Math.abs(dx) < 20f && pos.getY() > diveTarget.getY() + 5f) {
                // Descendre directement vers surfaceLevel
                float verticalStep = DIVE_SPEED * delta;
                float remainingDistance = pos.getY() - diveTarget.getY();
                if (verticalStep > remainingDistance) {
                    pos.setY(diveTarget.getY());
                } else {
                    pos.setY(pos.getY() - verticalStep);
                }
            }
        } else if (!targetingGrass && distanceSquared <= impactRadiusSquared) {
            // On vise le tuyau, utiliser le rayon d'impact normal
            applyDiveImpact();
            return;
        } else if (targetingGrass && pos.getY() <= diveTarget.getY() + 5f) {
            // On a atteint l'herbe, incrémenter le timer
            timeAtGrassLevel += delta;

            // Si on reste trop longtemps au niveau de l'herbe (plus de 2 secondes), forcer
            // la récupération
            if (timeAtGrassLevel > 2f) {
                state = State.RECOVERING;
                timeAtGrassLevel = 0f;
                return;
            }

            // Vérifier l'impact avec le joueur
            if (target != null && target.getPosition() != null) {
                Vector2 targetPos = target.getPosition();
                float playerDx = targetPos.getX() - pos.getX();
                float playerDy = targetPos.getY() - pos.getY();
                float playerDistanceSquared = playerDx * playerDx + playerDy * playerDy;
                if (playerDistanceSquared <= impactRadiusSquared) {
                    applyDiveImpact();
                    timeAtGrassLevel = 0f;
                    return;
                }
            }
            // Sinon, continuer à se déplacer horizontalement vers le joueur si on est déjà
            // au niveau de l'herbe
            float distance = (float) Math.sqrt(distanceSquared);
            if (distance > 2f) {
                float step = DIVE_SPEED * delta;
                if (step > distance) {
                    step = distance;
                }
                float nx = dx / distance;
                pos.add(nx * step, 0);
            } else {
                // On a atteint le diveTarget, même si on n'a pas touché le joueur, on passe en
                // récupération
                state = State.RECOVERING;
                timeAtGrassLevel = 0f;
                return;
            }
        } else {
            // Cas normal (pas encore assez proche ou pas au niveau de l'herbe)
            // Réinitialiser le timer car on n'est pas encore au niveau de l'herbe
            timeAtGrassLevel = 0f;

            float distance = (float) Math.sqrt(distanceSquared);
            float step = DIVE_SPEED * delta;
            if (step > distance) {
                step = distance;
            }
            float nx = dx / distance;
            float ny = dy / distance;
            pos.add(nx * step, ny * step);
        }

        // NE JAMAIS bloquer à minAltitude si on vise l'herbe
        if (!targetingGrass) {
            // On vise le tuyau (diveTarget = minAltitude) - bloquer à minAltitude
            if (pos.getY() < minAltitude) {
                pos.setY(minAltitude);
            }
        }
    }

    private void updateRecover(float delta) {
        float desiredOriginX = origin.getX();
        if (target != null && target.getPosition() != null) {
            desiredOriginX = target.getPosition().getX();
        }
        float followFactor = Math.min(1f, ORIGIN_FOLLOW_SMOOTH * delta);
        origin.setX(origin.getX() + (desiredOriginX - origin.getX()) * followFactor);

        Vector2 pos = getPosition();
        Vector2 targetPos = new Vector2(origin.getX(), hoverAltitude);

        // Calculer la direction vers la position cible
        float dx = targetPos.getX() - pos.getX();
        float dy = targetPos.getY() - pos.getY();
        float distanceSquared = dx * dx + dy * dy;

        if (distanceSquared < 36f) { // 6f * 6f - très proche
            pos.set(targetPos.getX(), targetPos.getY());
            state = State.PATROL;
            flyAnimation.reset();
            timeAtGrassLevel = 0f; // Réinitialiser le timer
            return;
        }

        // Se déplacer vers la position cible
        float distance = (float) Math.sqrt(distanceSquared);
        float step = RECOVER_SPEED * delta;
        if (step > distance) {
            step = distance;
        }
        float nx = dx / distance;
        float ny = dy / distance;
        pos.add(nx * step, ny * step);

        // IMPORTANT : Ne PAS bloquer à minAltitude pendant la récupération
        // La pie peut être en dessous de minAltitude (après avoir plongé jusqu'à
        // surfaceLevel)
        // et doit pouvoir remonter librement vers hoverAltitude
        // Le blocage à minAltitude n'est nécessaire que pendant PATROL et DIVING
        // (tuyaux)
    }

    private void applyDiveImpact() {
        if (target != null && target.getWorm() != null && target.getWorm().getCurrent() != null) {
            target.getWorm().getCurrent().takeDamage(DIVE_DAMAGE);
        }
        state = State.RECOVERING;
    }

    private void updateSprite(float delta) {
        TextureRegion frame = (state == State.DIVING ? diveAnimation : flyAnimation).getKeyFrame(delta);
        sprite.setRegion(frame);

        // IMPORTANT : Ne PAS modifier la position dans updateSprite() si on est en
        // DIVING
        // La position est gérée dans updateDive()
        // Juste mettre à jour le sprite à la position actuelle
        sprite.setPosition(getPosition());

        float currentX = getPosition().getX();
        // Ne PAS bloquer la position ici - c'est géré dans updateDive()
        // updateSprite() ne fait que mettre à jour le rendu
        boolean shouldFaceLeft = currentX < lastX;
        if (shouldFaceLeft != sprite.isFlipX()) {
            sprite.flip(true, false);
        }
        lastX = currentX;
    }

    private TextureRegion[] sliceRow(TextureRegion[][] regions, int row, int startColumn, int frameCount) {
        int maxColumns = Math.min(frameCount, SHEET_COLUMNS - startColumn);
        TextureRegion[] frames = new TextureRegion[maxColumns];
        for (int i = 0; i < maxColumns; i++) {
            frames[i] = regions[row][startColumn + i];
        }
        return frames;
    }

    @Override
    public void render(SpriteBatch batch) {
        draw(batch);
    }
}
