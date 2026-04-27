package com.irina.myfirstgame.core.ui;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.scenes.scene2d.ui.CheckBox;
import com.badlogic.gdx.scenes.scene2d.ui.Label;
import com.badlogic.gdx.scenes.scene2d.ui.Skin;
import com.badlogic.gdx.scenes.scene2d.ui.Slider;
import com.badlogic.gdx.scenes.scene2d.ui.TextButton;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;

/**
 * Fabrique centralisée pour harmoniser l'apparence des différents menus.
 */
public final class UiSkinFactory {

    private UiSkinFactory() {}

    public static Skin createDefaultSkin() {
        Skin skin = new Skin();
        BitmapFont font = new BitmapFont();
        BitmapFont titleFont = new BitmapFont();
        titleFont.getData().setScale(1.4f);

        Pixmap pixmap = new Pixmap(1, 1, Pixmap.Format.RGBA8888);
        pixmap.setColor(Color.WHITE);
        pixmap.fill();
        Texture texture = new Texture(pixmap);
        pixmap.dispose();

        skin.add("white", texture);
        skin.add("default-font", font);
        skin.add("title-font", titleFont);

        Color lightGray = Color.valueOf("d9d9d9");
        Color mediumGray = Color.valueOf("9c9c9c");
        Color darkGray = Color.valueOf("2a2a2a");
        Color accentGray = Color.valueOf("6f6f6f");

        Label.LabelStyle labelStyle = new Label.LabelStyle(font, lightGray);
        Label.LabelStyle titleStyle = new Label.LabelStyle(titleFont, Color.valueOf("f4f4f4"));
        Label.LabelStyle errorStyle = new Label.LabelStyle(font, Color.valueOf("ff6f6f"));
        Label.LabelStyle feedbackStyle = new Label.LabelStyle(font, Color.valueOf("b5ffb8"));

        skin.add("default", labelStyle);
        skin.add("title", titleStyle);
        skin.add("error", errorStyle);
        skin.add("feedback", feedbackStyle);

        TextButton.TextButtonStyle buttonStyle = new TextButton.TextButtonStyle();
        buttonStyle.font = font;
        buttonStyle.up = skin.newDrawable("white", accentGray);
        buttonStyle.down = skin.newDrawable("white", mediumGray);
        buttonStyle.over = skin.newDrawable("white", Color.valueOf("8a8a8a"));
        skin.add("default", buttonStyle);

        TextField.TextFieldStyle textFieldStyle = new TextField.TextFieldStyle();
        textFieldStyle.font = font;
        textFieldStyle.fontColor = lightGray;
        textFieldStyle.cursor = skin.newDrawable("white", Color.WHITE);
        textFieldStyle.selection = skin.newDrawable("white", Color.valueOf("bfbfbf"));
        textFieldStyle.background = skin.newDrawable("white", new Color(0.12f, 0.12f, 0.12f, 0.9f));
        skin.add("default", textFieldStyle);

        CheckBox.CheckBoxStyle checkBoxStyle = new CheckBox.CheckBoxStyle();
        checkBoxStyle.checkboxOff = skin.newDrawable("white", darkGray);
        checkBoxStyle.checkboxOn = skin.newDrawable("white", mediumGray);
        checkBoxStyle.font = font;
        skin.add("default", checkBoxStyle);

        Slider.SliderStyle sliderStyle = new Slider.SliderStyle();
        sliderStyle.background = skin.newDrawable("white", darkGray.cpy().mul(1f, 1f, 1f, 0.6f));
        sliderStyle.knob = skin.newDrawable("white", mediumGray);
        sliderStyle.knobOver = skin.newDrawable("white", accentGray);
        skin.add("default-horizontal", sliderStyle);

        return skin;
    }
}

