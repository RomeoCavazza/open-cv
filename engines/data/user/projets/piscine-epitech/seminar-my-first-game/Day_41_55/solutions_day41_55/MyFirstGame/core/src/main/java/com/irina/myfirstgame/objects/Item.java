package com.irina.myfirstgame.objects;

import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.math.Rectangle;
import com.badlogic.gdx.math.Vector2;
import com.irina.myfirstgame.entities.wormy.Wormy;
import com.irina.myfirstgame.interfaces.Collectible;
import com.irina.myfirstgame.interfaces.Spawnable;

public abstract class Item extends GameObject implements Collectible, Spawnable {
    protected boolean collected = false;
    protected TextureRegion sprite;
    protected final Rectangle bounds;
    protected float width = 50f, height = 50f;

    protected Item() {
        super();
        this.bounds = new Rectangle(0, 0, width, height);
    }
    
    @Override
    public void spawn(Vector2 at) {
        this.position.set(
            at.x - width * 0.5f,
            at.y - height * 0.5f
        );

        this.collected = false;
        this.active = true;

        this.bounds.set(position.x, position.y, width, height);
        this.onSpawn();
    }

    @Override
    public void update(float delta) {
        if (!active) return;
        this.bounds.setPosition(position.x, position.y);
    }

    @Override
    public void render(SpriteBatch batch) {
        if (!active || collected || sprite == null) {
            return;
        }
        batch.draw(sprite, position.x, position.y, width, height);
    }

    public Rectangle getBounds() {
        return bounds;
    }

    public boolean isCollected() {
        return collected;
    }
    
    @Override
    public final void collect(Wormy by) {
        if (collected) return;
        collected = true;
        active = false;
        onCollect(by);
    }

    protected abstract void onCollect(Wormy by);
}

