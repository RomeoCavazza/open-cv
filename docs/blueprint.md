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

## 7. Roadmap et Hardening (Phase MVP 66% -> 100%)

La priorité absolue est le **Hardening de la pipeline Data/AI** et l'optimisation de l'expérience interactive.

### PHASE A : Hardening Pipeline (Le "66%")
1. **Scraping Industriel** :
    - [ ] Validation du parsing sur les plateformes majeures (LinkedIn, Indeed, Welcome to the Jungle).
    - [ ] Implémentation du fallback **ScrapingAnt** (Proxy/Anti-bot/Cloudflare).
    - [ ] Support des "Prompts Directs" : Génération sans URL (ex: "Génère un CV DevOps") -> CV/Lettre génériques.
2. **Génération & Queueing** :
    - [ ] Orchestration asynchrone pour gérer plusieurs liens à la suite.
    - [ ] Mise en place d'une file d'attente (Queuing) et limite anti-spam.
    - [ ] Synchronisation Sidebar : Apparition instantanée et correcte (Nom Offre/Recruteur) après ingestion.

### PHASE B : Intelligence Interactive (Le "34%")
1. **Chatbar & RAG Optimization** :
    - [ ] Vérification de l'injection Contextuelle : S'assurer que le profil et les chunks RAG sont transmis à 100%.
    - [ ] **JSON Mutations** : Permettre au LLM de modifier directement la structure JSON des documents via le chat.
    - [ ] Comportement LLM : Ajustement du ton et de l'efficacité en phase de "refining".
2. **UI "Alive" & Versioning** :
    - [ ] Micro-animations d'attente (dots, planning/reasoning/generating status).
    - [ ] **Mini-Versioning** : Exploration d'un système de diffs local (inspiré de `mini-git`) pour naviguer dans l'historique des modifications LLM.

## 8. Validation Q&A End-to-End
Pour garantir une stabilité durable, les scénarios suivants doivent être validés manuellement et automatisés :
- [ ] Ingestion d'une offre protégée par Cloudflare.
- [ ] Génération concurrente de 3 instances en arrière-plan.
- [ ] Modification d'un paragraphe du CV via le chat et validation du JSON résultant.

- [x] Génération via dashboard global (restitution, cv, cover letter).
- [x] Génération via slots vides individuels.
- [x] Régénération via icônes d'écrasement.
- [x] Rendu immédiat post-génération (disparition du skeleton via BackgroundPollManager).
- [ ] Cohérence du chat avec injection complète du `JSON.profile`.

---

## 9. Évolution du Frontend (Pistes de Réflexion)

Bien que l'approche **Vanilla JS** soit actuelle et ultra-performante, deux technologies s'alignent parfaitement avec la philosophie "minimaliste et robuste" du projet pour réduire la verbosité du code de manipulation du DOM :

### 9.1 HTMX (Le choix du Server-Side)
- **Concept** : Permet d'effectuer des requêtes AJAX, de gérer des WebSockets et des Server-Sent Events directement via des attributs HTML.
- **Avantage pour RecruitAI** : Au lieu de recevoir du JSON et de reconstruire le DOM en JS (ex: la sidebar des offres), Axum pourrait renvoyer directement un fragment HTML.
- **Bénéfice** : Suppression de 50% du code JS côté client. Cohérence totale avec la puissance du backend Rust.

### 9.2 Alpine.js (Le "Tailwind du JS")
- **Concept** : Un framework déclaratif ultra-léger (8kb) qui s'utilise directement dans le HTML pour gérer les états locaux (modals, onglets, toggles).
- **Avantage pour RecruitAI** : Remplacerait les `document.getElementById` verbeux pour la gestion des pillules LLM (`llm-pill`) et des états d'affichage des boutons.
- **Bénéfice** : Code frontend plus lisible, déclaratif et plus facile à maintenir sans ajouter de complexe de build (pas de npm/webpack nécessaire).
