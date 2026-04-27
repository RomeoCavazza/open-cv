# Day 06 — Objectifs et Enseignements

## Objectifs

- Découvrir les fonctions en Python (définition, paramètres, return).  
- Appliquer la récursion pour résoudre des problèmes complexes.  
- Utiliser les fonctions comme paramètres (fonctions d'ordre supérieur).  
- Gérer les exceptions avec try/except.  
- Mesurer les performances et optimiser le code récursif.

## Enseignements

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
