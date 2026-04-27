# Day 16 — Objectifs et Enseignements

## Objectifs

- Pratiquer les bases JavaScript (fonctions, boucles, tableaux, chaînes).  
- Écrire des utilitaires classiques (FizzBuzz, range, filter).  
- Comparer structures (égalité superficielle et profonde).  
- Comprendre les systèmes de modules (ESM `export` vs CommonJS `module.exports`).

## Enseignements

- Modules JS : harmoniser le style (ESM `export` partout ou CommonJS partout). Mélanger `export` et `module.exports` complique l’import.  
- API natives : préférer `Array.prototype.filter` pour la lisibilité/perf, tout en sachant réécrire la logique.  
- Robustesse :  
  - `fizzBuzz` devrait prendre les bornes ou la limite en paramètre;  
  - `range` doit gérer `step = 0` (erreur) et valider les bornes;  
  - `objectsDeeplyEqual` doit distinguer objets vs tableaux et ignorer le prototype (`hasOwnProperty`).  
- Sensibilité à la casse : `countGs` peut normaliser (`toUpperCase()`) si besoin.  
- Style : conserver une interface de sortie cohérente (retourner les résultats au lieu de `console.log` dans les fonctions pures, puis afficher au niveau appelant).
