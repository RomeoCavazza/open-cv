# Day 09 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [hangman_game/](hangman_game/)

## Objectifs
- Relier back (moteur zombie) et front (pygame).  
- Tirer mot aléatoire depuis fichier passé en argument (`words.txt`).  
- Ajouter best score (fichier).  
- Rendre le jeu robuste (inputs, fichiers, états limites).  
- Afficher état complet dans l’UI (HUD).  

## Actions
- Implémentation `load_words_from_argv()` → choix mot aléatoire.  
- Refactor back en API (`init_game`, `apply_letter`, `apply_word`, `progress_bar`, `best_score_update`).  
- Création front pygame : gestion clavier (tampon), affichage HUD + gibet.  
- Test CLI (`python main.py words.txt`) → debug robustesse.  
- Ajout persistance scores (`best_scores.txt`).  

## Leçons
- Séparer clairement logique (back) et interface (front).  
- Robustesse = gestion stricte des erreurs (argv, fichiers, inputs).  
- Pygame → event loop + tampon clavier = remplacement naturel de `input()`.  
- Best score simple = append + comparaison minimale.  
- Avancer bloc par bloc → moteur stable → UI → features.  
