set shell := ["bash", "-cu"]

# default : afficher les commandes
default:
    @just --list

# initialise un Postgres local dans .pg/ (première fois uniquement)
db-init:
    mkdir -p .pg
    initdb -D .pg --auth=trust --no-locale --encoding=UTF8 -U alternance
    echo "unix_socket_directories = '$PWD/.pg'" >> .pg/postgresql.conf
    echo "listen_addresses = 'localhost'" >> .pg/postgresql.conf
    pg_ctl -D .pg -l .pg/postgres.log start
    sleep 1
    createdb -h localhost -U alternance alternance
    psql -h localhost -U alternance -d alternance -c "CREATE EXTENSION IF NOT EXISTS vector;"
    pg_ctl -D .pg stop

# démarre Postgres
db-up:
    pg_ctl -D .pg -l .pg/postgres.log start

# vide les tables applicatives locales
db-reset:
    pg_ctl -D .pg status >/dev/null 2>&1 || just db-up
    psql -h localhost -U alternance -d alternance -c "TRUNCATE TABLE annexes, messages, llm_calls, instances, chunks, profils, offres RESTART IDENTITY CASCADE;"

# arrête Postgres
db-down:
    pg_ctl -D .pg stop

# applique les migrations
migrate:
    sqlx migrate run

# psql shell
psql:
    psql -h localhost -U alternance -d alternance

# lance l'api en mode développement
dev:
    cargo run -p api --bin api

# crée un profil vierge
seed-blank:
    cargo run -p api --bin seed_blank

# remplit le profil actif depuis data/user
seed-profile:
    cargo run -p api --bin seed_profile

# remplit les offres et instances depuis data/offres et data/instances
seed-data:
    cargo run -p api --bin seed_offers_instances

# remplit le profil puis les offres/instances
seed-all: seed-profile seed-data

# version avec auto-rebuild (nécessite: cargo install cargo-watch)
watch:
    cargo watch -x 'run -p api --bin api'

# tests
test:
    cargo nextest run --workspace

# vérification rapide (sans build)
check:
    cargo check --workspace --all-targets

# lint
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# format
fmt:
    cargo fmt --all

# audit frontend
audit-frontend:
    find web -name '*.js' -print0 | while IFS= read -r -d '' file; do node --input-type=module --check < "$file"; done
    ! rg -n "innerHTML\s*=\s*[^'\"[:space:]]|document.write|eval\(" web
    tokei web

# audit complet
audit:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo deny check --config tooling/deny.toml
    cargo udeps --workspace --all-targets || true
    cargo bloat --release -p api --crates
    [ -f package.json ] && npm run lint:js || true
    [ -f package.json ] && npm run lint:css || true
    tokei .

# couverture de code (nécessite: cargo install cargo-tarpaulin)
coverage:
    cargo tarpaulin --config tooling/tarpaulin.toml

# benchmarks (nécessite: cargo install cargo-criterion)
bench:
    cargo criterion

# visualisation d'architecture (nécessite: cargo install cargo-modules)
viz-modules:
    cargo modules generate graph | dot -Tsvg > docs/modules.svg

# graphe de dépendances (nécessite: cargo install cargo-depgraph)
viz-deps:
    cargo depgraph --workspace --all-deps | dot -Tsvg > docs/deps.svg

# profilage CPU (nécessite: cargo install cargo-flamegraph)
flamegraph:
    cargo flamegraph --bin api

# tout (CI-like)
ci: audit test
