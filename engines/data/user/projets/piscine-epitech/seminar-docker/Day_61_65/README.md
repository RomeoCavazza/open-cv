# Days 61–65 — Notes

- Objectifs Bootstrap: [OBJECTIVES.md](OBJECTIVES.md)
- Objectifs Projet: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day61_65/](solutions_day61_65/)

## Objectifs
- Découvrir Docker et la conteneurisation.  
- Créer des Dockerfiles pour différentes technologies (Node.js, Python, Java).  
- Orchestrer plusieurs services avec Docker Compose.  
- Développer une application complète de vote distribuée (Popeye).  
- Gérer la communication inter-services (Redis, PostgreSQL).

## Actions

### Bootstrap - Application Node.js conteneurisée
- **Application Hello World** : Application Express simple sur Node.js
- **Dockerfile** : Création du Dockerfile avec base Debian, installation Node.js 16.x, copie du code, installation des dépendances
- **Docker Compose** : Configuration avec service hello-world (build) et service db (PostgreSQL 15.2-alpine)
- **Ports** : Mapping des ports (8080:3000 pour l'app, 5432:5432 pour PostgreSQL)
- **Build et run** : Construction de l'image et exécution avec Docker Compose

### Projet Popeye - Application de vote distribuée
- **Architecture microservices** : Application complète avec 5 services
  - **poll** : Application Flask (Python) pour l'interface de vote
    - Routes GET/POST pour afficher et enregistrer les votes
    - Connexion à Redis pour la queue de votes
    - Gestion des cookies (voter_id)
  - **worker** : Worker Java qui traite les votes depuis Redis
    - Connexion à Redis (blpop pour consommer la queue)
    - Connexion à PostgreSQL pour persister les votes
    - Gestion des doublons (INSERT ou UPDATE)
  - **result** : Application Node.js pour afficher les résultats en temps réel
    - Connexion à PostgreSQL pour récupérer les résultats
    - Interface web avec Socket.io pour les mises à jour en temps réel
  - **redis** : Queue de messages pour les votes (Redis 7.2-alpine)
  - **db** : Base de données PostgreSQL 15.2-alpine pour la persistance
- **Docker Compose** : Configuration complète avec :
  - Réseaux séparés par tiers (poll-tier, back-tier, result-tier)
  - Volumes nommés pour la persistance PostgreSQL
  - Variables d'environnement pour la configuration
  - Dépendances avec depends_on
  - Restart policies (unless-stopped)
- **Dockerfiles** : Un Dockerfile par service (poll, worker, result)
  - **poll** : Image Python avec Flask, installation des dépendances
  - **worker** : Image Java avec Maven, compilation et exécution
  - **result** : Image Node.js avec Express et Socket.io

## Leçons
- **Dockerfile** : Instructions FROM, RUN, WORKDIR, COPY, EXPOSE, CMD
- **Images** : Couches, cache, optimisation avec apt-get clean, images minimales (alpine)
- **Conteneurs** : Isolation, ports mapping, variables d'environnement
- **Docker Compose** : Définition de services, build context, ports, environment, networks, volumes
- **Architecture microservices** : Séparation des responsabilités (frontend, worker, backend, data)
- **Communication inter-services** : Redis pour la queue, PostgreSQL pour la persistance
- **Réseaux Docker** : Isolation par tiers, communication contrôlée entre services
- **Volumes** : Persistance des données, volumes nommés, init scripts (schema.sql)
- **Orchestration** : Dépendances, restart policies, healthchecks
- **Multi-langages** : Intégration Python, Java, Node.js dans une même stack
- **Bonnes pratiques** : Images minimales, gestion des erreurs, logging, monitoring

