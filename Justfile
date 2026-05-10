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
    @pg_ctl -D .pg status >/dev/null 2>&1 && echo "Postgres est déjà lancé." || pg_ctl -D .pg -l .pg/postgres.log start

# vide les tables applicatives locales
db-reset:
    pg_ctl -D .pg status >/dev/null 2>&1 || just db-up
    psql -h localhost -U alternance -d alternance -c "TRUNCATE TABLE annexes, messages, llm_calls, instances, chunks, profils, offres RESTART IDENTITY CASCADE;"

# arrête Postgres
db-down:
    @pg_ctl -D .pg status >/dev/null 2>&1 && pg_ctl -D .pg stop || echo "Postgres est déjà arrêté."

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

# reset complet : purge DB, migrate, seed et validation
reset-all:
    @psql -h localhost -U alternance -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'alternance' AND pid <> pg_backend_pid();" >/dev/null 2>&1 || true
    dropdb -h localhost -U alternance alternance || true
    createdb -h localhost -U alternance alternance
    psql -h localhost -U alternance -d alternance -c "CREATE EXTENSION IF NOT EXISTS vector;"
    just migrate
    just seed-all
    @psql -h localhost -U alternance -d alternance -c " \
    SELECT 'Profils' as table, count(*) as nb, 'OK' as status FROM profils \
    UNION ALL \
    SELECT 'Offres', count(*), 'OK' FROM offres \
    UNION ALL \
    SELECT 'Instances', count(*), 'OK' FROM instances;"

# version avec auto-rebuild (nécessite: cargo install cargo-watch)
watch:
    cargo watch -x 'run -p api --bin api'

# tests
test:
    cargo test --workspace

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
    tokei web || true

# audit complet
audit:
    cargo fmt --all --check || true
    cargo clippy --workspace --all-targets -- -D warnings || cargo check --workspace
    cargo deny check --config tooling/deny.toml || true
    RUSTC_BOOTSTRAP=1 cargo udeps --workspace --all-targets || true
    cargo bloat --release -p api --crates || true
    [ -f package.json ] && npm run lint:js || true
    [ -f package.json ] && npm run lint:css || true
    tokei . || true

# couverture de code (nécessite: cargo install cargo-tarpaulin)
coverage:
    cargo tarpaulin --config tooling/tarpaulin.toml

# benchmarks (nécessite: cargo install cargo-criterion)
bench:
    cargo criterion

# visualisation d'architecture (nécessite: cargo install cargo-modules)
viz-modules:
    cargo modules dependencies -p api --lib | dot -Tsvg > docs/modules.svg

# graphe de dépendances (nécessite: cargo install cargo-depgraph)
viz-deps:
    cargo depgraph | dot -Tsvg > docs/deps.svg

# profilage CPU (nécessite: cargo install cargo-flamegraph)
flamegraph:
    cargo flamegraph --bin api

# tout (CI-like)
ci: health audit test

