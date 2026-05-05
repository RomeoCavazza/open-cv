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
  - `lib.rs` : Connexion et gestion des transactions.
  - `profil.rs`, `offre.rs`, etc. : Implémentations spécifiques des repos SQL.
- **`llm_claude/`, `llm_openai/`, `llm_ollama/`** : Clients API pour les différents modèles.
- **`scraper_http/`** : Web scraping basique.

### 4. `crates/application/` (La Logique Métier)
*Orchestration des cas d'utilisation (Use Cases).*
- **`intake/`** : Pipeline d'ingestion d'offres (Scraping -> Extraction IA -> Déduplication).
- **`generate/`** : Pipeline de génération de candidature (Retrieval -> Reranking -> Planning -> Generation).
- **`chat/`** : Logique de discussion interactive avec l'IA sur une candidature.
- **`prompts/`** : Gestion centralisée des System Prompts envoyés à l'IA.

### 5. `crates/api/` (L'Interface HTTP)
*Point d'entrée Axum.*
- `main.rs` : Initialisation du serveur, de la base de données et des logs.
- `lib.rs` : Configuration du routeur, des middlewares et du service de fichiers statiques.
- `handlers/` : Contrôleurs HTTP qui font le pont entre les requêtes et les Use Cases.
- `bin/` : Scripts utilitaires (Seeding).
  - `seed_profile.rs` : Importation du profil réel.
  - `seed_blank.rs` : Création d'un état vierge.
  - `seed_offers_instances.rs` : Importation de données de test.
- `tests/api_integration.rs` : Tests de bout en bout validant les contrats API.

---

## 🌐 Frontend (Vanilla JS)

Le frontend est situé dans `/web` et fonctionne sans build-pipeline.

### 1. `web/assets/js/` (Logique Globale)
- `api.js` : Client HTTP centralisé pour communiquer avec le backend.
- `state.js` : Gestion de l'état local (offre active, profil chargé).
- `router.js` : Routage côté client (SPA) via `history.pushState`.
- `dashboard.js` : **Monolithe actuel** à découper (gestion de l'UI principale).
- `events.js` : (Prévu) Bus d'événements pour le découplage.

### 2. Moteurs de Rendu (Renderers)
*Documents isolés dans des iframes.*
- **`web/resume/`** : Moteur de rendu HTML/CSS pour le CV (optimisé pour impression A4).
- **`web/cover-letter/`** : Moteur de rendu pour la lettre de motivation.
- **`web/restitution/`** : Rendu de l'analyse "Reverse-Engineering" de l'offre.

---

## 🛠️ Infrastructure & Outils

- **`migrations/`** : Scripts SQL (`0001_init.sql`) définissant le schéma de la base Postgres.
- **`flake.nix`** : Configuration de l'environnement de développement (Rust, Postgres, Just).
- **`Justfile`** : Recettes d'automatisation (`just dev`, `just migrate`, `just test`).
- **`docs/`** : Documentation technique et opérationnelle.
  - `design.md` : Standards UI/UX (Monochrome + Blue).
  - `data_management.md` : Guide de réinitialisation et seeding.
  - `project_map.md` : Ce document.
