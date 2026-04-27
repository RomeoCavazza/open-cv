# Day 07 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day07/](solutions_day07/)

## Objectifs
- Implémenter Hangman (console).  
- Décomposer en briques simples.  
- Utiliser package externe (`english-words`).  
- Formaliser pseudocode.  
- Gérer règles (pénalités, victoire/défaite).  
- Tester variantes (Zombie mode, etc.).  

## Actions
- Brique 1 : fonction `check_penalties` (≥12).  
- Brique 2 : random 1–6.  
- Brique 3 : `english-words`, tirage mot.  
- Brique 4 : fonction masque `_ _ _`.  
- Pseudocode structuré (init → boucle → choix → fin).  
- Implémentation Hangman simple (débutant).  
- Expérimentations : version granulaire + infection zombie.  

## Leçons
- Décomposer → avancer bloc par bloc.  
- Installer/activer venv + packages.  
- Masque = liste modifiable pour lettres.  
- Logique stricte : lettre faux = +1, mot faux = +5, loose >12.  
- Pseudocode = plan avant code.  
- Alterner rigueur et créativité → progression efficace.
