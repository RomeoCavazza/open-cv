# main.py
### IMPORTS & INIT ###
import sys
import pygame
from hangman_zombie import (
    init_game,
    apply_letter,
    apply_word,
    progress_bar,
    best_score_update,
)

pygame.init()
screen = pygame.display.set_mode((800, 600))
pygame.display.set_caption(" Hangman Zombie")
clock = pygame.time.Clock()

# Background safe-load (si le fichier manque, on colore juste le fond)
try:
    bg = pygame.image.load("background.jpg").convert()
except Exception:
    bg = None
font = pygame.font.SysFont(None, 28)

### UTILS (FRONT END) ###
def draw_text(txt, x, y, color=(255, 255, 255)):
    img = font.render(txt, True, color)
    screen.blit(img, (x, y))

def draw_gallows_and_body(state):
    # gibet + perso (statique pour l’instant)
    pygame.draw.line(screen, (0,0,0), (150, 90), (150, 550), 5)
    pygame.draw.line(screen, (0,0,0), (300, 90), (150, 90), 5)
    pygame.draw.line(screen, (0,0,0), (90, 550), (300, 550), 5)
    pygame.draw.line(screen, (0,0,0), (150, 150), (200, 90), 5)
    pygame.draw.line(screen, (0,0,0), (250, 90), (250, 140), 5)
    pygame.draw.circle(screen, (0,0,0), (270, 160), 30, 5)
    pygame.draw.line(screen, (0,0,0), (250, 180), (250, 300), 5)
    pygame.draw.line(screen, (0,0,0), (250, 200), (230, 270), 5)
    pygame.draw.line(screen, (0,0,0), (250, 200), (270, 270), 5)
    pygame.draw.line(screen, (0,0,0), (250, 300), (230, 420), 5)
    pygame.draw.line(screen, (0,0,0), (250, 300), (260, 420), 5)

### GAME SETUP ###
# Le moteur lit sys.argv[1] pour choisir le wordlist (ex: words.txt)
state = init_game()
current = ""      # ce que le joueur est en train de taper
message = ""      # message de best score (une fois fini)
running = True

### MAIN LOOP ###
while running:
    for event in pygame.event.get():
        # Quitter par croix fenêtre
        if event.type == pygame.QUIT:
            running = False

        # Clavier
        if event.type == pygame.KEYDOWN:
            # ESC = quitter en toute circonstance
            if event.key == pygame.K_ESCAPE:
                running = False
                continue

            if state["finished"]:
                # Partie finie : R = restart
                if event.key == pygame.K_r:
                    state = init_game()
                    current, message = "", ""
            else:
                # Saisie en cours
                if event.key == pygame.K_BACKSPACE:
                    current = current[:-1]
                elif event.key == pygame.K_RETURN:
                    guess = current.strip().upper()
                    if len(guess) == 1:
                        apply_letter(state, guess)
                    elif len(guess) > 1:
                        apply_word(state, guess)
                    current = ""
                    if state["finished"] and not message:
                        message = best_score_update(state, "best_scores.txt")
                else:
                    # On n'ajoute que des lettres A-Z
                    if event.unicode.isalpha():
                        current += event.unicode.upper()

    ### RENDERING LOOP ###
    if bg is not None:
        screen.blit(bg, (0, 0))
    else:
        screen.fill((25, 25, 25))

    draw_gallows_and_body(state)

    draw_text(f"WORD: {' '.join(state['mask'])}", 30, 30)
    draw_text(f"INPUT: {current or '_'}  (Enter to submit)", 30, 60)
    draw_text(f"POP: {state['population']}   ZOMBIES: {state['zombies']}", 30, 90)
    draw_text(f"ATTEMPTS: {state['attempts']}", 30, 120)
    if state.get("last_event"):
        draw_text(f"EVENT: {state['last_event']}", 30, 150)
    draw_text("BAR: [" + progress_bar(state, 10) + "]", 30, 180)

    if state["finished"]:
        draw_text(("YOU WIN!" if state["win"] else "GAME OVER!") + "  (R = restart, ESC = quit)", 30, 520)
        if message:
            draw_text(message, 30, 550)

    ### FRAMERATE & UPDATE ###
    pygame.display.flip()
    clock.tick(60)

pygame.quit()