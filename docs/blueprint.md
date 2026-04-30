# Architecture — Builder de candidatures IA-native

> Cible : **Rust + Axum + Postgres/pgvector + Claude/Mistral**, scraping et
> génération IA au cœur, RAG fait maison, front statique conservé, montée en
> charge possible sans réécriture.

---

## 1. Postulats explicites

Avant tout choix technique, je pose les hypothèses. Si l'une est fausse, l'archi
change.

| # | Hypothèse                                                                 | Conséquence si fausse                          |
|---|---------------------------------------------------------------------------|------------------------------------------------|
| 1 | Volume : < 100 000 offres, < 10 000 instances générées                    | Si 10x : sharding, pas pgvector seul           |
| 2 | Latence acceptable : 100ms API, 5-30s pour une génération complète        | Si < 100ms génération : on ne fait pas de LLM  |
| 3 | Mode mono-utilisateur (toi) au début, multi-utilisateurs possible plus tard | Si SaaS d'emblée : auth + tenant isolation     |
| 4 | Hébergement : VPS unique ou local. Pas de Kubernetes.                     | Si edge/serverless : pas de Rust+JVM           |
| 5 | Tu acceptes 2-3 mois de courbe Rust pour un gain long terme               | Si livraison < 1 mois : TypeScript+Bun         |

---

## 2. Choix de langage : pourquoi Rust, et contre quoi

| Critère                | Rust+Axum    | TS+Hono/Bun  | Kotlin+Ktor  | Go+Chi       | Python+FastAPI |
|------------------------|--------------|--------------|--------------|--------------|----------------|
| Perf brute             | ★★★★★        | ★★★★         | ★★★          | ★★★★         | ★★             |
| RAM idle               | 15-50 Mo     | 50-100 Mo    | 200-400 Mo   | 30-80 Mo     | 80-150 Mo      |
| Cold start             | < 50 ms      | < 100 ms     | 1-3 s        | < 100 ms     | 500ms-2s       |
| Type system            | ★★★★★        | ★★★★         | ★★★★         | ★★           | ★★★ (mypy)     |
| Élégance OOP-like      | ★★★★ (traits)| ★★★★ (class) | ★★★★★        | ★★           | ★★★            |
| Écosystème scraping    | ★★★★         | ★★★★★        | ★★★          | ★★★★         | ★★★★★          |
| Écosystème LLM/RAG     | ★★★ (jeune)  | ★★★★         | ★★           | ★★★          | ★★★★★          |
| Productivité semaine 1 | ★★           | ★★★★★        | ★★★★         | ★★★★         | ★★★★★          |
| Productivité mois 6    | ★★★★★        | ★★★★         | ★★★★         | ★★★★         | ★★★            |

**Verdict** : Rust gagne sur tes deux priorités déclarées (perf maximale + IA au
cœur). Le seul vrai trou — écosystème LLM jeune — se contourne en appelant les
APIs via HTTP (`reqwest`), ce qui est de toute façon la bonne pratique.

**Ce que Rust te force à faire de bien** :
- séparation domaine / infra impossible à esquiver (lifetimes, traits)
- erreurs explicites (pas de `try/except` qui avale tout)
- pas de mutation cachée (`&mut` est visible)
- compile-time SQL avec sqlx (zéro requête cassée à runtime)

**Ce qui va te coûter** :
- 2-3 semaines de bagarre avec le borrow checker sur de l'async
- choix de bibliothèques à faire toi-même (pas de Django/Rails)
- compilation lente sur gros workspace (`cargo check` reste rapide)

---

## 3. Architecture : hexagonale, en workspace Cargo

```
alternance/
├── crates/
│   ├── domain/           # ← cœur métier, zéro dépendance infra
│   ├── ports/            # ← traits (interfaces) que le domaine exige
│   ├── adapters/
│   │   ├── postgres/     # impl ports::OffreRepo via sqlx
│   │   ├── llm_claude/   # impl ports::LlmClient via API Anthropic
│   │   ├── llm_ollama/   # impl ports::LlmClient via Ollama local
│   │   ├── scraper_http/ # impl ports::Scraper (sites statiques)
│   │   └── scraper_chrome/ # impl ports::Scraper (sites JS, anti-bot)
│   ├── application/      # use cases : IntakeOffre, GenerateApplication...
│   └── api/              # binaire Axum (HTTP, auth, observabilité)
├── web/                  # front actuel, intact
├── data/                 # données existantes (migration progressive vers PG)
├── migrations/           # SQL versionné (sqlx-cli)
├── prompts/              # prompts versionnés en .md, chargés au build
├── flake.nix             # tu utilises déjà Nix
└── Cargo.toml            # workspace
```

