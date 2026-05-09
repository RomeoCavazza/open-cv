# Rapport de Sante RecruitAI
Généré le : 2026-05-09 04:12:00

## 1. Statistiques du Code (Tokei)
```
```

## 2. Architecture du Projet (Eza)
```
.
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── adapters
│   │   ├── llm_claude
│   │   ├── llm_ollama
│   │   ├── llm_openai
│   │   ├── postgres
│   │   └── scraper_http
│   ├── api
│   │   ├── Cargo.toml
│   │   ├── src
│   │   └── tests
│   ├── application
│   │   ├── Cargo.toml
│   │   ├── src
│   │   └── tests
│   ├── domain
│   │   ├── Cargo.toml
│   │   └── src
│   └── ports
│       ├── Cargo.toml
│       └── src
├── docs
│   ├── assets
│   │   ├── canva.png
│   │   ├── modules.svg
│   │   ├── preview-cover-letter.png
│   │   ├── preview-restitution.png
│   │   └── preview-resume.png
│   ├── audit.md
│   ├── blueprint.md
│   ├── data_management.md
│   ├── design.md
│   ├── instructions.md
│   ├── project_map.md
│   ├── README.md
│   └── toolkit.md
├── eslint.config.js
├── flake.lock
├── flake.nix
├── Justfile
├── migrations
│   └── 0001_initial.sql
├── package.json
├── README.md
├── rust-toolchain.toml
├── stylelint.config.js
├── tooling
│   ├── deny.toml
│   ├── eslint.config.js
│   ├── health_report.md
│   ├── knip.json
│   ├── sonar-project.properties
│   ├── stylelint.config.js
│   └── tarpaulin.toml
└── web
    ├── assets
    │   ├── css
    │   ├── js
    │   ├── sounds
    │   └── templates
    ├── cover-letter
    │   ├── index.html
    │   ├── script.js
    │   └── style.css
    ├── index.html
    ├── restitution
    │   └── index.html
    └── resume
        ├── assets
        ├── index.html
        ├── script.js
        └── style.css
```

## 3. Securite et Dependances
### Audit Cargo
```
```

