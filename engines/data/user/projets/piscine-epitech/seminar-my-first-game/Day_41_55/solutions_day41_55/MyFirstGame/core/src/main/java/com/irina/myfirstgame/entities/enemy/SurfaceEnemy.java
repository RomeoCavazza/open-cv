package com.irina.myfirstgame.entities.enemy;

import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.Animation.PlayMode;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.irina.myfirstgame.core.Assets;
import com.irina.myfirstgame.entities.Player;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.systems.Animation;
import com.irina.myfirstgame.systems.Path;
import com.irina.myfirstgame.systems.Pathfinding;
import com.irina.myfirstgame.systems.Sprite;
import com.irina.myfirstgame.systems.SurfaceMovementHelper;
import com.irina.myfirstgame.world.Map;

/**
 * Classe abstraite pour les ennemis qui se déplacent à la surface et dans les tunnels.
 * Centralise toute la logique commune de mouvement, pathfinding et collision.
 */
public abstract class SurfaceEnemy extends Enemy {

    public static final int SHEET_COLUMNS = 3;
    public static final int SHEET_ROWS = 4;
    protected static final float STUCK_THRESHOLD = 0.15f;
    protected static final float UNSTUCK_DISTANCE_SMALL = 8f;
    protected static final float UNSTUCK_DISTANCE_MEDIUM = 16f;
    protected static final float UNSTUCK_DISTANCE_LARGE = 32f;

    protected final Player target;
    protected final Map map;
    protected final String collisionLayer;
    protected final Pathfinding pathfinding;
    protected final float surfaceLevel;

    protected final Animation animation;
    protected final Sprite sprite;

    protected Path currentPath = new Path();
    protected float repathTimer = 0f;
    protected float attackTimer = 0f;
    
    protected Vector2 lastPosition = new Vector2();
    protected float stuckTimer = 0f;
    private final Vector2 tempTargetPos = new Vector2();

    public SurfaceEnemy(Vector2 spawnPosition, Player target, float surfaceLevel, Map map, String collisionLayer, Animation animation, Sprite sprite) {
        super(spawnPosition.getX(), spawnPosition.getY());
        this.target = target;
        this.map = map;
        this.collisionLayer = collisionLayer;
        this.pathfinding = new Pathfinding(map, collisionLayer);
        this.surfaceLevel = surfaceLevel;
        this.animation = animation;
        this.sprite = sprite;
        this.lastPosition.set(spawnPosition.getX(), spawnPosition.getY());
    }

    // Méthodes abstraites à implémenter par les sous-classes
    protected abstract float getMoveSpeed();
    protected abstract float getAttackRange();
    protected abstract float getAttackCooldown();
    protected abstract int getAttackDamage();
    protected abstract float getRepathInterval();
    protected abstract String getTextureName();
    protected abstract float getAnimationFPS();
    
    // Méthode par défaut pour l'attaque (peut être surchargée)
    protected void performAttack(Player target) {
        if (target.getWorm() != null && target.getWorm().getCurrent() != null) {
            target.getWorm().getCurrent().takeDamage(getAttackDamage());
        }
    }
    
    // Méthodes statiques helper pour créer animation et sprite
    protected static Animation createAnimation(Assets assets, String textureName, float fps) {
        Texture texture = assets.getTexture(textureName);
        int frameWidth = texture.getWidth() / SHEET_COLUMNS;
        int frameHeight = texture.getHeight() / SHEET_ROWS;
        TextureRegion[][] regions = TextureRegion.split(texture, frameWidth, frameHeight);
        
        TextureRegion[] frames = new TextureRegion[SHEET_COLUMNS];
        for (int i = 0; i < SHEET_COLUMNS; i++) {
            frames[i] = regions[1][i];
        }
        
        Animation animation = new Animation(1f / fps, frames);
        animation.setPlayMode(PlayMode.LOOP);
        return animation;
    }
    