**Pourquoi ce découpage et pas un seul crate** :

Le crate `domain` ne dépend ni de tokio, ni de sqlx, ni de reqwest. Il décrit ce
qu'est une `Offre`, une `Instance`, un `ProfilCandidat` — pures structures + règles
métier. Ça veut dire :

1. tu peux remplacer Postgres par SQLite ou DuckDB en touchant **uniquement**
   `adapters/postgres`
2. tu peux switcher de Claude à Mistral à Ollama en touchant **uniquement** un
   adapter LLM
3. les tests du domaine tournent en millisecondes, sans DB ni réseau
4. quand une IA te génère du code "spaghetti FastAPI-style", il a littéralement
   nulle part où aller : la frontière des modules le force à mettre la logique
   au bon endroit

C'est ça, ton "OOP intelligente" : ce n'est pas l'héritage, c'est la **séparation
forcée des préoccupations** par le compilateur.

---

## 4. Schéma de données réel

Pas de "id, created_at" approximatif. Voici le SQL que tu lances réellement.

```sql
-- migrations/0001_init.sql

CREATE EXTENSION IF NOT EXISTS pgcrypto;   -- gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS vector;     -- pgvector
CREATE EXTENSION IF NOT EXISTS pg_trgm;    -- fuzzy match sur titres/entreprises

-- ─────────────────────────────────────────────────────────────────────────
-- OFFRES : une ligne par offre canonique. La dédup se fait à l'intake.
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE offres (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug            TEXT        NOT NULL UNIQUE,           -- ex: safran_alternance_ia_bordes
    source_url      TEXT        NOT NULL,
    source_host     TEXT        NOT NULL,                  -- pass.fr, safran.com...
    source_hash     BYTEA       NOT NULL,                  -- sha256(raw_text) pour dédup
    entreprise      TEXT        NOT NULL,
    intitule        TEXT        NOT NULL,
    localisation    TEXT,
    contrat         TEXT,                                  -- alternance, stage, CDI...
    raw_html        TEXT,
    raw_text        TEXT        NOT NULL,
    structured      JSONB       NOT NULL,                  -- résumé LLM (missions, stack, exigences)
    embedding       vector(1024),                          -- voqu-3 par défaut
    scraped_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at    TIMESTAMPTZ NOT NULL DEFAULT now(),    -- mise à jour à chaque re-scrape
    closed_at       TIMESTAMPTZ,                           -- offre disparue de la source
    UNIQUE (source_host, source_hash)
);

CREATE INDEX offres_entreprise_trgm ON offres USING gin (entreprise gin_trgm_ops);
CREATE INDEX offres_intitule_trgm   ON offres USING gin (intitule gin_trgm_ops);
CREATE INDEX offres_structured_gin  ON offres USING gin (structured jsonb_path_ops);
CREATE INDEX offres_embedding_hnsw  ON offres
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- ─────────────────────────────────────────────────────────────────────────
-- PROFIL : un seul candidat = toi. Versionné pour pouvoir rejouer.
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE profils (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    label           TEXT        NOT NULL,                  -- "v2026-04", "data-focus"...
    content         JSONB       NOT NULL,                  -- profil complet structuré
    is_active       BOOLEAN     NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX profils_one_active ON profils (is_active) WHERE is_active = true;

-- ─────────────────────────────────────────────────────────────────────────
-- CHUNKS : expériences, projets, compétences découpés pour le RAG.
-- ─────────────────────────────────────────────────────────────────────────
CREATE TYPE chunk_kind AS ENUM ('experience', 'projet', 'formation', 'competence', 'phrase_lettre');

CREATE TABLE chunks (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    profil_id       UUID        NOT NULL REFERENCES profils(id) ON DELETE CASCADE,
    kind            chunk_kind  NOT NULL,
    titre           TEXT        NOT NULL,
    content         TEXT        NOT NULL,
    metadata        JSONB       NOT NULL DEFAULT '{}'::jsonb,
    embedding       vector(1024) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX chunks_profil      ON chunks(profil_id);
CREATE INDEX chunks_kind        ON chunks(kind);
CREATE INDEX chunks_embedding   ON chunks USING hnsw (embedding vector_cosine_ops);

-- ─────────────────────────────────────────────────────────────────────────
-- INSTANCES : 1 instance = 1 candidature (offre + CV + lettre + traces)
-- ─────────────────────────────────────────────────────────────────────────
CREATE TYPE instance_status AS ENUM ('draft', 'generating', 'ready', 'sent', 'archived', 'failed');

CREATE TABLE instances (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug            TEXT        NOT NULL UNIQUE,           -- compat avec data/instances/<slug>/
    offre_id        UUID        NOT NULL REFERENCES offres(id) ON DELETE RESTRICT,
    profil_id       UUID        NOT NULL REFERENCES profils(id) ON DELETE RESTRICT,
    status          instance_status NOT NULL DEFAULT 'draft',
    resume_json     JSONB,
    cover_letter_json JSONB,
    notes           JSONB       NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at         TIMESTAMPTZ
);
CREATE INDEX instances_offre  ON instances(offre_id);
CREATE INDEX instances_status ON instances(status);

-- ─────────────────────────────────────────────────────────────────────────
-- LLM_CALLS : traçabilité complète, indispensable pour debug et coûts
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE llm_calls (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id     UUID        REFERENCES instances(id) ON DELETE CASCADE,
    purpose         TEXT        NOT NULL,                  -- 'extract_offre', 'generate_resume'...
    provider        TEXT        NOT NULL,                  -- 'anthropic', 'mistral', 'ollama'
    model           TEXT        NOT NULL,
    prompt_hash     BYTEA       NOT NULL,                  -- pour cache
    prompt          TEXT        NOT NULL,
    response        TEXT,
    tokens_in       INTEGER,
    tokens_out      INTEGER,
    cost_usd        NUMERIC(10,6),
    latency_ms      INTEGER,
    error           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX llm_calls_instance ON llm_calls(instance_id);
CREATE INDEX llm_calls_hash     ON llm_calls(prompt_hash);  -- cache lookup
```

