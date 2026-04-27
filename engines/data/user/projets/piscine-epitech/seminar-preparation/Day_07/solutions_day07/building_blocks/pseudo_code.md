# Pseudo-code — Hangman (Day 07)

## Réflexions
objectif = faire une version console du jeu du pendu.  
pas besoin d’interface, juste texte / terminal.  
règles principales :  
- 12 pénalités max,  
- lettres fausses = +1,  
- mot faux = +5,  
- mot trouvé = gagné.  

---

## Initialisation
- importer `random` et `english_words_lower_set`  
- choisir un mot aléatoire dans la liste (mot_secret)  
- créer une version masquée du mot (mot_affiche = underscores `_ _ _`)  
- compteur de pénalités = 0  
- limite = 12  

---

## Boucle principale
tant que (pénalités < limite) ET (mot_affiche ≠ mot_secret) :  
1. afficher mot_affiche (ex: `A _ _ L E`)  
2. afficher nombre de pénalités actuelles  
3. demander input au joueur (lettre ou mot entier)  

---

## Traitement des choix
si input est UNE SEULE lettre :  
- si la lettre est dans mot_secret → révéler toutes les occurrences dans mot_affiche  
- sinon → pénalités += 1  

si input est un MOT complet :  
- si mot == mot_secret → victoire immédiate (sortir de la boucle)  
- sinon → pénalités += 5  

---

## Conditions de fin
après la boucle :  
- si mot_affiche == mot_secret → afficher "Victoire !"  
- sinon si pénalités ≥ limite → afficher "Défaite !" + révéler le mot_secret  

---

## Notes / Bonus
- penser à ignorer la casse (maj/min).  
- gérer les lettres déjà proposées (éviter double pénalité).  
- possibilité d’extension : plusieurs joueurs, thèmes, timer, vies custom, ASCII art…  
