# Day 34 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day34/](solutions_day34/)

## Objectifs
- Maîtriser l'héritage et les classes abstraites.  
- Comprendre les enum et leur utilisation.  
- Gérer les compteurs statiques et la pluralisation.  

## Actions
- **Ex01** : Classe Animal avec enum Type (MAMMAL, FISH, BIRD) et constructeur protégé.  
- **Ex02** : Compteurs statiques pour chaque type d'animal avec gestion singulier/pluriel.  
- **Ex03** : Classe Cat héritant d'Animal avec couleur et méthode meow().  
- **Ex04** : Constructeurs multiples pour Cat avec délégation this().  
- **Ex05** : Intégration complète avec gestion automatique du miaulement à la création.  

## Leçons
- **Enum** : types énumérés pour les constantes, toString().toLowerCase().  
- **Héritage** : extends, super(), accès aux attributs protégés.  
- **Compteurs statiques** : variables de classe partagées entre toutes les instances.  
- **Pluralisation** : gestion des cas singulier/pluriel dans les messages.  
- **Constructeurs multiples** : délégation pour éviter la duplication de code.  
- **Visibilité** : protected pour l'héritage, private pour l'encapsulation.  
- **Ordre des messages** : attention à la séquence d'affichage dans les tests.