**Ce que ce schéma fait que celui de l'autre doc ne faisait pas** :

- contraintes d'unicité réelles (`source_host, source_hash`) → idempotence du scraping
- HNSW vs IVFFlat tranché (HNSW : meilleur recall, plus de RAM, OK à ton volume)
- traçabilité complète des appels LLM dans `llm_calls` → tu sauras combien tu dépenses, tu pourras debugger, tu pourras rejouer
- cache des prompts par hash → tu ne payes pas deux fois la même requête
- séparation `profils` + `chunks` versionnée → tu peux faire évoluer ton CV sans casser les anciennes instances
- `closed_at` sur les offres → tu sais quand une offre disparaît
- statut explicite sur `instances` (enum), pas un champ texte libre

---

## 5. Stack Rust détaillée, choix argumentés

| Besoin             | Choix              | Pourquoi pas l'alternative                                              |
|--------------------|--------------------|--------------------------------------------------------------------------|
| HTTP framework     | **Axum**           | Actix : OK mais culture toxique. Rocket : trop magique, async tardif.    |
| Runtime async      | **Tokio**          | smol/async-std : moins large écosystème.                                 |
| DB                 | **sqlx**           | sea-orm : trop ORM, perd le SQL type-checked. diesel : sync, vieillit.   |
| Migrations         | **sqlx-cli**       | refinery : OK mais sqlx-cli intégré.                                     |
| Vecteurs           | **pgvector crate** | Qdrant client : ajoute un service ; on attend d'en avoir besoin.         |
| HTTP client        | **reqwest**        | hyper direct : trop bas niveau pour ce cas.                              |
| Scraping HTML      | **scraper**        | select : plus vieux, moins maintenu.                                     |
| Extraction texte   | **trafilatura via cmd ou readability-rs** | readability-lxml côté Python : pas notre langage. |
| Headless browser   | **chromiumoxide**  | thirtyfour : Selenium, plus lourd. fantoccini : moins maintenu.          |
| Sérialisation      | **serde + serde_json** | standard de facto                                                    |
| Erreurs (lib)      | **thiserror**      | structuré, derive macro                                                  |
| Erreurs (binaire)  | **anyhow**         | simple, contexte chaîné                                                  |
| Logs/traces        | **tracing**        | log : trop simple. slog : EOL.                                           |
| Config             | **figment**        | config-rs : OK mais figment plus ergonomique avec serde.                 |
| PDF                | **typst** (CLI)    | headless Chrome : marche mais 200Mo de mémoire. wkhtmltopdf : abandonné. |
| Validation         | **validator**      | garde : moins répandu                                                    |
| Tests intégration  | **testcontainers** | démarre un vrai Postgres jetable                                         |
| LLM clients        | **fait maison via reqwest** | rig : prometteur mais jeune, on peut migrer plus tard.          |

