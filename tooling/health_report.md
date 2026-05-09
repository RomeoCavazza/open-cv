# Rapport de Sante RecruitAI
Généré le : 2026-05-09 02:24:49

## 1. Statistiques du Code (Tokei)
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 CSS                       3         1498         1313           38          147
 JavaScript               26         3195         2703           92          400
 JSON                      2           30           30            0            0
 Just                      1          173          118           27           28
 Nix                       1           80           63            6           11
 SQL                       1          244          195           28           21
 SVG                       1         4902         4223          679            0
 TOML                     13          349          295           17           37
─────────────────────────────────────────────────────────────────────────────────
 HTML                      5         1015          957            4           54
 |- CSS                    4           65           59            0            6
 |- JavaScript             2          138          129            1            8
 (Total)                             1218         1145            5           68
─────────────────────────────────────────────────────────────────────────────────
 Markdown                 10          758            0          591          167
 |- BASH                   4           48           30           11            7
 (Total)                              806           30          602          174
─────────────────────────────────────────────────────────────────────────────────
 Rust                     68         9489         8405           53         1031
 |- Markdown              28          155            0          142           13
 (Total)                             9644         8405          195         1044
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                   131        22139        18520         1689         1930
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
 File  .text      Size Crate
10.0%  28.9% 1018.4KiB std
 3.5%  10.3%  361.5KiB rustls
 3.0%   8.6%  302.3KiB sqlx_postgres
 2.9%   8.3%  292.3KiB ring
 2.0%   5.7%  201.8KiB sqlx_core
 1.5%   4.3%  153.0KiB regex_syntax
 1.3%   3.9%  135.9KiB tokio
 1.2%   3.5%  124.6KiB regex_automata
 1.2%   3.5%  122.7KiB adapter_postgres
 1.0%   2.9%  101.6KiB seed_offers_instances
 0.7%   2.0%   70.8KiB [Unknown]
 0.7%   2.0%   69.7KiB tracing_subscriber
 0.7%   1.9%   67.9KiB webpki
 0.6%   1.9%   65.8KiB serde_json
 0.4%   1.3%   44.2KiB url
 0.3%   1.0%   34.1KiB chrono
 0.3%   0.8%   29.4KiB domain
 0.3%   0.8%   28.9KiB idna
 0.2%   0.5%   18.3KiB tracing
 0.1%   0.4%   15.2KiB anyhow
 2.2%   6.4%  225.5KiB And 53 more crates. Use -n N to show more.
34.6% 100.0%    3.4MiB .text section size, the file size is 10.0MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

## 5. Nettoyage et Code Mort (Knip)
```
```

---
