# Audit Technique - RecruitAI

Date de l'audit : 8 mai 2026
Statut global : **Sain** (Architecture robuste, mais quelques points de friction identifiés).

## 1. Synthèse de la Dette Technique

### 1.1 Backend (Rust)

| Point | Description | Impact | Priorité |
| :--- | :--- | :--- | :--- |
| **Duplication de logique** | `maybe_generate_resume` et `maybe_generate_cover_letter` sont quasi identiques dans `generate/steps.rs`. | Maintenabilité | Moyenne |
| **Erreurs génériques** | Usage fréquent de `AppError::Other(e.to_string())`. | Diagnostic | Faible |
| **Logique métier en SQL** | La classification des offres (`fn_infer_offre_category`) est gérée par un trigger SQL avec Regex. | Testabilité | Haute |
| **Stockage Binaire** | Photos et PDF stockés en `BYTEA` dans PostgreSQL. | Performance / Backup | Faible |
| **Gestion des Tâches (Queue)** | Les générations sont lancées via `tokio::spawn` direct, sans file d'attente (Job Queue) ni worker pool. Risque de surcharge LLM. | Fiabilité à l'échelle | Haute |
| **Statut d'Instance Global** | Un seul statut `Generating` pour l'instance. **Partiellement résolu** : Le `BackgroundPollManager` permet désormais un suivi atomique par document via `localStorage`. | Robustesse UI / Race conditions | Moyenne |

### 1.2 Frontend (Vanilla JS)

| Point | Description | Impact | Priorité |
| :--- | :--- | :--- | :--- |
| **Manipulation DOM** | Mises à jour manuelles via `document.getElementById` sans framework réactif. | Robustesse UI | Moyenne |
| **Pont Legacy** | `window.state` utilisé comme pont entre les nouveaux contrôleurs et l'ancien code. | Qualité de code | Moyenne |
| **UX Erreurs** | Usage mixte de `alert()` et de `Toast`. **Amélioré** : Migration vers notifications sonores et architecture Master Poller. | Expérience Utilisateur | Faible |

---

## 2. Analyse Détaillée

### 2.1 Architecture Hexagonale
L'architecture est très bien respectée. La séparation des préoccupations entre le `domain`, les `ports` (interfaces) et les `adapters` (implémentations concrètes) est claire.
- **Point positif** : Facilité déconcertante pour changer de provider LLM (Claude, OpenAI, Ollama).
- **Axe d'amélioration** : Le module `generate` commence à dépasser la taille critique. Une décomposition par phase de pipeline (`Retrieve`, `Rerank`, `Plan`, `Generate`) dans des fichiers séparés est recommandée.

### 2.2 Base de Données
L'utilisation de `pgvector` et des index GIN pour la recherche plein texte est excellente.
- **Risque identifié** : Le trigger `tg_infer_offre_category` est une "boîte noire" métier. Si un utilisateur veut modifier les catégories, il doit modifier le schéma SQL plutôt que la configuration applicative.

### 2.3 Performance et Compilation
- **Dépendances** : Le projet présente quelques duplications de versions pour des crates comme `base64` ou `chrono`.
- **Binaire** : `cargo-bloat` indique un binaire de ~2.1MiB, ce qui est excellent pour une application de cette envergure.

---

## 3. Recommandations de Hardening

1.  **Refactoring Pipeline** : Fusionner les logiques de génération de documents dans `application/src/generate/steps.rs` pour utiliser un template de génération structurée commun.
2.  **Migration de Logique** : Déplacer la classification des offres du SQL vers une couche de service Rust pour permettre des tests unitaires sur les règles de tri.
3.  **UI Consistency** : Supprimer tous les `alert()` au profit du système de Toasts déjà présent dans `ui.js`.
4.  **Nettoyage JS** : Continuer la migration vers les contrôleurs (`ProfileController`, etc.) pour supprimer totalement la dépendance à `window.state`.
5.  **Système de Background Jobs** : Remplacer les `tokio::spawn` par un vrai Message Broker / Job Queue (ex: table `jobs` SQL ou `mpsc`) et atomiser les statuts (`resume_status`, `cover_letter_status`) pour une véritable concurrence asynchrone sécurisée.