**Sur Typst pour le PDF** : c'est probablement le choix non-évident le plus
intéressant. Typst est un langage de typesetting moderne (successeur spirituel
de LaTeX). Tu écris des templates `.typ`, tu les remplis avec ton JSON, tu
appelles le binaire `typst`. Résultat : PDF parfait, 50ms de génération, pas de
Chrome qui mange 500Mo. Et le langage est *agréable*, contrairement à LaTeX.

---

## 6. Stack IA — les 6 briques que tu utilises VRAIMENT

L'écosystème IA est immense. Sur ce projet, tu n'as besoin que de 6 briques.
Le reste (PyTorch, LangChain, Pinecone, MLflow, Kubernetes, Gradio…) est
culture générale ou hors sujet. La section 9 (anti-patterns) liste ce qui est
explicitement refusé.

### 6.1 Tableau de décision

| Besoin                  | Choix                       | Pourquoi pas l'alternative                                              |
|-------------------------|-----------------------------|--------------------------------------------------------------------------|
| LLM principal           | **Anthropic Claude API**    | OpenAI : OK mais plus cher, structured output moins propre. Gemini : écosystème français moins fluide. |
| LLM secondaire / fallback | **Mistral API**           | Hébergé en France (RGPD), excellent en français, moins cher en gros volumes. Sert aussi de plan B si Anthropic down. |
| Embeddings              | **voyage-3** (Voyage AI)    | OpenAI ada-002 : moins bon en multilingue. Cohere embed-v3 : alternative valable. |
| Vector store            | **pgvector + HNSW**         | Qdrant : excellent (écrit en Rust), mais service de plus à opérer. Pinecone/Weaviate : SaaS payant inutile. ChromaDB : jouet. |
| Orchestration RAG       | **fait maison, ~80 lignes Rust** | LangChain/LlamaIndex : abstractions instables, magie qui se casse en debug. Tu codes `embed → top_k → rerank → prompt` à la main, c'est trivial. |
| Traçabilité             | **table `llm_calls`** (cf. §4) | LangSmith / Helicone : SaaS, vendor lock-in. Postgres suffit. |

### 6.2 Le trait `LlmClient` — l'abstraction qui tient tout

Point central de l'architecture IA. Un seul trait, plusieurs implémentations
interchangeables. Le domaine ne connaît que le trait.

```rust
// crates/ports/src/llm.rs

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Génération texte libre.
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError>;

    /// Génération structurée : on précise un schéma JSON, on récupère du T.
    /// Implémentations : tool_use (Claude), response_format (Mistral).
    async fn extract<T>(&self, req: ExtractionRequest) -> Result<T, LlmError>
    where
        T: DeserializeOwned + JsonSchema + Send;

    /// Embeddings (peut être un autre service, ex: Voyage).
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError>;

    fn name(&self) -> &'static str;
}
```

**Adapters fournis** :
- `crates/adapters/llm_claude` → Anthropic API, `tool_use` pour structured output
- `crates/adapters/llm_mistral` → Mistral API, `response_format: json_schema`
- `crates/adapters/embed_voyage` → Voyage AI (séparable du LLM principal)

**Bénéfice direct** : ton use case `GenerateApplicationUseCase` prend
`Arc<dyn LlmClient>` en paramètre. Tu testes avec un mock, tu déploies avec
Claude, tu peux switcher sur Mistral en changeant **une ligne** dans le wiring
de `main.rs`. Zéro couplage.

### 6.3 Coûts attendus (calcul réaliste)

