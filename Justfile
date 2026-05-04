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
    cargo run -p api

# version avec auto-rebuild (nécessite: cargo install cargo-watch)
watch:
    cargo watch -x 'run -p api'

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
    cargo deny check
    cargo udeps --workspace --all-targets
    cargo bloat --release -p api --crates
    tokei .

# prépare le cache sqlx pour la CI (mode offline)
sqlx-prepare:
    cargo sqlx prepare --workspace

# tout (CI-like)
ci: audit test
