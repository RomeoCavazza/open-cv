# Day 02 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day02/](solutions_day02/)

## Objectifs
- Découvrir Python (bases, divisions, modulo).  
- Générer suites et manipuler chiffres.  
- Approcher π par séries infinies.  

## Actions
- Tests dans le REPL Python (`/` vs `//`).  
- Génération de la suite 1 + 11 + 111 …  
- Utilisation du modulo (pair/impair, divisions euclidiennes).  
- Extraction et somme des digits d’un entier.  
- Séparation partie entière et décimale d’un flottant.  
- Implémentation série de Leibniz pour π.  
- Implémentation série de Nilakantha pour π.  

## Leçons
- **Division** : `/` = flottant, `//` = entière.  
- **Modulo** : outil central pour parité et reste euclidien.  
- **Suites** : générées par concaténation et boucles.  
- **Digits** : extraction via `str()`, conversion `int()`, utilisation `sum(map(int,...))`.  
- **Parties d’un flottant** : `int(n)` pour partie entière, `n - int(n)` pour partie décimale.  
- **Approximation de π** :  
  - **Leibniz** : π=4k=0∑∞​2k+1(−1)k​ ⮕ brillant pour l'époque mais lent et peu précis.  
  - **Nilakantha** : π=3+4n=2,4,6,…∑∞​n(n+1)(n+2)(−1)n/2+1​ ⮕ plus verbeuse mais convergence rapide, plus fiable.