    protected static Sprite createSprite(Assets assets, String textureName) {
        Texture texture = assets.getTexture(textureName);
        int frameWidth = texture.getWidth() / SHEET_COLUMNS;
        int frameHeight = texture.getHeight() / SHEET_ROWS;
        TextureRegion[][] regions = TextureRegion.split(texture, frameWidth, frameHeight);
        
        TextureRegion initialFrame = regions[1][0];
        Sprite sprite = new Sprite(initialFrame);
        sprite.setOriginCenter();
        return sprite;
    }
    
    protected static void setupHitbox(Assets assets, String textureName, SurfaceEnemy enemy, float scale) {
        Texture texture = assets.getTexture(textureName);
        int frameWidth = texture.getWidth() / SHEET_COLUMNS;
        int frameHeight = texture.getHeight() / SHEET_ROWS;
        
        float hitboxWidth = frameWidth * scale;
        float hitboxHeight = frameHeight * scale;
        enemy.setWidth(hitboxWidth);
        enemy.setHeight(hitboxHeight);
    }

    @Override
    public void update(float delta) {
        if (attackTimer > 0f) {
            attackTimer -= delta;
        }
        repathTimer -= delta;

        Vector2 currentPos = getPosition();
        float dx = currentPos.getX() - lastPosition.getX();
        float dy = currentPos.getY() - lastPosition.getY();
        float distanceSquared = dx * dx + dy * dy;
        
        if (distanceSquared < 0.25f) { // 0.5f * 0.5f
            stuckTimer += delta;
        } else {
            stuckTimer = 0f;
        }
        
        lastPosition.set(currentPos);
        
        if (stuckTimer > STUCK_THRESHOLD) {
            tryUnstuck(delta);
            stuckTimer = 0f;
        } else {
            updatePath();
            moveAlongPath(delta);
        }

        handleAttack();
        updateSprite(delta);
    }

    protected float getGrassBottom() {
        return SurfaceMovementHelper.getGrassBottom(surfaceLevel, map);
    }
    
    protected boolean isInPipe(float x, float y) {
        return SurfaceMovementHelper.isInPipe(x, y, getWidth(), getHeight(), surfaceLevel, map, collisionLayer);
    }
    
    protected boolean isAtSurface() {
        Vector2 pos = getPosition();
        return SurfaceMovementHelper.isAtSurface(pos.getY(), getHeight(), surfaceLevel, map);
    }

    protected void updatePath() {
        if (target == null || target.getPosition() == null) {
            return;
        }
        if (!currentPath.isEmpty() && repathTimer > 0f) {
            return;
        }
        
        Vector2 targetPos = target.getPosition();
        if (isAtSurface()) {
            float halfHeight = getHeight() * 0.5f;
            float groundY = getGrassBottom() + halfHeight;
            tempTargetPos.set(targetPos.getX(), groundY);
            targetPos = tempTargetPos;
        }
        
        currentPath = pathfinding.compute(getPosition(), targetPos);
        currentPath.next();
        repathTimer = getRepathInterval();
    }

