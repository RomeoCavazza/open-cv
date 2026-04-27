import pygame, turtle

pygame.init()
surface = pygame.Surface((600, 600))
screen = pygame.display.set_mode((600, 600))
image = pygame.image.load("background.jpg")

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
    screen.blit(image,(0,0))
    ### HANGER ###
    pygame.draw.line(screen, 000000, (150, 90), (150, 550), 5)
    pygame.draw.line(screen, 000000, (300, 90), (150, 90), 5)       
    pygame.draw.line(screen, 000000, (90, 550), (300, 550), 5) 
    pygame.draw.line(screen, 000000, (150, 150), (200, 90), 5)   
    ### ROPE ###
    pygame.draw.line(screen, 000000, (250, 90), (250, 140), 5)   
    ### HANGMAN ###
    pygame.draw.circle(screen, 000000, (270, 160), 30, 5)
    pygame.draw.line(screen, 000000, (250, 180), (250, 300), 5)   
    pygame.draw.line(screen, 000000, (250, 200), (230, 270), 5)
    pygame.draw.line(screen, 000000, (250, 200), (270, 270), 5)
    pygame.draw.line(screen, 000000, (250, 300), (230, 420), 5)
    pygame.draw.line(screen, 000000, (250, 300), (260, 420), 5)
    pygame.display.flip() 

pygame.quit()