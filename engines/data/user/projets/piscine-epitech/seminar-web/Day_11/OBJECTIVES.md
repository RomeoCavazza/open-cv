# Day 11 — Objectifs et Enseignements

## Objectifs

- Pratiquer Linux/Bash (scripts, permissions, globbing).  
- Manipuler commandes de base (`ls`, `find`, archiver/compresser).  
- Automatiser un flux Git simple (add/commit/push).

## Enseignements

- Shebang et exécution : `#!/bin/bash` + `chmod +x` pour rendre un script exécutable.  
- `find` : regrouper les motifs avec parenthèses échappées `\( ... -o ... \)` ; cibler `-type f` pour éviter les dossiers ; prudence avant `-delete`.  
- `ls -p -m` : formatage pratique pour un rendu compact (virgules) et lisible (suffixe `/`).  
- Git : le premier `push` d’une nouvelle branche requiert souvent `--set-upstream` ; sécuriser l’usage en validant les arguments (`$#`) et en affichant un `Usage`.  
- Noms avec espaces : toujours citer les chemins (`"..."`).  
- Archivage/Compression : distinguer tar (archiver) et gzip (compresser) ; produire un `.tar.gz` ou un flux tar pipé vers gzip selon les consignes.
