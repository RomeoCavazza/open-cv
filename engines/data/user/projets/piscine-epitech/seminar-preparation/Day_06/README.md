# Day 06 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day06/](solutions_day06/)

## Objectifs
- Découvrir les fonctions en Python (définition, paramètres, return).  
- Appliquer la récursion pour résoudre des problèmes complexes.  
- Utiliser les fonctions comme paramètres (fonctions d'ordre supérieur).  
- Gérer les exceptions avec try/except.  
- Mesurer les performances et optimiser le code récursif.  

## Actions
- Définition de fonctions simples (avec `return` et `print`).  
- Construction d'un "sandwich" visuel avec assemblage de fonctions.  
- Génération de listes avec list comprehensions et fonctions.  
- Gestion d'input utilisateur avec validation et gestion d'erreurs.  
- Implémentation de fonctions récursives (somme, palindrome).  
- Nettoyage de chaînes (caractères alphanumériques, normalisation).  
- Traversal récursif de système de fichiers avec `os.listdir`.  
- Validateurs de mots de passe (longueur, caractères spéciaux, chiffres).  
- Passage de fonctions comme paramètres (`passcheck`).  
- Challenge : fonction puissance récursive + mesure temporelle.  

## Leçons
- **Fonctions** : bloc réutilisable, `def nom(paramètres):`, `return` vs `print`.  
- **Récursion** : fonction qui s'appelle elle-même, condition d'arrêt indispensable.  
- **Cas de base** : éviter récursion infinie (ex: `len(s) <= 1` pour palindrome).  
- **Nettoyage string** : `isalnum()`, `lower()` pour normaliser avant traitement.  
- **Système de fichiers** : `os.listdir()`, `os.path.join()`, `os.path.isdir()`.  
- **Fonctions d'ordre supérieur** : passer fonction comme paramètre, plus de flexibilité.  
- **Validation** : découper en fonctions spécialisées, combiner résultats.  
- **Try/except** : capturer `ValueError` pour input non-conformes.  
- **Performance** : `time.time()` pour mesurer, récursion peut être coûteuse.  
- **Architecture** : séparer logique métier (validation) de l'interface (input/output).  
