# Day 09 — Objectifs et Enseignements

## Objectifs

- Relier back (moteur zombie) et front (pygame).  
- Tirer mot aléatoire depuis fichier passé en argument (`words.txt`).  
- Ajouter best score (fichier).  
- Rendre le jeu robuste (inputs, fichiers, états limites).  
- Afficher état complet dans l’UI (HUD).

## Enseignements

- Séparer clairement logique (back) et interface (front).  
- Robustesse = gestion stricte des erreurs (argv, fichiers, inputs).  
- Pygame → event loop + tampon clavier = remplacement naturel de `input()`.  
- Best score simple = append + comparaison minimale.  
- Avancer bloc par bloc → moteur stable → UI → features.
