==========================================
RAPPORT DE SANTE INDUSTRIEL — RECRUITAI
==========================================
Généré le : 2026-05-10 22:36:27

==========================================
0. QUALITY GATE SUMMARY
==========================================
| Check | Status |
| :--- | :--- |
| Formatting | PASS |
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
 CSS                       3         1576         1379           39          158
 JavaScript               27         4034         3401          124          509
 JSON                      2           30           30            0            0
 Just                      1          224          169           27           28
 Nix                       1           87           70            6           11
 SQL                       1          253          201           30           22
 SVG                       2         9692         8212         1480            0
 TOML                     13          353          299           17           37
─────────────────────────────────────────────────────────────────────────────────
 HTML                      5         1050         1012            1           37
 |- CSS                    2           47           43            0            4
 |- JavaScript             1          145          130            3           12
 (Total)                             1242         1185            4           53
─────────────────────────────────────────────────────────────────────────────────
 Markdown                 10          846            0          666          180
 |- BASH                   4           38           26            8            4
 (Total)                              884           26          674          184
─────────────────────────────────────────────────────────────────────────────────
 Rust                     71        11849        10518           92         1239
 |- Markdown              32          199            0          181           18
 (Total)                            12048        10518          273         1257
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                   136        30423        25490         2674         2259
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

==========================================
2. ARCHITECTURE ET STRUCTURE
==========================================
ARBORESCENCE (EZA)
```
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
      Loaded 1068 security advisories (from /home/tco/.cargo/advisory-db)
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
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/application-ef884d6faf032a14.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/application-6866775b59a41fe0.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/profil_test-a7e5eabec6905256.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_openai-ca623f5b383f03ed.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_claude-30eca804f1f688e4.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_scraper_http-ffd8a4cf0073f0fb.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_claude-b553515234d2c2d5.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_ollama-31676fa200553554.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_openai-1ec1f36959e7306d.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_scraper_http-2d95625e48a0b9f0.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_llm_ollama-0d329a91cd76c3e9.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_postgres-fddb81576fca307a.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/adapter_postgres-67daa008f0b294ca.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-37f3a935bfdfcbcd.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-d1ad98e1d2394783.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_blank-347cc6ed0329ab53.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_profile-bf08b8648b186fd4.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_profile-bcf3b4957182c5d8.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_chunks-51615e55a2e2db49.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-0c5b253ccb96fe24.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_offers_instances-bfeb12868a141e1e.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api_integration-b800eae36b90f3bf.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_blank-0b5ddf04cd2598ad.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/api-d28bb0a9a5aa5562.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_chunks-37c2555e91fdeca6.d"
info: Loading depinfo from "/home/tco/Bureau/alternance/target/debug/deps/seed_offers_instances-772bb3a8c1bf606e.d"
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


Unused exports (1)
showNotification  function  web/assets/js/render/audio.js:31:17
```

==========================================
5. PERFORMANCE ET OPTIMISATION
==========================================
ANALYSE DU BINAIRE (CARGO BLOAT)
```
 File  .text     Size Crate
 8.9%  27.6% 891.5KiB std
 3.8%  11.8% 379.8KiB rustls
 3.0%   9.3% 301.8KiB sqlx_postgres
 2.9%   9.0% 292.3KiB ring
 1.6%   5.1% 164.8KiB sqlx_core
 1.4%   4.3% 139.8KiB tokio
 1.2%   3.8% 124.0KiB reqwest
 1.1%   3.3% 106.1KiB hyper_util
 0.8%   2.6%  83.0KiB hyper
 0.7%   2.2%  70.8KiB [Unknown]
 0.7%   2.1%  68.5KiB webpki
 0.5%   1.6%  51.8KiB http
 0.5%   1.5%  48.7KiB url
 0.4%   1.2%  39.6KiB serde_json
 0.4%   1.1%  35.8KiB adapter_postgres
 3.9%  12.1% 391.7KiB And 64 more crates. Use -n N to show more.
32.3% 100.0%   3.2MiB .text section size, the file size is 9.8MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

==========================================
Fin du rapport
