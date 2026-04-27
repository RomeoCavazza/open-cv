package com.irina.myfirstgame.stats;

import org.junit.jupiter.api.*;
import static org.assertj.core.api.Assertions.assertThat;

class HungerTest {
    private Hunger hunger;
    
    @BeforeEach
    void setUp() {
        hunger = new Hunger();
        hunger.setMaxHunger(100);
    }
    
    @Test
    @DisplayName("Test 5: Faim ne descend pas en dessous de 0 et déclenche starvation")
    void decrease_ShouldNotGoBelowZero_AndTriggerStarvation() {
        hunger.decrease(150);
        assertThat(hunger.getCurrentHunger()).isEqualTo(0);
        assertThat(hunger.isStarving()).isTrue();
    }
    
    @Test
    @DisplayName("Test 6: Restauration de faim ne dépasse pas maxHunger")
    void increase_ShouldNotExceedMaxHunger() {
        hunger.increase(150);
        assertThat(hunger.getCurrentHunger()).isEqualTo(100);
    }
}



