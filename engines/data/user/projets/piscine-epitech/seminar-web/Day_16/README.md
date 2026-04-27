# Day 16 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day16/](solutions_day16/)

## Objectifs
- Pratiquer les bases JavaScript (fonctions, boucles, tableaux, chaînes).  
- Écrire des utilitaires classiques (FizzBuzz, range, filter).  
- Comparer structures (égalité superficielle et profonde).  
- Comprendre les systèmes de modules (ESM `export` vs CommonJS `module.exports`).  

## Actions
- Task 01 — `drawTriangle(height)` : affiche une pyramide de `$` en incrémentant ligne par ligne.  
- Task 02 — `arraysAreEqual(arr1, arr2)` : compare longueur et éléments index par index.  
- Task 03 — `countGs(str)` : compte les caractères `G` via `split` + `filter`.  
- Task 04 — `fizzBuzz()` : génère `1..18` et imprime la séquence combinant `Fizz`, `Buzz` ou l’indice.  
- Task 05 — `range(start, end, step)` : construit un tableau d’entiers croissant/décroissant selon `step` (CommonJS: `module.exports`).  
- Task 06 — `objectsDeeplyEqual(a, b)` : égalité profonde récursive pour objets imbriqués.  
- Task 07 — `arrayFiltering(array, test)` : implémente un `filter` manuel en appelant un prédicat `test`.  

## Leçons
- Modules JS : harmoniser le style (ESM `export` partout ou CommonJS partout). Mélanger `export` et `module.exports` complique l’import.  
- API natives : préférer `Array.prototype.filter` pour la lisibilité/perf, tout en sachant réécrire la logique.  
- Robustesse :  
  - `fizzBuzz` devrait prendre les bornes ou la limite en paramètre;  
  - `range` doit gérer `step = 0` (erreur) et valider les bornes;  
  - `objectsDeeplyEqual` doit distinguer objets vs tableaux et ignorer le prototype (`hasOwnProperty`).  
- Sensibilité à la casse : `countGs` peut normaliser (`toUpperCase()`) si besoin.  
- Style : conserver une interface de sortie cohérente (retourner les résultats au lieu de `console.log` dans les fonctions pures, puis afficher au niveau appelant).  
