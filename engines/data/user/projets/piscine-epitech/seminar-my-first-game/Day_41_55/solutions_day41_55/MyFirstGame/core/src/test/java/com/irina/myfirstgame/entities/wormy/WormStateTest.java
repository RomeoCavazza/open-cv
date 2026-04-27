package com.irina.myfirstgame.entities.wormy;

import com.badlogic.gdx.Application;
import com.badlogic.gdx.ApplicationLogger;
import com.badlogic.gdx.Gdx;
import org.junit.jupiter.api.*;
import static org.assertj.core.api.Assertions.assertThat;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;
import java.lang.reflect.Field;

class WormStateTest {
    private static Application mockApplication;
    private static Application originalApp;
    private WormState wormState;
    
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
        ApplicationLogger mockLogger = mock(ApplicationLogger.class);
        when(mockApplication.getApplicationLogger()).thenReturn(mockLogger);
        
        Field appField = Gdx.class.getDeclaredField("app");
        appField.setAccessible(true);
        appField.set(null, mockApplication);
    }
    
    @BeforeEach
    void setUp() {
        wormState = new WormState();
        Baby baby = new Baby();
        wormState.evolveTo(baby);
    }
    
    @AfterAll
    static void disposeGdx() throws Exception {
        Field appField = Gdx.class.getDeclaredField("app");
        appField.setAccessible(true);
        appField.set(null, originalApp);
    }
    
    @Test
    @DisplayName("Test 7: Enregistrement correct des 3 types d'aliments")
    void registerFood_ShouldTrackAllThreeFoods() {
        wormState.registerFood("burger");
        wormState.registerFood("frites");
        wormState.registerFood("soda");
        
        assertThat(wormState.hasAteBurger()).isTrue();
        assertThat(wormState.hasAteFrites()).isTrue();
        assertThat(wormState.hasAteSoda()).isTrue();
    }
    
    @Test
    @DisplayName("Test 8: Évolution Baby vers Adult quand les 3 aliments sont mangés")
    void checkEvolution_ShouldEvolveBabyToAdult_WhenAllFoodsEaten() {
        wormState.registerFood("burger");
        wormState.registerFood("frites");
        wormState.registerFood("soda");
        
        assertThat(wormState.getCurrent()).isInstanceOf(Adult.class);
        assertThat(wormState.hasEvolvedToAdult()).isTrue();
    }
    
    @Test
    @DisplayName("Test 9: Position et vélocité sont préservées lors de l'évolution")
    void evolveTo_ShouldPreservePosition() {
        Baby baby = new Baby();
        baby.setPosition(100, 200);
        baby.setVelocity(5, 3);
        wormState.evolveTo(baby);
        
        Wormy babyState = wormState.getCurrent();
        float savedX = babyState.getPosition().getX();
        float savedY = babyState.getPosition().getY();
        float savedVelX = babyState.getVelocity().getX();
        float savedVelY = babyState.getVelocity().getY();
        
        Adult adult = new Adult();
        wormState.evolveTo(adult);
        
        Wormy current = wormState.getCurrent();
        assertThat(current).isInstanceOf(Adult.class);
        assertThat(current).isSameAs(adult);
        
        assertThat(current.getPosition().getX()).isEqualTo(savedX);
        assertThat(current.getPosition().getY()).isEqualTo(savedY);
        assertThat(current.getVelocity().getX()).isEqualTo(savedVelX);
        assertThat(current.getVelocity().getY()).isEqualTo(savedVelY);
    }
}
