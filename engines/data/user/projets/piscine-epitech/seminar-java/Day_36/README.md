# Day 36 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day36/](solutions_day36/)

## Objectifs
- Approfondir les interfaces et leur implémentation.  
- Comprendre les classes abstraites et les méthodes finales.  
- Gérer les exceptions personnalisées.  

## Actions
- **Character** : Classe abstraite avec attributs protégés (name, life, agility, strength, wit, RPGClass).  
- **Movable** : Interface définissant les mouvements (moveRight, moveLeft, moveForward, moveBack).  
- **Warrior** : Hérite de Character, override des méthodes de mouvement avec messages personnalisés.  
- **Mage** : Hérite de Character, implémentation spécifique avec messages personnalisés.  
- **Exceptions** : Gestion des exceptions avec WeaponException, méthodes throws et try-catch.  

## Leçons
- **Interfaces** : Contrats à implémenter, méthodes abstraites.  
- **Classes abstraites** : Classe de base non instanciable, méthodes communes.  
- **Méthodes finales** : Empêchent la redéfinition (unsheathe).  
- **Exceptions personnalisées** : Création de WeaponException, gestion avec throws et try-catch.  
- **Encapsulation** : Attributs protected pour l'héritage, getters publics.  
- **Override** : Redéfinition des méthodes parentes avec comportement spécifique.  