Avec Claude Sonnet + Voyage embeddings, pour une candidature complète :

| Étape                  | Tokens in | Tokens out | Coût USD |
|------------------------|-----------|------------|----------|
| Extraction offre       | ~3 000    | ~500       | ~$0.020  |
| Embedding offre + query| 1 appel   | -          | ~$0.001  |
| Rerank chunks (12→6)   | ~2 000    | ~200       | ~$0.013  |
| Plan candidature       | ~3 000    | ~400       | ~$0.020  |
| Génération CV          | ~3 500    | ~1 200     | ~$0.029  |
| Génération lettre      | ~3 000    | ~800       | ~$0.022  |
| **Total**              | ~14 500   | ~3 100     | **~$0.10** |

100 candidatures = ~10 USD. Avec cache `llm_calls` activé sur les étapes
déterministes (extraction d'offres déjà vues, embedding d'offres connues),
divise par 2 ou 3. Avec Mistral en fallback, divise encore par 2. **Tu ne
ruines pas.**

Avec Claude Opus, multiplie ces chiffres par ~5 : ~$0.50 par candidature.
Réserve Opus aux étapes où la qualité fait la différence (génération lettre),
laisse Sonnet sur le reste.

---

## 7. Flux d'intake d'une offre — le vrai

L'autre doc disait "POST /api/offers/intake récupère et extrait". Voici ce qui
se passe vraiment, étape par étape, avec les pièges.

```
┌────────────┐
│ Front      │  POST /api/offres { url } ou { raw_text }
└─────┬──────┘
      │
┌─────▼──────────────────────────────────────────────────────────┐
│ application::IntakeOffreUseCase                                │
│                                                                │
│  1. Normalisation URL (canonical, virer ?utm_*, fragments)     │
│  2. Lookup cache : source_host + sha256(url) déjà vu ?         │
│     └─ oui → retourner offre existante (idempotence)           │
│  3. Choix scraper :                                            │
│     ├─ host ∈ liste JS-required → scraper_chrome               │
│     └─ sinon → scraper_http                                    │
│  4. Fetch + extraction texte (trafilatura/readability)         │
│  5. Détection anti-bot : si HTML < 500c ou contient            │
│     "Cloudflare"/"Just a moment" → escalade chrome, sinon fail │
│  6. Hash du raw_text → si déjà en DB sur autre URL :           │
│     fusion (offre republiée), update last_seen_at              │
│  7. Appel LLM "extract_offre" (structured output) :            │
│     entreprise, intitule, localisation, contrat,               │
│     missions[], stack[], exigences[], soft_skills[]            │
│  8. Embedding du texte structuré (voyage-3 ou OpenAI ada)      │
│  9. INSERT offres ... RETURNING                                │
│ 10. Trace dans llm_calls                                       │
└─────┬──────────────────────────────────────────────────────────┘
      │
      ▼
   { offre_id, structured, similar_offres[], suggested_chunks[] }
```

**Pièges concrets que personne ne te dit** :

- **Cloudflare/Datadome** sur LinkedIn, Welcome to the Jungle, Apec : impossibles
  en HTTP brut. `chromiumoxide` aide mais ne suffit pas toujours. Solutions : (a)
  flaresolverr en sidecar, (b) résidentiel proxy si tu deviens sérieux, (c)
  copier-coller manuel dans un panel "raw_text" — *garde toujours cette option*.
- **Pages d'offres qui mutent** : la même URL peut afficher une offre différente
  une semaine plus tard. D'où `last_seen_at` + `closed_at`.
- **Encoding** : les sites français en latin-1 mal déclarés existent encore.
  `reqwest` ne devine pas, tu dois utiliser `encoding_rs`.
- **Robots.txt** : techniquement à respecter. En pratique, un scraping mono-user
  pour ses candidatures n'est pas dans le radar légal, mais tu mets un
  `User-Agent` honnête avec contact.
- **Rate limiting** sortant : `tower::limit::RateLimitLayer` + `governor` pour
  les appels LLM. Sinon premier crash sur 429.
- **Structured output LLM** : avec Claude tu utilises `tool_use` ; avec Mistral,
  `response_format: json_schema` ; avec Ollama, tu pries ou tu utilises
  `outlines`/`jsonformer`. Abstraction = ton trait `LlmClient::extract<T:
  JsonSchema>`.