### Doublons de dependances
```
base64 v0.22.1
├── adapter-llm-claude v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_claude)
│   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama)
│   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── adapter-llm-openai v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_openai)
│   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application)
│   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── hyper-util v0.1.20
│   ├── axum v0.7.9
│   │   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   ├── hyper-rustls v0.27.9
│   │   └── reqwest v0.12.28
│   │       ├── adapter-llm-claude v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_claude) (*)
│   │       ├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama) (*)
│   │       ├── adapter-llm-openai v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_openai) (*)
│   │       └── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http)
│   │           └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   └── reqwest v0.12.28 (*)
├── reqwest v0.12.28 (*)
├── sqlx-core v0.8.6
│   ├── sqlx v0.8.6
│   │   ├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres)
│   │   │   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   │   └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   └── sqlx-postgres v0.8.6
│       └── sqlx v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

base64 v0.22.1
├── sqlx-core v0.8.6
│   ├── sqlx-macros v0.8.6 (proc-macro)
│   │   └── sqlx v0.8.6 (*)
│   ├── sqlx-macros-core v0.8.6
│   │   └── sqlx-macros v0.8.6 (proc-macro) (*)
│   └── sqlx-postgres v0.8.6
│       └── sqlx-macros-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

chrono v0.4.44
├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres) (*)
├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
├── domain v0.1.0 (/home/tco/Bureau/alternance/crates/domain)
│   ├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres) (*)
│   ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│   └── ports v0.1.0 (/home/tco/Bureau/alternance/crates/ports)
│       ├── adapter-llm-claude v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_claude) (*)
│       ├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama) (*)
│       ├── adapter-llm-openai v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_openai) (*)
│       ├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres) (*)
│       ├── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http) (*)
│       ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│       └── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
├── ports v0.1.0 (/home/tco/Bureau/alternance/crates/ports) (*)
├── schemars v0.8.22
│   ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│   └── domain v0.1.0 (/home/tco/Bureau/alternance/crates/domain) (*)
├── sqlx-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

chrono v0.4.44
├── sqlx-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

futures-channel v0.3.32
├── futures v0.3.32
│   ├── adapter-llm-claude v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_claude) (*)
│   ├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama) (*)
│   ├── adapter-llm-openai v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_openai) (*)
│   ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│   └── ports v0.1.0 (/home/tco/Bureau/alternance/crates/ports) (*)
├── futures-util v0.3.32
│   ├── axum v0.7.9 (*)
│   ├── axum-core v0.4.5
│   │   └── axum v0.7.9 (*)
│   ├── futures v0.3.32 (*)
│   ├── futures-executor v0.3.32
│   │   └── futures v0.3.32 (*)
│   ├── hyper-util v0.1.20 (*)
│   ├── reqwest v0.12.28 (*)
│   ├── sqlx-core v0.8.6 (*)
│   ├── sqlx-postgres v0.8.6 (*)
│   ├── tower v0.5.3
│   │   ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   │   ├── axum v0.7.9 (*)
│   │   ├── reqwest v0.12.28 (*)
│   │   └── tower-http v0.6.10
│   │       ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   │       └── reqwest v0.12.28 (*)
│   └── tower-http v0.6.10 (*)
├── hyper v1.9.0
│   ├── axum v0.7.9 (*)
│   ├── hyper-rustls v0.27.9 (*)
│   ├── hyper-util v0.1.20 (*)
│   └── reqwest v0.12.28 (*)
├── hyper-util v0.1.20 (*)
└── sqlx-postgres v0.8.6 (*)

futures-channel v0.3.32
└── sqlx-postgres v0.8.6 (*)

futures-sink v0.3.32
├── futures-channel v0.3.32 (*)
└── futures-util v0.3.32
    ├── sqlx-core v0.8.6 (*)
    └── sqlx-postgres v0.8.6 (*)

futures-sink v0.3.32
├── futures v0.3.32 (*)
├── futures-channel v0.3.32 (*)
├── futures-util v0.3.32 (*)
└── tokio-util v0.7.18
    ├── reqwest v0.12.28 (*)
    └── tower-http v0.6.10 (*)

futures-util v0.3.32 (*)

futures-util v0.3.32 (*)

getrandom v0.2.17
├── rand_core v0.6.4
│   ├── rand v0.8.6
│   │   ├── sqlx-postgres v0.8.6 (*)
│   │   └── sqlx-postgres v0.8.6 (*)
│   └── rand_chacha v0.3.1
│       └── rand v0.8.6 (*)
└── ring v0.17.14
    ├── rustls v0.23.40
    │   ├── hyper-rustls v0.27.9 (*)
    │   ├── reqwest v0.12.28 (*)
    │   ├── sqlx-core v0.8.6 (*)
    │   ├── sqlx-core v0.8.6 (*)
    │   └── tokio-rustls v0.26.4
    │       ├── hyper-rustls v0.27.9 (*)
    │       └── reqwest v0.12.28 (*)
    └── rustls-webpki v0.103.13
        └── rustls v0.23.40 (*)

getrandom v0.4.2
└── uuid v1.23.1
    ├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres) (*)
    ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
    ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
    ├── domain v0.1.0 (/home/tco/Bureau/alternance/crates/domain) (*)
    ├── schemars v0.8.22 (*)
    ├── sqlx-core v0.8.6 (*)
    └── sqlx-postgres v0.8.6 (*)

hashbrown v0.15.5
├── hashlink v0.10.0
│   ├── sqlx-core v0.8.6 (*)
│   └── sqlx-core v0.8.6 (*)
├── sqlx-core v0.8.6 (*)
└── sqlx-core v0.8.6 (*)

hashbrown v0.17.0
└── indexmap v2.14.0
    ├── sqlx-core v0.8.6 (*)
    └── sqlx-core v0.8.6 (*)

log v0.4.29
├── sqlx-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

log v0.4.29
├── html5ever v0.39.0
│   └── scraper v0.26.0
│       └── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http) (*)
├── markup5ever v0.39.0
│   └── html5ever v0.39.0 (*)
├── reqwest v0.12.28 (*)
├── selectors v0.36.1
│   └── scraper v0.26.0 (*)
├── sqlx-core v0.8.6 (*)
├── sqlx-postgres v0.8.6 (*)
├── tracing v0.1.44
│   ├── adapter-llm-claude v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_claude) (*)
│   ├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama) (*)
│   ├── adapter-llm-openai v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_openai) (*)
│   ├── adapter-postgres v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/postgres) (*)
│   ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│   ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│   ├── axum v0.7.9 (*)
│   ├── axum-core v0.4.5 (*)
│   ├── hyper-util v0.1.20 (*)
│   ├── sqlx-core v0.8.6 (*)
│   ├── sqlx-core v0.8.6 (*)
│   ├── sqlx-postgres v0.8.6 (*)
│   ├── sqlx-postgres v0.8.6 (*)
│   ├── tower v0.5.3 (*)
│   ├── tower-http v0.6.10 (*)
│   └── tracing-subscriber v0.3.23
│       └── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
└── tracing-log v0.2.0
    └── tracing-subscriber v0.3.23 (*)

smallvec v1.15.1
├── cssparser v0.36.0
│   ├── scraper v0.26.0 (*)
│   └── selectors v0.36.1 (*)
├── hyper v1.9.0 (*)
├── icu_normalizer v2.2.0
│   └── idna_adapter v1.2.2
│       └── idna v1.1.0
│           └── url v2.5.8
│               ├── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http) (*)
│               ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│               ├── reqwest v0.12.28 (*)
│               ├── sqlx-core v0.8.6 (*)
│               ├── sqlx-core v0.8.6 (*)
│               ├── sqlx-macros-core v0.8.6 (*)
│               └── tower-http v0.6.10 (*)
├── idna v1.1.0 (*)
├── parking_lot_core v0.9.12
│   └── parking_lot v0.12.5
│       ├── futures-intrusive v0.5.0
│       │   ├── sqlx-core v0.8.6 (*)
│       │   └── sqlx-core v0.8.6 (*)
│       ├── string_cache v0.9.0
│       │   └── web_atoms v0.2.4
│       │       └── markup5ever v0.39.0 (*)
│       └── tokio v1.52.2
│           ├── api v0.1.0 (/home/tco/Bureau/alternance/crates/api)
│           ├── application v0.1.0 (/home/tco/Bureau/alternance/crates/application) (*)
│           ├── axum v0.7.9 (*)
│           ├── hyper v1.9.0 (*)
│           ├── hyper-rustls v0.27.9 (*)
│           ├── hyper-util v0.1.20 (*)
│           ├── reqwest v0.12.28 (*)
│           ├── sqlx-core v0.8.6 (*)
│           ├── tokio-rustls v0.26.4 (*)
│           ├── tokio-stream v0.1.18
│           │   ├── sqlx-core v0.8.6 (*)
│           │   └── sqlx-core v0.8.6 (*)
│           ├── tokio-util v0.7.18 (*)
│           ├── tower v0.5.3 (*)
│           └── tower-http v0.6.10 (*)
├── selectors v0.36.1 (*)
├── sqlx-core v0.8.6 (*)
├── sqlx-postgres v0.8.6 (*)
└── tracing-subscriber v0.3.23 (*)

smallvec v1.15.1
├── sqlx-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

sqlx-core v0.8.6 (*)

sqlx-core v0.8.6 (*)

sqlx-postgres v0.8.6 (*)

sqlx-postgres v0.8.6 (*)

tokio v1.52.2 (*)

tokio v1.52.2
├── sqlx-core v0.8.6 (*)
└── sqlx-macros-core v0.8.6 (*)

uuid v1.23.1 (*)

uuid v1.23.1
├── sqlx-core v0.8.6 (*)
└── sqlx-postgres v0.8.6 (*)

webpki-roots v0.26.11
├── sqlx-core v0.8.6 (*)
└── sqlx-core v0.8.6 (*)

webpki-roots v1.0.7
├── hyper-rustls v0.27.9 (*)
├── reqwest v0.12.28 (*)
└── webpki-roots v0.26.11 (*)
```

