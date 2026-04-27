# Days 61–65 — Objectifs et Enseignements

## Objectifs

- Découvrir Docker et la conteneurisation.  
- Créer des Dockerfiles pour différentes technologies (Node.js, Python, Java).  
- Orchestrer plusieurs services avec Docker Compose.  
- Développer une application complète de vote distribuée (Popeye).  
- Gérer la communication inter-services (Redis, PostgreSQL).

## Enseignements

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