---

## 8. Flux de génération d'une instance

```
POST /api/instances { offre_id, profil_id }
       │
       ▼
┌────────────────────────────────────────────────────────────┐
│ GenerateApplicationUseCase                                 │
│                                                            │
│  Étape 1 — RETRIEVE                                        │
│  ├─ embed(offre.structured) → query                        │
│  ├─ top-k sur chunks WHERE profil_id = ?                   │
│  │   filtres : kind ∈ {experience, projet, competence}     │
│  └─ rerank LLM : "ces 12 chunks sont pertinents pour       │
│      cette offre ? renvoie les 6 meilleurs avec score"     │
│                                                            │
│  Étape 2 — PLAN (1 appel LLM)                              │
│  └─ "Voici l'offre + ces 6 chunks. Plan en JSON :          │
│      sections du CV à mettre en avant, angle de la lettre, │
│      mots-clés à intégrer"                                 │
│                                                            │
│  Étape 3 — RESUME (1 appel LLM, parallèle avec étape 4)    │
│  └─ structured output → resume.json                        │
│                                                            │
│  Étape 4 — COVER LETTER (1 appel LLM, parallèle)           │
│  └─ structured output → cover-letter.json                  │
│                                                            │
│  Étape 5 — VALIDATE                                        │
│  ├─ schéma JSON                                            │
│  ├─ longueur lettre raisonnable                            │
│  ├─ pas de hallucinations (entreprise/intitulé matchent)   │
│  └─ pas de phrases creuses (heuristique : n-gram check)    │
│                                                            │
│  Étape 6 — PERSIST                                         │
│  ├─ UPDATE instance SET resume_json=, cover_letter_json=,  │
│  │   status='ready'                                        │
│  └─ écriture miroir data/instances/<slug>/*.json           │
│                                                            │
│  Étape 7 — RETURN { instance_id, slug, preview_url }       │
└────────────────────────────────────────────────────────────┘
```

**Pourquoi ces 7 étapes et pas un seul mégaprompt** :

1. **Coût** : un retrieve + plan court + 2 générations cible = ~6k tokens. Un
   mégaprompt avec tout le profil + offre = 20k+ tokens. ×3 sur ta facture.
2. **Qualité** : un LLM qui doit générer 1500 mots structurés en un appel est
   pire que deux LLMs qui font 750 mots chacun avec contexte ciblé.
3. **Debug** : quand la lettre est mauvaise, tu sais à quelle étape regarder.
4. **Reprise** : si étape 4 échoue, tu reprends de l'étape 4, pas de zéro.
5. **Parallélisme** : étapes 3 et 4 tournent en parallèle (`tokio::join!`).

---

## 9. Anti-patterns que je refuse, et pourquoi

| Tentation                                  | Pourquoi non                                                 |
|--------------------------------------------|--------------------------------------------------------------|
| LangChain / LangGraph / LlamaIndex / AutoGen | abstractions qui changent tous les 3 mois, magie qui se casse en debug. Tu codes 80 lignes Rust pour ce dont tu as besoin (cf. §6.2). |
| Gradio / Streamlit                         | tu as déjà un front HTML/CSS/JS plus joli et plus contrôlable                |
| PyTorch / TensorFlow / JAX                 | tu n'entraînes rien, tu consommes des LLMs via API                           |
| Pinecone / Weaviate / Milvus / ChromaDB    | pgvector + HNSW couvre tes besoins ; un service de moins à opérer            |
| MLflow / W&B / DVC / Ray                   | pas d'entraînement à tracker ; `llm_calls` suffit pour la traçabilité d'inférence |
| Microservices                              | un seul utilisateur, un seul VPS — overhead pur                |
| Redis pour cache                           | Postgres `unlogged table` ou `llm_calls` indexé fait le job   |
| Kafka / queues                             | un `tokio::spawn` + table `jobs` SQL suffit jusqu'à 10 req/s  |
| Qdrant dès le début                        | pgvector + HNSW tient à ton volume, on ajoute un service quand on mesure un problème |
| ORM lourd (SeaORM, Diesel)                 | sqlx + queries explicites, type-checked à la compile         |
| GraphQL                                    | ton front c'est toi, REST suffit largement                   |
| Auth dès le jour 1                         | mode local, on ajoute middleware quand on déploie            |
| Tests E2E avant tests d'intégration        | priorité : tests domaine + tests adapter Postgres + 1-2 E2E   |
| Mégaprompt qui fait tout                   | cf. section 7                                                |
| Re-scraper à chaque vue d'offre            | dédup par hash, `last_seen_at`, refresh manuel/cron          |

