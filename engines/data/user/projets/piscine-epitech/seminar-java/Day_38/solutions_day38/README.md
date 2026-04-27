# Day 38 — Solutions

- Objectifs: [../OBJECTIVES.md](../OBJECTIVES.md)
- Solutions: [solutions_day38/](./)

## Structure des solutions

### Factory Pattern (Ex01-Ex02)
- **Toy** : Classe abstraite avec titre
- **TeddyBear** / **Gameboy** : Implémentations concrètes
- **Factory** : Méthode create() pour créer des jouets, getPapers() pour les emballages
- **Elf** : Gestion de la sélection et de l'emballage des jouets
- **GiftPaper** : wrap() et unwrap() pour gérer les cadeaux

### Composite Pattern (Ex03)
- **Sentence** : Interface commune
- **Word** : Implémentation concrète pour les mots individuels
- **SentenceComposite** : Composition récursive pour les phrases complexes

### Observer Pattern (Ex04)
- **Observable** / **Observer** : Interfaces du pattern
- **Order** : Sujet observable avec notification automatique
- **Customer** : Observateur qui affiche les mises à jour

### Decorator Pattern (Ex05)
- **Warrior** : Classe abstraite de base
- **BasicWarrior** / **KingWarrior** : Implémentations concrètes
- **StuffDecorator** : Classe abstraite pour les décorateurs
- **Shield** / **FireSword** : Décorateurs ajoutant HP ou dégâts

## Compilation et exécution

```bash
# Compilation avec les packages
javac Factory/*.java Composite/*.java Observer/*.java Decorator/*.java

# Exécution (exemple)
java Factory.Example
```
