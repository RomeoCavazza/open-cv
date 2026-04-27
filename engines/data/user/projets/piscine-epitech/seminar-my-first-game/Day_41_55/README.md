# Days 41–55 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [MyFirstGame/](MyFirstGame/)

## Objectifs
- Développer un jeu 2D complet en Java en respectant les principes OOP.  
- Appliquer les principes SOLID et les design patterns (State, Observer, Factory, etc.).  
- Mettre en place une architecture modulaire et extensible.  
- Implémenter des systèmes de jeu complexes (états, pathfinding, collisions, IA).  
- Documenter complètement le projet (UML, Javadoc, Game Design Document).  
- Assurer une couverture de tests suffisante avec JaCoCo.  

## Actions
- **Game Design Document** : Conception complète du jeu avant l'implémentation
- **Architecture** : Conception modulaire avec séparation des responsabilités (core, entities, systems, world)
- **Documentation UML** : Diagrammes PlantUML montrant les design patterns utilisés
- **Implémentation** : Développement du jeu avec libGDX (exemple : Wormy avec états Baby/Adult/Super)
- **Systèmes de jeu** : Vie, progression, pathfinding, collisions, gestion des entités
- **Rendu et animations** : Gestion des sprites, animations, caméra, HUD
- **Interface utilisateur** : Menu principal, pause, game over, settings
- **Tests unitaires** : Couverture avec JaCoCo des composants essentiels
- **Javadoc** : Documentation complète de toutes les classes publiques

## Leçons
- **Game Design Document** : Importance de la conception avant l'implémentation, document exhaustif
- **Architecture de jeu** : Game loop, systèmes de rendu, gestion d'état, séparation des responsabilités
- **OOP & Design Patterns** : Application des principes SOLID, State, Observer, Factory dans un contexte réel
- **Documentation UML** : Diagrammes montrant clairement les design patterns et l'architecture
- **libGDX** : Sprites, animations, gestion des entrées, caméra, batch rendering (alternative à JavaFX/AWT/Swing)
- **Tests & Qualité** : JUnit 5, JaCoCo pour la couverture de code, tests des systèmes critiques
- **Javadoc** : Documentation complète et professionnelle pour faciliter la maintenance
- **Optimisation** : Cache de TextureRegion, réduction des allocations, distances au carré
- **Gestion de projet** : Architecture modulaire, documentation exhaustive, versioning, revue de code