    protected void moveAlongPath(float delta) {
        if (currentPath == null || currentPath.isEmpty()) {
            return;
        }
        Vector2 waypoint = currentPath.peek();
        if (waypoint == null) {
            return;
        }
        Vector2 pos = getPosition();
        
        if (isInCorner(pos.getX(), pos.getY())) {
            Vector2 centerDir = findTunnelCenter(pos.getX(), pos.getY());
            if (centerDir != null) {
                float escapeSpeed = getMoveSpeed() * 1.5f;
                float escapeX = pos.getX() + centerDir.getX() * escapeSpeed * delta;
                float escapeY = pos.getY() + centerDir.getY() * escapeSpeed * delta;
                
                escapeY = SurfaceMovementHelper.adjustYForSurface(pos.getX(), escapeY, getWidth(), getHeight(),
                                                                 surfaceLevel, map, collisionLayer);
                
                if (!isColliding(escapeX, escapeY)) {
                    getPosition().set(escapeX, escapeY);
                    currentPath.clear();
                    repathTimer = 0f;
                    return;
                }
            }
        }
        
        if (isAtSurface()) {
            SurfaceMovementHelper.adjustWaypointForSurface(waypoint, pos.getX(), pos.getY(),
                                                          getWidth(), getHeight(),
                                                          surfaceLevel, map, collisionLayer);
        }
        
        float dx = waypoint.getX() - pos.getX();
        float dy = waypoint.getY() - pos.getY();
        
        if (isAtSurface()) {
            dy = SurfaceMovementHelper.adjustVerticalMovement(dy, pos.getX(), pos.getY(), 
                                                             getWidth(), getHeight(), 
                                                             surfaceLevel, map, collisionLayer);
        }
        
        float distanceSquared = dx * dx + dy * dy;
        float maxStep = getMoveSpeed() * delta;
        float maxStepSquared = maxStep * maxStep;

        if (distanceSquared <= maxStepSquared) {
            getPosition().set(waypoint);
            currentPath.next();
        } else {
            float distance = (float) Math.sqrt(distanceSquared);
            float stepX = (dx / distance) * maxStep;
            float stepY = (dy / distance) * maxStep;
            float nextX = pos.getX() + stepX;
            float nextY = pos.getY() + stepY;
            
            nextY = SurfaceMovementHelper.adjustYForSurface(nextX, nextY, getWidth(), getHeight(),
                                                           surfaceLevel, map, collisionLayer);
            
            if (isInCorner(nextX, nextY)) {
                Vector2 centerDir = findTunnelCenter(nextX, nextY);
                if (centerDir != null) {
                    float avoidX = nextX + centerDir.getX() * 16f;
                    float avoidY = nextY + centerDir.getY() * 16f;
                    
                    avoidY = SurfaceMovementHelper.adjustYForSurface(avoidX, avoidY, getWidth(), getHeight(),
                                                                    surfaceLevel, map, collisionLayer);
                    
                    if (!isColliding(avoidX, avoidY)) {
                        getPosition().set(avoidX, avoidY);
                        return;
                    }
                }
            }
            
            if (!isColliding(nextX, nextY)) {
                getPosition().set(nextX, nextY);
            } else {
                boolean moved = false;
                
                if (Math.abs(stepX) > 0.1f && !isColliding(nextX, pos.getY())) {
                    getPosition().set(nextX, pos.getY());
                    moved = true;
                } else if (Math.abs(stepY) > 0.1f && !isColliding(pos.getX(), nextY)) {
                    boolean canMoveVertically = !isAtSurface();
                    if (isAtSurface()) {
                        float grassBottom = getGrassBottom();
                        canMoveVertically = isInPipe(pos.getX(), grassBottom);
                    }
                    if (canMoveVertically) {
                        getPosition().set(pos.getX(), nextY);
                        moved = true;
                    }
                } else if (Math.abs(stepX) > 0.1f) {
                    float partialX = pos.getX() + stepX * 0.5f;
                    float partialY = pos.getY();
                    partialY = SurfaceMovementHelper.adjustYForSurface(partialX, partialY, getWidth(), getHeight(),
                                                                      surfaceLevel, map, collisionLayer);
                    if (!isColliding(partialX, partialY)) {
                        getPosition().set(partialX, partialY);
                        moved = true;
                    }
                } else if (!isAtSurface() && Math.abs(stepY) > 0.1f) {
                    float partialY = pos.getY() + stepY * 0.5f;
                    if (!isColliding(pos.getX(), partialY)) {
                        getPosition().set(pos.getX(), partialY);
                        moved = true;
                    }
                }
                
                if (!moved) {
                    currentPath.clear();
                    repathTimer = 0f;
                }
            }
        }
    }
    
