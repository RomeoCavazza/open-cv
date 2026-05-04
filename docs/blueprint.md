# RecruitAI — Spécifications Techniques (Blueprint)

Ce document est la référence technique du projet. Il décrit l'architecture hexagonale, le pipeline IA et les choix technologiques structurants.

## 1. Vision et Objectifs
Le système transforme une offre d'emploi brute en un pack de candidature complet et structuré :
1.  **Analyse "Reverse-Engineering"** : Décodage des missions réelles et des enjeux cachés.
2.  **CV sur mesure** : JSON structuré injecté dans un moteur de rendu A4.
3.  **Lettre de motivation** : Argumentaire ciblé découpé en sections sémantiques.

## 2. Stack Technologique
- **Backend** : Rust (Axum + Tokio + SQLx).
- **Base de données** : PostgreSQL 16 + `pgvector` (RAG).
- **IA** : Abstraction multi-modèles (Anthropic Claude, OpenAI, Ollama).
- **Frontend** : Vanilla JS / HTML5 / CSS3 (Zéro framework, zéro build pipeline).
- **Environnement** : Nix (reproductibilité) + Just (automatisation).

## 3. Architecture Hexagonale (Workspace Cargo)
Le code est découpé en 5 crates pour assurer un découplage total de la logique métier :
- **`domain`** : Modèles de données purs (Offre, Profil, Instance). Aucune dépendance externe.
- **`ports`** : Interfaces (traits) définissant les besoins du domaine (Repository, LlmClient).
- **`adapters`** : Implémentations concrètes (Postgres, Claude, Scraper).
- **`application`** : Cas d'utilisation (Ingestion, Génération) et orchestration RAG.
- **`api`** : Point d'entrée HTTP Axum et service des fichiers statiques.

## 4. Pipeline de Génération IA
L'intelligence du système repose sur la génération **structurée** (JSON Schema) :
1.  **Retrieval** : Recherche vectorielle des segments de profil (`chunks`) pertinents pour l'offre.
2.  **Reranking** : Scoring des segments par l'IA pour ne garder que le meilleur "fit".
3.  **Planning** : Définition de la stratégie de réponse.
4.  **Extraction** : Génération des payloads JSON (CV/Lettre) contraints par des schémas stricts.
5.  **Persistance** : Validation métier et stockage dans PostgreSQL.

## 5. Architecture Frontend
Le frontend est délibérément minimaliste pour garantir performance et pérennité :
- **Isolation via Iframes** : Les documents (CV/Lettre) sont rendus dans des iframes isolées. Cela permet un typage CSS spécifique (unités `mm` pour l'impression A4) sans pollution du style global du dashboard.
- **Vanilla ES6** : Utilisation intensive de modules natifs.
- **Routage Client** : Gestion des vues via `history.pushState` et fallback serveur sur `index.html`.

## 6. Modèle de Données (Postgres)
- **`offres`** : Source brute + analyse IA.
- **`profils`** : Données candidat + annexes binaires (PDF/Images).
- **`chunks`** : Fragments de profil vectorisés pour le RAG.
- **`instances`** : Lien profil-offre + historique des messages de chat.
