# Day 37 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day37/](solutions_day37/)

## Objectifs
- Découvrir les génériques (Generics) en Java.  
- Comprendre les avantages du typage fort à la compilation.  
- Maîtriser les génériques simples et multiples.  

## Actions
- **Ex01 - Solo** : Classe générique simple `<T>` pour un conteneur d'une valeur.  
- **Ex02 - Pair** : Classe générique avec deux paramètres `<T, V>` pour stocker deux valeurs.  
- **Ex03 - Duet** : Méthodes statiques génériques min() et max() avec contraintes Comparable.  
- **Ex04 - Battalion** : Gestion de collections génériques avec List<Character>.  
- **Comparable** : Interface implémentée par Character pour le tri.  

## Leçons
- **Génériques** : Typage fort à la compilation, `<T>` pour un type générique.  
- **Paramètres multiples** : `<T, V>` pour plusieurs types génériques.  
- **Contraintes** : `<T extends Comparable<T>>` pour limiter les types acceptés.  
- **Type safety** : Détection des erreurs à la compilation plutôt qu'à l'exécution.  
- **Wildcards** : `?` pour les types inconnus, `? extends` pour la covariance.  
- **Raw types** : À éviter, perte de sécurité de type.  