    protected boolean isInCorner(float x, float y) {
        if (map == null) return false;
        
        float reducedHalfWidth = getWidth() * 0.4f;
        float reducedHalfHeight = getHeight() * 0.4f;
        float checkY = y - reducedHalfHeight + 8f;
        
        boolean leftWall = map.isSolidAt(x - reducedHalfWidth - 1f, checkY, collisionLayer);
        boolean rightWall = map.isSolidAt(x + reducedHalfWidth + 1f, checkY, collisionLayer);
        boolean topWall = map.isSolidAt(x, y + reducedHalfHeight + 1f, collisionLayer);
        boolean bottomWall = map.isSolidAt(x, y - reducedHalfHeight - 1f, collisionLayer);
        
        boolean topLeftCorner = topWall && leftWall;
        boolean topRightCorner = topWall && rightWall;
        boolean bottomLeftCorner = bottomWall && leftWall;
        boolean bottomRightCorner = bottomWall && rightWall;
        
        return topLeftCorner || topRightCorner || bottomLeftCorner || bottomRightCorner;
    }
    
    protected Vector2 findTunnelCenter(float x, float y) {
        if (map == null) return null;
        
        float halfWidth = getWidth() * 0.4f;
        float halfHeight = getHeight() * 0.4f;
        float checkY = y - halfHeight + 8f;
        
        boolean leftWall = map.isSolidAt(x - halfWidth - 1f, checkY, collisionLayer);
        boolean rightWall = map.isSolidAt(x + halfWidth + 1f, checkY, collisionLayer);
        boolean topWall = map.isSolidAt(x, y + halfHeight + 1f, collisionLayer);
        boolean bottomWall = map.isSolidAt(x, y - halfHeight - 1f, collisionLayer);
        
        float dirX = 0f;
        float dirY = 0f;
        
        if (leftWall) dirX += 1f;
        if (rightWall) dirX -= 1f;
        if (topWall) dirY -= 1f;
        if (bottomWall) dirY += 1f;
        
        float lengthSquared = dirX * dirX + dirY * dirY;
        if (lengthSquared > 0.01f) { // 0.1f * 0.1f
            float length = (float) Math.sqrt(lengthSquared);
            return new Vector2(dirX / length, dirY / length);
        }
        
        return null;
    }

    protected void handleAttack() {
        if (target == null || target.getPosition() == null || attackTimer > 0f) {
            return;
        }
        Vector2 playerPos = target.getPosition();
        Vector2 enemyPos = getPosition();
        float dx = playerPos.getX() - enemyPos.getX();
        float dy = playerPos.getY() - enemyPos.getY();
        float distanceSquared = dx * dx + dy * dy;

        if (distanceSquared <= getAttackRange() * getAttackRange()) {
            performAttack(target);
            attackTimer = getAttackCooldown();
        }
    }

    protected void updateSprite(float delta) {
        alignSprite();
        sprite.setRegion(animation.getKeyFrame(delta));

        if (target != null && target.getPosition() != null) {
            boolean movingLeft = target.getPosition().getX() < getPosition().getX();
            if (movingLeft && !sprite.isFlipX()) {
                sprite.flip(true, false);
            } else if (!movingLeft && sprite.isFlipX()) {
                sprite.flip(true, false);
            }
        } else if (sprite.isFlipX()) {
            sprite.flip(true, false);
        }
    }

    protected boolean isColliding(float proposedX, float proposedY) {
        if (map == null) {
            return false;
        }
        float halfHeight = getHeight() * 0.45f;
        float checkY = proposedY - halfHeight + 8f;
        
        boolean collisionX = map.isSolidAt(proposedX, checkY, collisionLayer);
        
        float topY = proposedY + halfHeight + 1f;
        float bottomY = proposedY - halfHeight - 1f;
        
        boolean collisionY = map.isSolidAt(proposedX, topY, collisionLayer)
            || map.isSolidAt(proposedX, bottomY, collisionLayer);
        
        return collisionX || collisionY;
    }