---

## 10. Observabilité minimale dès le jour 1

Pas négociable, même en mono-user.

- `tracing` partout, format JSON en prod, pretty en dev
- middleware Axum qui injecte `request_id` dans chaque span
- chaque appel LLM logué avec `purpose`, `tokens_in`, `tokens_out`, `cost_usd`,
  `latency_ms` dans `llm_calls`
- endpoint `/metrics` Prometheus (axum-prometheus, 20 lignes)
- une vue SQL `v_llm_costs_daily` que tu consultes une fois par semaine

```sql
CREATE VIEW v_llm_costs_daily AS
SELECT
    date_trunc('day', created_at) AS jour,
    provider,
    model,
    purpose,
    COUNT(*)              AS nb_calls,
    SUM(tokens_in)        AS tokens_in,
    SUM(tokens_out)       AS tokens_out,
    SUM(cost_usd)         AS cost_usd,
    AVG(latency_ms)::INT  AS avg_latency_ms
FROM llm_calls
GROUP BY 1, 2, 3, 4
ORDER BY 1 DESC;
```

Tu vas vouloir cette vue. Crois-moi.

---

## 11. Phasage réaliste

**Phase 0 — semaine 1**
- workspace Cargo, crates vides, flake.nix qui marche
- Postgres + pgvector via docker-compose (ou nix)
- migration 0001 appliquée
- endpoint `GET /health` et `GET /api/offres` qui lit la DB
- Axum sert ton `web/` actuel sur `/`

**Phase 1 — semaines 2-3**
- import des `data/offres/raw/*.md` existants vers la table `offres`
- import des `data/instances/*` existantes vers `instances`
- import du profil utilisateur vers `profils` + `chunks` (avec embeddings)
- endpoint `GET /api/instances/:slug` qui sert le JSON depuis la DB
  (le front ne change pas)

**Phase 2 — semaines 4-5**
- adapter `llm_claude` (ou Mistral, selon ton compte API)
- use case `IntakeOffreUseCase` avec scraper HTTP simple
- panel dans `web/index.html` : champ URL, bouton Analyser
- structured output Claude → `offres.structured`

**Phase 3 — semaines 6-7**
- `GenerateApplicationUseCase` complet
- retrieve + rerank + plan + génération parallèle
- validation schéma + heuristiques anti-hallucination

**Phase 4 — au besoin seulement**
- `scraper_chrome` quand un site bloque
- adapter `llm_ollama` si tu veux du local
- export PDF via Typst
- desktop app via Tauri (réutilise tout le backend Rust)

---

## 12. Ce que tu peux faire ce soir

1. `cargo new --lib crates/domain` et écrire les structs `Offre`, `Profil`, `Instance` sans aucune dépendance
2. `flake.nix` qui fournit `rustc`, `cargo`, `sqlx-cli`, `postgresql_16`, `typst`
3. `migrations/0001_init.sql` (copier la section 4)
4. `cargo add axum tokio sqlx --features postgres` dans un nouveau crate `api`
5. Un endpoint `GET /api/offres` qui SELECT et renvoie du JSON

Tu auras un truc qui tourne en 2-3 heures. Pas de magie, pas de slop, du code
que tu lis et que tu comprends entièrement.

---

## 13. Quand abandonner Rust

Sois honnête avec toi-même. Si après 3 semaines :

- tu te bats encore avec le borrow checker sur du basique
- tu n'as pas livré un seul endpoint qui marche bout-en-bout
- tu copies-colles du code IA sans le comprendre
- tu n'as plus envie d'ouvrir le repo

Alors tu pivotes vers **TypeScript strict + Bun + Hono + Drizzle + pgvector**.
Tu gardes le même schéma SQL, la même architecture hexagonale (TS la supporte
très bien), et tu livres. L'élégance d'un projet fini bat l'élégance d'un
projet abandonné.

Mais essaie d'abord. Le Rust qui clique, ça change un développeur.