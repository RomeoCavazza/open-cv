package com.irina.myfirstgame.stats;

import org.junit.jupiter.api.*;
import static org.assertj.core.api.Assertions.assertThat;

class HealthTest {
    private Health health;
    
    @BeforeEach
    void setUp() {
        health = new Health();
        health.setMaxHealth(100);
    }
    
    @Test
    @DisplayName("Test 1: Dégâts réduisent correctement la santé")
    void takeDamage_ShouldReduceHealth() {
        health.takeDamage(30);
        assertThat(health.getCurrentHealth()).isEqualTo(70);
    }
    
    @Test
    @DisplayName("Test 2: Dégâts ne descendent pas en dessous de 0")
    void takeDamage_ShouldNotGoBelowZero() {
        health.takeDamage(150);
        assertThat(health.getCurrentHealth()).isEqualTo(0);
        assertThat(health.isDead()).isTrue();
    }
    
    @Test
    @DisplayName("Test 3: Soins augmentent correctement la santé")
    void heal_ShouldIncreaseHealth() {
        health.takeDamage(50);
        health.heal(20);
        assertThat(health.getCurrentHealth()).isEqualTo(70);
    }
    
    @Test
    @DisplayName("Test 4: Soins ne dépassent pas maxHealth")
    void heal_ShouldNotExceedMaxHealth() {
        health.heal(50);
        assertThat(health.getCurrentHealth()).isEqualTo(100);
    }
}