## 4. Poids et Optimisation
### Analyse du binaire (Cargo Bloat)
```
 File  .text     Size Crate
16.5%  30.3%   1.0MiB std
 5.6%  10.2% 355.6KiB rustls
 4.8%   8.7% 304.4KiB sqlx_postgres
 4.1%   7.5% 262.3KiB ring
 3.2%   5.8% 202.7KiB sqlx_core
 2.4%   4.3% 150.1KiB regex_syntax
 2.1%   3.9% 136.0KiB tokio
 2.0%   3.6% 126.3KiB regex_automata
 1.8%   3.4% 117.5KiB adapter_postgres
 1.5%   2.8%  98.5KiB seed_offers_instances
 1.3%   2.4%  84.0KiB serde_json
 1.1%   2.0%  70.0KiB tracing_subscriber
 1.1%   2.0%  68.2KiB webpki
 0.7%   1.2%  43.3KiB url
 0.6%   1.1%  39.0KiB [Unknown]
 0.5%   1.0%  34.7KiB chrono
 0.5%   0.8%  28.7KiB idna
 0.3%   0.5%  18.3KiB tracing
 0.2%   0.5%  15.8KiB anyhow
 0.2%   0.4%  14.5KiB serde
 3.4%   6.2% 214.7KiB And 53 more crates. Use -n N to show more.
54.6% 100.0%   3.4MiB .text section size, the file size is 6.2MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

## 5. Nettoyage et Code Mort (Knip)
```
```

---
