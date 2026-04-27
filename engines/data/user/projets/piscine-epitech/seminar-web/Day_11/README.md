# Day 11 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day11/](solutions_day11/)

## Objectifs
- Pratiquer Linux/Bash (scripts, permissions, globbing).  
- Manipuler commandes de base (`ls`, `find`, archiver/compresser).  
- Automatiser un flux Git simple (add/commit/push).  

## Actions
- Script `mr_clean` : suppression des fichiers de sauvegarde avec `find . -type f \( -name "*~" -o -name "#*#" \) -delete`.  
- Script `push_that.sh` :  
  - vérifie la présence d’un message de commit,  
  - exécute `git add .`, `git commit -m "$msg"`,  
  - `git push` avec repli `--set-upstream origin main | master` si nécessaire.  
- `task02/z` : création d’un fichier `z` contenant `Z`.  
- `task03/midLS` : `ls -p -m` pour lister en une ligne, séparée par virgules (avec `/` pour les dossiers).  
- `task06` : construction d’une arborescence musicale (genres/artistes/albums/pistes).  
- `task07` : compression/archivage de `task06` au format gzip (`task06.gz`).  

## Leçons
- Shebang et exécution : `#!/bin/bash` + `chmod +x` pour rendre un script exécutable.  
- `find` : regrouper les motifs avec parenthèses échappées `\( ... -o ... \)` ; cibler `-type f` pour éviter les dossiers ; prudence avant `-delete`.  
- `ls -p -m` : formatage pratique pour un rendu compact (virgules) et lisible (suffixe `/`).  
- Git : le premier `push` d’une nouvelle branche requiert souvent `--set-upstream` ; sécuriser l’usage en validant les arguments (`$#`) et en affichant un `Usage`.  
- Noms avec espaces : toujours citer les chemins (`"..."`).  
- Archivage/Compression : distinguer tar (archiver) et gzip (compresser) ; produire un `.tar.gz` ou un flux tar pipé vers gzip selon les consignes.  
