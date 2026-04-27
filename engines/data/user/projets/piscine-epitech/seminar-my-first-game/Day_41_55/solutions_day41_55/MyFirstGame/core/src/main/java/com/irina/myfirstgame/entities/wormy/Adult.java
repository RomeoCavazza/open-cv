package com.irina.myfirstgame.entities.wormy;

import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.irina.myfirstgame.entities.valueobjects.Vector2;
import com.irina.myfirstgame.systems.Animation;
import com.irina.myfirstgame.systems.Sprite;

public class Adult extends Wormy {

    private Animation walkAnimation;
    private Animation upAnimation;
    private Animation turnAnimation;
    private Sprite sprite;
    private float speed = 120f;

    private boolean faceLeft;
    private boolean movingUp;
    private boolean movingDown;
    private boolean turningUp;
    private boolean turningDown;

    private float hungerTimer;
    private float healthTimer;

    public Adult() {
        super();
        getHealth().setMaxHealth(100);
        getHunger().setMaxHunger(100);
    }

    @Override
    public Sprite getSprite() {
        return sprite;
    }

    @Override
    public void setSprite(Sprite sprite) {
        this.sprite = sprite;
    }

    public void setWalkAnimation(Animation walkAnimation) {
        this.walkAnimation = walkAnimation;
    }

    public void setUpAnimation(Animation upAnimation) {
        this.upAnimation = upAnimation;
    }

    public void setTurnAnimation(Animation turnAnimation) {
        this.turnAnimation = turnAnimation;
    }

    public void setFaceLeft(boolean faceLeft) {
        this.faceLeft = faceLeft;
    }

    public void setMovingUp(boolean movingUp) {
        this.movingUp = movingUp;
    }

    public void setMovingDown(boolean movingDown) {
        this.movingDown = movingDown;
    }

    public void setTurningUp(boolean turning) {
        this.turningUp = turning;
    }

    public void setTurningDown(boolean turning) {
        this.turningDown = turning;
    }

    public boolean isMovingUp() {
        return movingUp;
    }

    public boolean isMovingDown() {
        return movingDown;
    }

    public float getSpeed() {
        return speed;
    }

    public void setSpeed(float speed) {
        this.speed = speed;
    }

    public void updateAnimation(float delta) {
        if (sprite == null) {
            return;
        }

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

    public void applyWobble(float wobbleTime) {
        if (sprite != null) {
            float amp = 0.03f;
            float freq = 5f;
            float scale = 1f + (float) Math.sin(wobbleTime * Math.PI * 2f * freq) * amp;
            sprite.setScale(scale, 1f - (scale - 1f));
            updateSpritePosition();
        }
    }

    private void updateSpritePosition() {
        if (sprite != null) {
            Vector2 pos = getPosition();
            sprite.setPosition(pos.getX(), pos.getY());
        }
    }

    @Override
    public void update(float delta) {
        // Mise à jour de la position basée sur la vélocité de l'Entity parent
        // Identique à Baby pour avoir le même comportement
        Vector2 pos = getPosition();
        Vector2 vel = getVelocity();
        pos.add(vel.getX() * delta, vel.getY() * delta);

        hungerTimer += delta;
        healthTimer += delta;
        // decrease hunger every 5 seconds (faster decay)
        if (hungerTimer >= 5f) {
            getHunger().decrease(15);
            hungerTimer = 0f;
        }
        // on check la vie toutes les 1 secondes
        if (healthTimer >= 1f) {
            int currentHunger = getHunger().getCurrentHunger();
            // si hunger est à 0, on perd de la vie (slower death)
            if (currentHunger <= 0) {
                getHealth().takeDamage(5);
            } else if (currentHunger >= 50) {
                getHealth().heal(25);
            }
            healthTimer = 0f;
        }

        updateAnimation(delta);
    }
}
