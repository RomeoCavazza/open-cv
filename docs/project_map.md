# Carte du Projet (Project Map)

Ce document décrit l'arborescence complète du projet RecruitAI et le rôle de chaque fichier/dossier.

## 📦 Backend (Rust Workspace)

Le backend suit une architecture hexagonale découpée en **crates**.

### 1. `crates/domain/` (Le Cœur)
*Contient les modèles de données purs, sans dépendance technique.*
- `profil.rs` : Structs typées du candidat (Identité, Expériences, Compétences).
- `offre.rs` : Modèle d'une offre d'emploi et de son analyse structurée.
- `instance.rs` : Lien entre un profil et une offre (contient le CV et la Lettre générés).
- `chunk.rs` : Fragments de texte vectorisés pour le RAG.
- `ids.rs` : Types d'identifiants fortement typés (UUID wrappers).

### 2. `crates/ports/` (Les Interfaces)
*Définit les traits (contrats) que les adapters doivent implémenter.*
- `repos.rs` : Interfaces de persistance (ProfilRepo, OffreRepo, etc.).
- `llm.rs` : Interface pour les appels aux modèles d'IA (Completion & Extraction).
- `embedder.rs` : Interface pour la vectorisation de texte.
- `scraper.rs` : Interface pour l'extraction de texte depuis des URLs.

### 3. `crates/adapters/` (Les Implémentations)
*Contient le code lié aux outils et services externes.*
- **`postgres/`** : Persistance via SQLx.
- **`llm_claude/`, `llm_openai/`, `llm_ollama/`** : Clients API pour les différents modèles.
- **`scraper_http/`** : Web scraping basique.

### 4. `crates/application/` (La Logique Métier)
*Orchestration des cas d'utilisation (Use Cases).*
- **`intake/`** : Pipeline d'ingestion d'offres (Scraping -> Extraction IA -> Déduplication).
- **`generate/`** : Pipeline de génération de candidature (Retrieval -> Reranking -> Planning -> Generation).
- **`chat/`** : Logique de discussion interactive avec l'IA sur une candidature.
- **`prompts/`** : Gestion centralisée des System Prompts.

### 5. `crates/api/` (L'Interface HTTP)
*Point d'entrée Axum.*
- `main.rs` : Initialisation du serveur et de la base.
- `lib.rs` : Configuration du routeur et du service statique.
- `handlers/` : Pont entre requêtes HTTP et Use Cases.
- `bin/` : Scripts de seeding (`seed_profile`, `seed_offers_instances`).

---

## 🌐 Frontend (Vanilla JS)

Le frontend est situé dans `/web` et fonctionne en architecture modulaire orientée contrôleurs.

### 1. `web/assets/js/` (Cœur & Orchestration)
- `dashboard.js` : **Orchestrateur central**. Gère le routage et délègue la logique aux contrôleurs.
- `api.js` : Client HTTP centralisé.
- `ui.js` : Composants et utilitaires UI globaux (Toasts, Skeletons, Modals).
- `router.js` : Routage SPA via `history.pushState`.

### 2. `web/assets/js/controllers/` (Business Logic)
- `OfferController.js` : Gestion des offres (chargement, sélection, filtrage).
- `ProfileController.js` : Gestion du profil candidat (édition, sauvegarde, upload).
- `IngestController.js` : Gestion de l'ingestion de nouvelles offres.

### 3. `web/assets/js/modules/` (Infrastructure)
- `events.js` : **Bus d'événements** (Pattern EventTarget) pour le découplage inter-modules.
- `view.js` : Gestion des transitions de vue (Tabs, Loader states).

### 4. Moteurs de Rendu (Renderers)
- **`web/resume/`**, **`web/cover-letter/`**, **`web/restitution/`** : Rendu de documents isolés.

---

## 🛠️ Infrastructure & Outils

- **`migrations/`** : Schéma SQL Postgres.
- **`Justfile`** : Commandes automatisées (`just dev`, `just test`).
- **`tooling/`** : Configurations (ESLint, Stylelint, Cargo-deny).
- **`docs/`** : Documentation (Cleanup, Design, Data Management).
