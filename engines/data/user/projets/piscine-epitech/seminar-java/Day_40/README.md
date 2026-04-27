# Day 40 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day40/](solutions_day40/)

## Objectifs
- Intégrer toutes les notions Java acquises.  
- Développer une application complète de gestion de boulangerie.  
- Appliquer les génériques, interfaces et design patterns.  

## Actions
- **Ex01 - Hiérarchie Food** : Interface Food, classes abstraites (Bread, Drink, Sandwich, Dessert) et implémentations concrètes.  
- **Ex02 - Menus génériques** : Menu générique `<D extends Food, M extends Food>` avec Breakfast, Lunch, AfternoonTea.  
- **Ex03 - Logique métier** : Stock avec Map<Class<? extends Food>, Integer>, CustomerOrder avec gestion des commandes.  

## Leçons
- **Architecture modulaire** : Hiérarchie d'interfaces et classes abstraites bien structurée.  
- **Génériques avec contraintes** : Menu<D extends Food, M extends Food> pour type-safe.  
- **Reflection appliquée** : Stock utilisant Class<? extends Food> pour gestion générique.  
- **Gestion d'exceptions** : NoSuchFoodException pour la gestion d'erreurs métier.  
- **Collections génériques** : List<Food>, Map<Class<? extends Food>, Integer> pour stockage type-safe.  
- **Application complète** : Du modèle de données à la logique métier, application prête pour production.  

