==========================================
RAPPORT DE SANTE INDUSTRIEL — RECRUITAI
==========================================
Généré le : 2026-05-17 00:33:31

==========================================
0. QUALITY GATE SUMMARY
==========================================
| Check | Status |
| :--- | :--- |
| Formatting | FAIL |
| Rust Lints | PASS |
| JS Lints | PASS |
| Security & Licenses | PASS |

==========================================
1. STATISTIQUES DU CODE (TOKEI)
==========================================
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 CSS                       3         1961         1761           36          164
 JavaScript               28         4890         4289          125          476
 JSON                      2           30           30            0            0
 Just                      1          224          169           27           28
 Nix                       1           87           70            6           11
 SQL                       1          281          226           30           25
 SVG                       2         9692         8212         1480            0
 TOML                     13          354          300           17           37
─────────────────────────────────────────────────────────────────────────────────
 HTML                      5         1050         1012            1           37
 |- CSS                    2           47           43            0            4
 |- JavaScript             1          146          130            4           12
 (Total)                             1243         1185            5           53
─────────────────────────────────────────────────────────────────────────────────
 Markdown                 10          987            0          768          219
 |- BASH                   5           40           28            8            4
 |- CSS                    1           12           11            0            1
 |- JavaScript             1           25           22            1            2
 |- JSON                   1           67           67            0            0
 |- Rust                   1           28           23            1            4
 |- TOML                   1           14           12            0            2
 (Total)                             1173          163          778          232
─────────────────────────────────────────────────────────────────────────────────
 Rust                     73        14690        13136           95         1459
 |- Markdown              32          201            0          186           15
 (Total)                            14891        13136          281         1474
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                   139        34826        29541         2785         2500
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

==========================================
2. ARCHITECTURE ET STRUCTURE
==========================================
ARBORESCENCE (EZA)
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
│   │   ├── deps.svg
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

VISUALISATIONS
| Graph | Status |
| :--- | :--- |
| Modules Graph | Missing (run just viz-modules) |
| Dependencies Graph | Missing (run just viz-deps) |

==========================================
3. SECURITE ET CONFORMITE
==========================================
AUDIT CARGO (VULNERABILITIES)
```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1090 security advisories (from /home/tco/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (356 crate dependencies)
```

CARGO DENY (LICENSES & BANS)
```
advisories ok, bans ok, licenses ok, sources ok
```

==========================================
4. DEPENDANCES ET HYGIENE
==========================================
DEPENDANCES INUTILISEES (UDEPS)
```
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/domain-849bcc345b31b740.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/domain-7c551cfde9c257b0.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/ports-1c14cd0f602bb202.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/ports-eadb8a1d02398f28.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/application-6866775b59a41fe0.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/application-ef884d6faf032a14.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/profil_test-a7e5eabec6905256.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_scraper_http-ffd8a4cf0073f0fb.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_openai-ca623f5b383f03ed.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_ollama-d09934610a58eb21.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_claude-30eca804f1f688e4.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_claude-b553515234d2c2d5.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_scraper_http-2d95625e48a0b9f0.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_openai-1ec1f36959e7306d.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_ollama-b594887e457c5de8.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_postgres-67daa008f0b294ca.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_postgres-fddb81576fca307a.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-01abb1ad1aa8e023.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-e2717ae54c40a618.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-c9a9a90fbdf507ee.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_profile-8d98b93e556a8b2b.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_blank-c6477df0fb0fdff1.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_offers_instances-92570f1d076a9ae2.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api_integration-a85ea431ff3daceb.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_profile-0264977df9d7f4a9.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_chunks-797d546697dd3786.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_blank-c5bd9249c88ec02d.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_offers_instances-17b880514f090a27.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-5fd15cb9e0923475.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_chunks-7cceac4b29edecb8.d"
All deps seem to have been used.
```

DOUBLONS DE DEPENDANCES
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
    ├── adapter-llm-ollama v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/llm_ollama) (*)
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
│   ├── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http) (*)
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
│           ├── adapter-scraper-http v0.1.0 (/home/tco/Bureau/alternance/crates/adapters/scraper_http) (*)
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

HYGIENE FRONTEND (KNIP)
```


Unused exports (3)
showNotification      function  web/assets/js/render/audio.js:31:17
parseHttpUrl          function  web/assets/js/utils.js:25:17       
normalizePhoneDigits  function  web/assets/js/utils.js:65:17       
```

==========================================
5. PERFORMANCE ET OPTIMISATION
==========================================
ANALYSE DU BINAIRE (CARGO BLOAT)
```
 File  .text     Size Crate
 9.4%  29.4% 884.2KiB std
 3.9%  12.0% 361.5KiB rustls
 3.2%  10.1% 302.3KiB sqlx_postgres
 3.1%   9.7% 292.3KiB ring
 2.2%   6.7% 201.8KiB sqlx_core
 1.6%   5.0% 150.6KiB adapter_postgres
 1.5%   4.5% 135.9KiB tokio
 1.1%   3.3%  99.2KiB seed_offers_instances
 0.8%   2.4%  70.8KiB [Unknown]
 0.7%   2.3%  67.9KiB webpki
 0.7%   2.1%  61.9KiB serde_json
 0.5%   1.5%  44.2KiB url
 0.4%   1.1%  34.1KiB chrono
 0.3%   1.0%  28.9KiB idna
 0.2%   0.6%  18.3KiB tracing
 2.3%   7.2% 217.4KiB And 49 more crates. Use -n N to show more.
32.1% 100.0%   2.9MiB .text section size, the file size is 9.1MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

==========================================
Fin du rapport
