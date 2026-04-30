# Index de la documentation

Ce répertoire `/docs` centralise l'ensemble de la documentation du projet. Il mélange délibérément la documentation de la version actuellement en production et les brouillons/propositions pour les futures évolutions majeures du système.

Voici comment s'y retrouver :

## 1. Documentation de la version actuelle
- **`how_it_works.md`**, **`instructions.md`**... : Ces fichiers décrivent le fonctionnement de l'outil dans son état actuel (architecture basée sur des scripts Python, fichiers JSON, et rendu frontend statique Vanilla JS/CSS).

## 2. L'évolution vers la V3 (Backend Rust & Front dynamique)
- **`blueprint.md`** : C'est le document d'architecture cible. Il propose et détaille l'évolution du projet vers un backend Rust complet (Axum, base PostgreSQL, architecture hexagonale), l'intégration avancée de l'IA (LLMs et Embeddings) et une refonte de l'expérience frontend (Svelte).
- **`design.md`** : Un document de style sous forme de "pseudo-prompt" qui décrit de manière exhaustive le design system cible (inspiré de Coinbase, avec des tons violet/indigo, et des typographies calmes et institutionnelles).

## 3. Preuves de concept (PoC)
- **Dossier `tar.gz/`** : Contient deux archives compressées (`alternance-skel.tar.gz` et `alternance-skel-v2.tar.gz`). Ce sont des propositions concrètes d'implémentation (squelettes de code) pour la future architecture backend en Rust.