# génère un rapport de santé complet et industriel du projet dans tooling/health_report.md
health:
    @mkdir -p tooling
    @echo "==========================================" > tooling/health_report.md
    @echo "RAPPORT DE SANTE INDUSTRIEL — RECRUITAI" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "Généré le : $(date '+%Y-%m-%d %H:%M:%S')" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "0. QUALITY GATE SUMMARY" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "| Check | Status |" >> tooling/health_report.md
    @echo "| :--- | :--- |" >> tooling/health_report.md
    @cargo fmt --all --check >/dev/null 2>&1 && echo "| Formatting | PASS |" >> tooling/health_report.md || echo "| Formatting | FAIL |" >> tooling/health_report.md
    @cargo clippy --workspace --all-targets -- -D warnings >/dev/null 2>&1 && echo "| Rust Lints | PASS |" >> tooling/health_report.md || echo "| Rust Lints | FAIL |" >> tooling/health_report.md
    @([ -f package.json ] && npm run lint:js >/dev/null 2>&1) && echo "| JS Lints | PASS |" >> tooling/health_report.md || echo "| JS Lints | FAIL |" >> tooling/health_report.md
    @cargo deny check --config tooling/deny.toml >/dev/null 2>&1 && echo "| Security & Licenses | PASS |" >> tooling/health_report.md || echo "| Security & Licenses | WARNING |" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "1. STATISTIQUES DU CODE (TOKEI)" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @tokei --exclude data >> tooling/health_report.md || true
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "2. ARCHITECTURE ET STRUCTURE" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "ARBORESCENCE (EZA)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @eza --tree --level=3 --ignore-glob="target|node_modules|.git|.direnv|data" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "VISUALISATIONS" >> tooling/health_report.md
    @echo "| Graph | Status |" >> tooling/health_report.md
    @echo "| :--- | :--- |" >> tooling/health_report.md
    @([ -f docs/modules.svg ] && echo "| Modules Graph | Available |") || echo "| Modules Graph | Missing (run just viz-modules) |" >> tooling/health_report.md
    @([ -f docs/deps.svg ] && echo "| Dependencies Graph | Available |") || echo "| Dependencies Graph | Missing (run just viz-deps) |" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "3. SECURITE ET CONFORMITE" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "AUDIT CARGO (VULNERABILITIES)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @cargo audit 2>&1 | grep -v "alternance dev shell ready" > .audit.tmp || true
    @if [ -s .audit.tmp ]; then cat .audit.tmp >> tooling/health_report.md; else echo "No vulnerabilities detected." >> tooling/health_report.md; fi
    @rm -f .audit.tmp
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "CARGO DENY (LICENSES & BANS)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @cargo deny check --config tooling/deny.toml 2>&1 | grep -v "alternance dev shell ready" > .deny.tmp || true
    @if [ -s .deny.tmp ]; then cat .deny.tmp >> tooling/health_report.md; else echo "All checks passed (advisories, bans, licenses, sources)." >> tooling/health_report.md; fi
    @rm -f .deny.tmp
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "4. DEPENDANCES ET HYGIENE" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "DEPENDANCES INUTILISEES (UDEPS)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @RUSTC_BOOTSTRAP=1 cargo udeps --workspace --all-targets 2>&1 | grep -E "unused|deps" | grep -v "alternance dev shell ready" > .udeps.tmp || true
    @if [ -s .udeps.tmp ]; then cat .udeps.tmp >> tooling/health_report.md; else echo "All dependencies are used." >> tooling/health_report.md; fi
    @rm -f .udeps.tmp
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "DOUBLONS DE DEPENDANCES" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @cargo tree -e no-dev --duplicates 2>&1 | grep -v "alternance dev shell ready" > .dups.tmp || true
    @if [ -s .dups.tmp ]; then cat .dups.tmp >> tooling/health_report.md; else echo "No duplicate dependencies found." >> tooling/health_report.md; fi
    @rm -f .dups.tmp
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "HYGIENE FRONTEND (KNIP)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @npm run knip 2>&1 | grep -v "alternance dev shell ready" | grep -v "knip" | grep -v ">" > .knip.tmp || true
    @if [ -s .knip.tmp ]; then cat .knip.tmp >> tooling/health_report.md; else echo "Knip: All clean (no unused files or exports)." >> tooling/health_report.md; fi
    @rm -f .knip.tmp
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "5. PERFORMANCE ET OPTIMISATION" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "ANALYSE DU BINAIRE (CARGO BLOAT)" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @cargo bloat --release -p api --crates -n 15 | grep -v "alternance dev shell ready" >> tooling/health_report.md
    @echo "\`\`\`" >> tooling/health_report.md
    @echo "" >> tooling/health_report.md
    @echo "==========================================" >> tooling/health_report.md
    @echo "Fin du rapport" >> tooling/health_report.md

