# Carte du Projet (Project Map)

Ce document décrit l'arborescence complète du projet RecruitAI et le rôle de chaque fichier/dossier.

## Backend (Rust Workspace)

Le backend suit une architecture hexagonale découpée en crates.

### 1. crates/domain/ (Le Coeur)
Contient les modèles de données purs.
- profil.rs : Structs typées du candidat.
- offre.rs : Modèle d'une offre d'emploi et de son analyse.
- instance.rs : Lien entre un profil et une offre.
- chunk.rs : Fragments de texte vectorisés pour le RAG.
- ids.rs : Types d'identifiants fortement typés.

### 2. crates/ports/ (Les Interfaces)
Définit les traits (contrats) que les adapters doivent implémenter.
- repos.rs : Interfaces de persistance.
- llm.rs : Interface pour les appels aux modèles d'IA.
- embedder.rs : Interface pour la vectorisation.
- scraper.rs : Interface pour l'extraction de texte.

### 3. crates/adapters/ (Les Implémentations)
Contient le code lié aux outils et services externes.
- postgres/ : Persistance via SQLx.
- llm_claude/, llm_openai/, llm_ollama/ : Clients API pour les différents modèles.
- scraper_http/ : Web scraping HTTP basique.

### 4. crates/application/ (La Logique Métier)
Orchestration des cas d'utilisation (Use Cases).
- intake/ : Pipeline d'ingestion.
- generate/ : Pipeline de génération de candidature.
- chat/ : Logique interactive modulaire (types.rs, persistence.rs, streaming.rs).
- prompts/ : Gestion centralisée des System Prompts.

### 5. crates/api/ (L'Interface HTTP)
Point d'entrée Axum.
- handlers/ : Pont entre requêtes HTTP et Use Cases.
- bin/ : Scripts de seeding.

---

## Frontend (Vanilla JS)

Le frontend fonctionne en architecture modulaire orientée contrôleurs.

### 1. web/assets/js/ (Coeur)
- dashboard.js : Orchestrateur central.
- api.js : Client HTTP centralisé.
- chat.js : Gestion de l'interaction IA temps réel.
- ui.js : Composants UI globaux.

### 2. web/assets/js/controllers/ (Business Logic)
- OfferController.js, ProfileController.js, IngestController.js.

### 3. web/assets/js/modules/ (Infrastructure)
- events.js : Bus d'événements global.

---

## Infrastructure et Documentation

- data/ : Templates JSON, scripts de seeding et assets statiques.
- migrations/ : Schéma SQL Postgres.
- docs/ : Documentation technique (blueprint.md, toolkit.md).
- Justfile : Commandes automatisées.

---

## Dette Technique & Cleanup

- **LlmError::Truncated** : Remplacer le smoke test manuel par un test unitaire permanent.
- **scraper_ant** : Implémenter l'adapter comme fallback HTTP pour contourner les protections anti-bot.
- **seed_offers_instances.rs** : Supprimer les types `Legacy*` et le code de migration associé.
