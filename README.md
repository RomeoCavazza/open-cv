# alternance — backend Rust

Builder de candidatures IA-native. Cf. `docs/architecture_rust_v3.md` pour la
vision complète.

## Démarrage rapide

```bash
# 1. Entrer dans l'environnement Nix
nix develop

# 2. Initialiser Postgres local (première fois uniquement)
just db-init

# 3. Démarrer Postgres
just db-up

# 4. Appliquer les migrations
just migrate

# 5. Lancer le serveur
just dev
# ou : cargo run -p api
```

Le serveur écoute sur `http://127.0.0.1:3000` et :

- sert ton `web/` actuel sur `/`
- expose `GET /health` (health check)
- expose `GET /api/offres?limit=20` (liste depuis la DB)
- expose `GET /api/instances/:slug` (instance par slug)

## Structure

```
crates/
├── domain/                # entités pures, zéro dépendance infra
├── ports/                 # traits (LlmClient, OffreRepo, Scraper...)
├── application/           # use cases
├── adapters/
│   ├── postgres/          # impl OffreRepo/InstanceRepo via sqlx
│   ├── llm_claude/        # impl LlmClient via Anthropic API (squelette)
│   └── scraper_http/      # impl Scraper basique HTTP
└── api/                   # binaire Axum
```

## Sqlx en mode online vs offline

Pour l'instant, `sqlx::query!` valide les requêtes à la **compilation**, ce qui
nécessite que la DB soit lancée et migrée pour que `cargo build` réussisse.

Pour passer en mode offline (DB pas requise pour build) :
```bash
cargo sqlx prepare --workspace
git add .sqlx/
```

## Workflow Phase 0

1. ✅ Workspace Cargo, crates vides qui compilent
2. ✅ Postgres + pgvector via Nix
3. ✅ Migration 0001 appliquée
4. ✅ Endpoints `GET /health`, `GET /api/offres`, `GET /api/instances/:slug`
5. ✅ Axum sert `web/` en static

## Workflow Phase 1 (à venir)

- Import des `data/offres/raw/*.md` existants vers la table `offres`
- Import des `data/instances/*` existantes vers `instances`
- Import du profil utilisateur vers `profils` + `chunks` (avec embeddings)

## Workflow Phase 2 (à venir)

- Implémenter `ClaudeClient::complete` et `ClaudeClient::extract`
- `IntakeOffreUseCase` complet
- Panel HTML dans `web/index.html` : champ URL, bouton Analyser

## Tests

```bash
just test          # cargo nextest run
cargo test -p domain  # tests domaine seul (millisecondes)
```
