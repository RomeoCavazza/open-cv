# RecruitAI — Spécifications Techniques (Blueprint)

Ce document est la référence technique du projet. Il décrit l'architecture hexagonale, le pipeline IA et les choix technologiques structurants.

## 1. Vision et Objectifs
Le système transforme une offre d'emploi brute en un pack de candidature complet et structuré :
1. Analyse "Reverse-Engineering" : Décodage des missions réelles et des enjeux cachés.
2. CV sur mesure : JSON structuré injecté dans un moteur de rendu A4.
3. Lettre de motivation : Argumentaire ciblé découpé en sections sémantiques.

## 2. Stack Technologique
- Backend : Rust (Axum + Tokio + SQLx).
- Base de données : PostgreSQL 16 + pgvector (RAG).
- IA : Abstraction multi-modèles (Anthropic Claude, OpenAI, Ollama).
- Frontend : Vanilla JS / HTML5 / CSS3 (Zéro framework).
- Environnement : Nix (reproductibilité) + Just (automatisation).

## 3. Architecture Hexagonale (Workspace Cargo)
Le code est découpé en 5 crates pour assurer un découplage total de la logique métier :
- domain : Modèles de données purs.
- ports : Interfaces (traits) définissant les besoins du domaine.
- adapters : Implémentations concrètes.
- application : Cas d'utilisation et orchestration RAG.
- api : Point d'entrée HTTP Axum et service des fichiers statiques.

## 4. Pipeline de Génération IA
L'intelligence du système repose sur la génération structurée :
1. Retrieval : Recherche vectorielle des segments de profil pertinents.
2. Reranking : Scoring des segments par l'IA.
3. Planning : Définition de la stratégie de réponse.
4. Extraction : Génération des payloads JSON contraints.
5. Persistance : Validation métier et stockage dans PostgreSQL.

## 5. Architecture Frontend
Le frontend est minimaliste pour garantir performance et pérennité :
- Isolation via Iframes : Les documents sont rendus dans des iframes isolées.
- Polling Centralisé (Master Poller) : Une seule boucle de polling dans la fenêtre parente gère les requêtes API pour tous les documents en cours, réduisant la charge réseau.
- État Réactif via Storage : Utilisation des événements `window.onstorage` pour notifier les iframes des changements d'état sans polling redondant.
- Vanilla ES6 : Utilisation intensive de modules natifs.
- Routage Client : Gestion des vues via history.pushState.

## 6. Modèle de Données (Postgres)
- offres : Source brute + analyse IA.
- profils : Données candidat + annexes binaires.
- chunks : Fragments de profil vectorisés pour le RAG.
- instances : Lien profil-offre + historique des messages de chat.

## 7. Roadmap et Hardening (Phase Actuelle)

La priorité est passée de la stabilité de base à la **résilience industrielle**.

### HIGH (Résilience Système)
1. **Job Persistence** : Mise en place d'une table de jobs pour garantir la reprise des générations après redémarrage serveur.
2. **Gestion de Troncature** : Découpage intelligent du contexte pour éviter les erreurs `LlmError::Truncated` sur les gros profils.
3. **Classification Rust** : Migration de la logique de catégorisation du SQL vers le backend Rust.

### MED (Robustesse et UX)
1. **Master Poller & Reactive UI** : Centralisation du polling et notifications sonores via `localStorage` (Déployé).
2. **Partial Refresh** : Capacité de régénérer et rafraîchir un seul livrable sans recharger tout l'écran.
3. **Scraping Resilience** : Intégration de **ScrapingAnt** pour contourner les protections anti-bot complexes.

### LOW (Hygiène et Polish)
1. **Design Premium** : Peaufinage des micro-animations et transitions.
2. **Chat Unifié** : Support multi-offres dans l'interface de discussion.

## 8. Validation Q&A End-to-End
Pour garantir une stabilité durable, les scénarios suivants doivent être validés manuellement et automatisés :
- [x] Génération via dashboard global (restitution, cv, cover letter).
- [x] Génération via slots vides individuels.
- [x] Régénération via icônes d'écrasement.
- [x] Rendu immédiat post-génération (disparition du skeleton via BackgroundPollManager).
- [ ] Cohérence du chat avec injection complète du `JSON.profile`.