    protected void tryUnstuck(float delta) {
        Vector2 pos = getPosition();
        float halfWidth = getWidth() * 0.5f;
        float halfHeight = getHeight() * 0.5f;
        boolean atSurface = isAtSurface();
        
        float[] distances = {UNSTUCK_DISTANCE_SMALL, UNSTUCK_DISTANCE_MEDIUM, UNSTUCK_DISTANCE_LARGE};
        
        float[][] directions;
        if (atSurface) {
            directions = new float[][] {
                {0f, -1f},
                {-1f, 0f},
                {1f, 0f},
                {-0.707f, -0.707f},
                {0.707f, -0.707f}
            };
        } else {
            directions = new float[][] {
                {0f, 1f},
                {0f, -1f},
                {-1f, 0f},
                {1f, 0f},
                {-0.707f, 0.707f},
                {0.707f, 0.707f},
                {-0.707f, -0.707f},
                {0.707f, -0.707f}
            };
        }
        
        for (float[] dir : directions) {
            for (float dist : distances) {
                float testX = pos.getX() + dir[0] * dist;
                float testY = pos.getY() + dir[1] * dist;
                
                if (atSurface) {
                    testY = SurfaceMovementHelper.adjustYForSurface(testX, testY, getWidth(), getHeight(),
                                                                   surfaceLevel, map, collisionLayer);
                }
                
                if (!isColliding(testX, testY)) {
                    getPosition().set(testX, testY);
                    currentPath.clear();
                    repathTimer = 0f;
                    return;
                }
            }
        }
        
        float pushX = 0f;
        float pushY = 0f;
        
        float[] cornerOffsets = {
            -halfWidth, -halfHeight,
            halfWidth, -halfHeight,
            -halfWidth, halfHeight,
            halfWidth, halfHeight
        };
        
        for (int i = 0; i < cornerOffsets.length; i += 2) {
            float cornerX = pos.getX() + cornerOffsets[i];
            float cornerY = pos.getY() + cornerOffsets[i + 1];
            if (map.isSolidAt(cornerX, cornerY, collisionLayer)) {
                pushX += (pos.getX() - cornerX) * 0.5f;
                pushY += (pos.getY() - cornerY) * 0.5f;
            }
        }
        
        float pushLengthSquared = pushX * pushX + pushY * pushY;
        if (pushLengthSquared > 0.01f) { // 0.1f * 0.1f
            float pushLength = (float) Math.sqrt(pushLengthSquared);
            pushX = (pushX / pushLength) * UNSTUCK_DISTANCE_MEDIUM;
            pushY = (pushY / pushLength) * UNSTUCK_DISTANCE_MEDIUM;
            float testX = pos.getX() + pushX;
            float testY = pos.getY() + pushY;
            
            if (atSurface) {
                testY = adjustYForSurfacePosition(testX, testY, halfHeight);
            }
            
            if (!isColliding(testX, testY)) {
                getPosition().set(testX, testY);
                currentPath.clear();
                repathTimer = 0f;
                return;
            }
        }
        
        currentPath.clear();
        repathTimer = 0f;
        
        float randomAngle;
        if (atSurface) {
            randomAngle = (float) (Math.random() * Math.PI);
        } else {
            randomAngle = (float) (Math.random() * Math.PI * 2);
        }
        float shakeX = (float) Math.cos(randomAngle) * UNSTUCK_DISTANCE_SMALL;
        float shakeY = (float) Math.sin(randomAngle) * UNSTUCK_DISTANCE_SMALL;
        float testX = pos.getX() + shakeX;
        float testY = pos.getY() + shakeY;
        
        if (atSurface) {
            testY = adjustYForSurfacePosition(testX, testY, halfHeight);
        }
        
        if (!isColliding(testX, testY)) {
            getPosition().set(testX, testY);
        }
    }

    protected void alignSprite() {
        if (sprite != null) {
            sprite.setPosition(getPosition());
        }
    }

    private float adjustYForSurfacePosition(float x, float y, float halfHeight) {
        float grassBottom = surfaceLevel - map.getTileHeight();
        float testBottomY = y - halfHeight;
        float tolerance = map.getTileHeight() * 0.5f;
        boolean trulyOnGrass = testBottomY >= grassBottom - tolerance && testBottomY <= grassBottom + tolerance;
        boolean isOnPipe = isInPipe(x, y);
        
        if (!isOnPipe && trulyOnGrass) {
            return getGrassBottom() + halfHeight;
        }
        return y;
    }


    @Override
    public void render(SpriteBatch batch) {
        sprite.draw(batch);
    }
}

