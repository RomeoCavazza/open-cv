package com.irina.myfirstgame.entities.wormy;

import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.irina.myfirstgame.systems.Animation;
import com.irina.myfirstgame.systems.Sprite;

public class Baby extends Wormy {
    private Animation walkAnimation;
    private Animation upAnimation;
    private Animation turnAnimation;
    private Sprite sprite;
    private float speed;
    private boolean faceLeft;
    private boolean movingUp;
    private boolean movingDown;
    private boolean turningUp;
    private boolean turningDown;
    private float hungerTimer;
    private float healthTimer;

    public Baby() {
        super();
        this.speed = 80f;
        this.faceLeft = false;

        getHealth().setMaxHealth(50);
        getHunger().setMaxHunger(50);
    }

    public void crawl() {
    }

    @Override
    public Sprite getSprite() {
        return this.sprite;
    }

    public void setWalkAnimation(Animation animation) {
        this.walkAnimation = animation;
    }

    public void setUpAnimation(Animation animation) {
        this.upAnimation = animation;
    }

    public void setTurnAnimation(Animation animation) {
        this.turnAnimation = animation;
    }

    public void setSprite(Sprite sprite) {
        this.sprite = sprite;
    }

    public void setFaceLeft(boolean faceLeft) {
        this.faceLeft = faceLeft;
    }

    public boolean isFaceLeft() {
        return faceLeft;
    }

    public void setMovingUp(boolean up) {
        this.movingUp = up;
    }

    public boolean isMovingUp() {
        return movingUp;
    }

    public void setMovingDown(boolean down) {
        this.movingDown = down;
    }

    public boolean isMovingDown() {
        return movingDown;
    }

    public void setTurningUp(boolean turning) {
        this.turningUp = turning;
    }

    public void setTurningDown(boolean turning) {
        this.turningDown = turning;
    }

    public float getSpeed() {
        return speed;
    }

    public void updateAnimation(float delta) {
        if (sprite != null) {
            float velX = getVelocity().getX();
            float velY = getVelocity().getY();
            float absVelX = Math.abs(velX);
            float absVelY = Math.abs(velY);

            boolean horizontalMove = absVelX > 0.1f;
            boolean verticalMove = absVelY > 0.1f || movingUp || movingDown;
            boolean isTurning = turningUp || turningDown;

            if (!horizontalMove && !verticalMove && !isTurning) {
                if (walkAnimation != null) {
                    walkAnimation.pause();
                    walkAnimation.setStateTime(0f);
                    TextureRegion idleFrame = walkAnimation.getKeyFrame(0f);
                    if (idleFrame != null) {
                        sprite.setRegion(idleFrame);
                    }
                }
                if (upAnimation != null) {
                    upAnimation.pause();
                    upAnimation.setStateTime(0f);
                }
                if (turnAnimation != null) {
                    turnAnimation.pause();
                    turnAnimation.setStateTime(0f);
                }
            } else if (isTurning) {
                if (turnAnimation != null) {
                    turnAnimation.play();
                    sprite.setRegion(turnAnimation.getKeyFrame(delta));
                    if (turnAnimation.isAnimationFinished(delta)) {
                        turningUp = false;
                        turningDown = false;
                        turnAnimation.reset();
                        turnAnimation.pause();
                    }
                }
            } else if (verticalMove) {
                if (upAnimation != null) {
                    upAnimation.play();
                    sprite.setRegion(upAnimation.getKeyFrame(delta));
                }
                if (walkAnimation != null) {
                    walkAnimation.pause();
                }
            } else {
                if (walkAnimation != null) {
                    walkAnimation.play();
                    sprite.setRegion(walkAnimation.getKeyFrame(delta));
                }
                if (upAnimation != null) {
                    upAnimation.pause();
                    upAnimation.setStateTime(0f);
                }
            }

            boolean flipY = movingDown || turningDown;
            if (sprite.isFlipY() != flipY) {
                sprite.flip(false, true);
            }

            boolean onHorizontal = !(movingUp || movingDown || turningUp || turningDown);
            if (onHorizontal) {
                boolean shouldFlipX = !faceLeft;
                if (sprite.isFlipX() != shouldFlipX) {
                    sprite.flip(true, false);
                }
            } else if (sprite.isFlipX()) {
                sprite.flip(true, false);
            }

            updateSpritePosition();
        }
    }

    public void applyWobble(float wobbleTime) {
        if (sprite != null) {
            float amp = 0.05f;
            float freq = 6f;
            float scale = 1f + (float) Math.sin(wobbleTime * Math.PI * 2f * freq) * amp;
            sprite.setScale(scale, 1f - (scale - 1f));
            updateSpritePosition();
        }
    }

    private void updateSpritePosition() {
        if (sprite != null) {
            sprite.setPosition(getPosition());
        }
    }

    @Override
    public void update(float delta) {
        // Mise à jour de la position basée sur la vélocité de l'Entity parent
        com.irina.myfirstgame.entities.valueobjects.Vector2 pos = getPosition();
        com.irina.myfirstgame.entities.valueobjects.Vector2 vel = getVelocity();
        pos.add(vel.getX() * delta, vel.getY() * delta);
        // met à jour le timer pour health et hunger
        hungerTimer += delta;
        healthTimer += delta;
        // decrease hunger every 5 seconds (faster decay)
        if (hungerTimer >= 5f) {
            getHunger().decrease(10);
            hungerTimer = 0f;
        }
        // on check la vie toutes les 1 secondes
        if (healthTimer >= 1f) {
            int currentHunger = getHunger().getCurrentHunger();
            // si hunger est à 0, on perd de la vie (slower death)
            if (currentHunger <= 0) {
                getHealth().takeDamage(5);
            } else if (currentHunger >= 50) {
                getHealth().heal(15);
            }
            healthTimer = 0f;
        }

        updateAnimation(delta);
    }
}
