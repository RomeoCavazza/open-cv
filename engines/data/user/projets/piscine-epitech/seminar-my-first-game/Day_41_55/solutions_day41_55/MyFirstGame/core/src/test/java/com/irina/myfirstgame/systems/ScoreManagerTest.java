package com.irina.myfirstgame.systems;

import com.badlogic.gdx.Application;
import com.badlogic.gdx.ApplicationLogger;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Preferences;
import org.junit.jupiter.api.*;
import static org.assertj.core.api.Assertions.assertThat;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;
import java.lang.reflect.Field;

class ScoreManagerTest {
    private static Application mockApplication;
    private static Preferences mockPreferences;
    private static Application originalApp;
    private ScoreManager scoreManager;
    
    @BeforeAll
    static void initGdx() throws Exception {
        try {
            Field appField = Gdx.class.getDeclaredField("app");
            appField.setAccessible(true);
            originalApp = (Application) appField.get(null);
        } catch (Exception e) {
            originalApp = null;
        }
        
        mockApplication = mock(Application.class);
        mockPreferences = mock(Preferences.class);
        ApplicationLogger mockLogger = mock(ApplicationLogger.class);
        
        when(mockApplication.getApplicationLogger()).thenReturn(mockLogger);
        when(mockApplication.getPreferences(anyString())).thenReturn(mockPreferences);
        when(mockPreferences.getInteger(anyString(), anyInt())).thenReturn(0);
        
        Field appField = Gdx.class.getDeclaredField("app");
        appField.setAccessible(true);
        appField.set(null, mockApplication);
    }
    
    @BeforeEach
    void setUp() {
        scoreManager = new ScoreManager();
    }
    
    @AfterAll
    static void disposeGdx() throws Exception {
        Field appField = Gdx.class.getDeclaredField("app");
        appField.setAccessible(true);
        appField.set(null, originalApp);
    }
    
    @Test
    @DisplayName("Test 10: Ajout de points, cumul, ignore valeurs négatives, reset")
    void addPoints_ShouldIncreaseScore_AndIgnoreNegative() {
        scoreManager.addPoints(50);
        assertThat(scoreManager.getCurrentScore()).isEqualTo(50);
        
        scoreManager.addPoints(30);
        assertThat(scoreManager.getCurrentScore()).isEqualTo(80);
        
        scoreManager.addPoints(-10);
        assertThat(scoreManager.getCurrentScore()).isEqualTo(80);
        
        scoreManager.resetScore();
        assertThat(scoreManager.getCurrentScore()).isEqualTo(0);
        
        scoreManager.addPoints(25);
        assertThat(scoreManager.getCurrentScore()).isEqualTo(25);
    }
}

