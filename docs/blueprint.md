# Blueprint — Architecture Backend & IA

Ce document décrit l'architecture actuelle du builder de candidatures IA-native (Rust + Postgres + frontend statique). Il sert de référence pour les choix techniques et la structure du projet.

## 1. Postulats et Choix Techniques

Le système est conçu autour de trois livrables fondamentaux :
1. **La restitution de l'offre** : une extraction structurée (synthèse, fit, signaux implicites, points d'attention).
2. **Le CV adapté** : structuré en JSON pour correspondre parfaitement à l'offre.
3. **La lettre de motivation** : rédigée sur mesure, avec des paragraphes sémantiques ciblés.

### La Stack Technique

- **Langage Backend** : Rust
- **Framework HTTP** : Axum + Tokio
- **Base de Données** : PostgreSQL (avec `pgvector`, `pgcrypto`, `pg_trgm`) via `sqlx`
- **Architecture** : Hexagonale (Domain, Ports, Adapters, Application, API)
- **Modèle IA Principal** : client Anthropic Claude via `reqwest`
- **Embeddings** : interface dédiée dans `ports`, actuellement alimentée par un embedder mock côté API
- **Frontend** : HTML/CSS/JS statique servi par Axum, avec iframes pour les documents
- **Environnement** : Nix + Just (reproductibilité absolue)

## 2. Architecture Hexagonale

L'architecture est segmentée en "Crates" (paquets Rust) au sein d'un workspace Cargo :

```
alternance/
├── crates/
│   ├── domain/           # Cœur métier (types de données), zéro dépendance infra
│   ├── ports/            # Interfaces (Traits) que le domaine exige (ex: LlmClient)
│   ├── adapters/         # Implémentations concrètes des ports
│   │   ├── postgres/     # Persistance via sqlx
│   │   ├── llm_claude/   # Client API Anthropic
│   │   ├── scraper_http/ # Scraping basique
│   ├── application/      # Use cases (IntakeOffre, GenerateApplication)
│   └── api/              # Point d'entrée HTTP (Axum)
├── web/                  # Frontend Vanilla JS
└── migrations/           # Schémas SQL (sqlx)
```

## 3. Schéma de Données (PostgreSQL)

La base de données stocke l'intégralité du contexte. Les données locales (fichiers JSON/MD) ne sont plus utilisées comme source de vérité.

- **`offres`** : Stocke l'URL, le hash pour déduplication, le texte brut et le résultat `structured` de l'IA (JSONB).
- **`profils`** : Définit les utilisateurs de la plateforme (mono-utilisateur au lancement).
- **`chunks`** : Les briques de données (expériences, compétences, projets) utilisées pour le RAG. Intègre les vecteurs `embedding (1024)`.
- **`instances`** : Représente une candidature (lien entre Profil et Offre). Stocke les livrables `resume_json` et `cover_letter_json`.
- **`llm_calls`** : Table critique d'observabilité pour tracer chaque requête IA, ses tokens (in/out), son coût et sa latence.

## 4. Pipeline de Génération IA

Le processus pour générer une candidature passe par le use case central `GenerateApplicationUseCase` :

1. **Retrieve** : recherche des chunks de profil les plus pertinents pour l'offre.
2. **Rerank** : le LLM score et filtre les chunks retenus.
3. **Plan** : le LLM construit un plan de candidature.
4. **Restitution** : génération de l'analyse structurée de l'offre.
5. **Resume** : génération du CV structuré.
6. **Cover Letter** : génération de la lettre structurée.
7. **Validate / Persist** : validation métier minimale puis sauvegarde de l'instance en base.

## 5. Le Trait `LlmClient`

Toute l'intégration IA est masquée derrière un trait unique dans `crates/ports/src/llm.rs` :

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Génération texte libre.
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError>;

    /// Génération structurée. On précise un schéma JSON, on récupère un JSON.
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError>;

    fn name(&self) -> &'static str;
}
```

Ce couplage lâche permet de changer de modèle (Claude, Mistral, OpenAI) de manière transparente. Les appels LLM privilégient le "Structured Output" (Tool Use) pour forcer le modèle à renvoyer des données conformes au JSON attendu par le frontend.

## 6. Frontend

Le frontend reste volontairement léger.
- **HTML + JS + CSS statiques** : pas de pipeline de build front dédié dans le repo.
- **Isolation par iframe** : le rendu du CV, de la lettre et de la restitution reste séparé de l'UI principale.
- **Service statique Axum** : le backend sert directement le dossier `web/` et expose l'API REST consommée par cette interface.
