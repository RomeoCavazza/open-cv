# RecruitAI Cleanup : Audit & Roadmap

## 1. AUDIT INITIAL & RÉPARATIONS (Session 2026-05-04/05)
*Ce qui a été identifié et corrigé pour stabiliser le socle.*

- **[FIXED] Fuite de couche (Domain)** : `serde_json::Value` remplacé par des structs typées dans `crates/domain/src/profil.rs`.
- **[FIXED] Crash API (Claude)** : Sécurisation des headers HTTP (suppression des `unwrap` critiques).
- **[FIXED] Régression Assets** : Restauration du service statique via `nest_service` et fallback SPA dans `lib.rs`.
- **[FIXED] Alignement API** : Support du verbe **PUT** sur le profil et correction de la route des annexes (`/api/profile/active/annexes`).
- **[FIXED] Validation** : Renforcement de la validation JSON en entrée pour éviter les échecs silencieux.

## 2. ÉTAT ACTUEL

- **Backend** : ✅ Stable, modulaire, typé. Tag : `backend-stable-2026-05-05`.
- **Tests** : ✅ 63 tests validés (Workspace total). Suite d'intégration HTTP résiliente.
- **Base de Données** : ✅ Pipeline de reset/seed canonique validé (Justfile).
- **RAG/Indexation** : ✅ Fonctionnel via `seed_chunks`.
- **Frontend** : ⚠️ Toujours monolithique (`dashboard.js`), couplage global élevé.

## 3. ROADMAP DE REMÉDIATION

### Phase 3.5 : Désentrelacement Frontend (Cible prioritaire)
**Objectif** : Supprimer les couplages globaux avant le découpage physique des fichiers.
1.  **Bus d'Événements** : Créer `web/assets/js/events.js` (Pattern `EventTarget`).
2.  **Window Purge** : Supprimer `window.activeJobId`, `window.state`, etc. (un commit par variable).
3.  **Organisation Interne** : Regrouper les fonctions de `dashboard.js` par blocs logiques (UI, Offres, Profil).
4.  **Security** : Remplacer les derniers `innerHTML` par du `textContent` ou des templates.

### Phase 4 : Modularisation (Le Split) - [2026-05-06]
**Status: SUCCESS ✅**
- **Controllers**: Extracted `OfferController.js` and `ProfileController.js`.
- **Decoupling**: `dashboard.js` reduced from **808 LOC** to **365 LOC**.
- **Architecture**: Orchestrator pattern implemented. `dashboard.js` only handles high-level events and routing.
- **Persistence**: Fixed Profile data persistence and API body limits (10MB).

### Phase 5-7 : Durcissement & Production
- **IA** : Fix bug RAG 500 (profil vide) → passage en 422.
- **Data** : Tests Postgres réels (Round-trip JSONB).
- **Polish** : Skeletons, transitions fluides et export PDF final.

### 3. Frontend Modularization (Phase 3.5) - [2026-05-05]
**Status: SUCCESS ✅**
- **Event Bus**: Unified event bus implemented in `web/assets/js/modules/events.js`.
- **Decoupling**: `dashboard.js` and `chat.js` are now independent. Communication happens via `OFFER_SELECTED`, `LLM_PROVIDER_CHANGED`, etc.
- **Window Purge**: `window.activeJobId`, `window.activeInstanceSlug`, and `window.activeInstanceData` have been completely removed.
- **Premium UI**: Added CSS loaders and an event-driven Toast notification system.
- **Debt remaining**: Physical splitting of `dashboard.js` into smaller controllers (Phase 4) is pending but unblocked.

### 5. Chat Streaming & Advanced Tooling (Phase 8) - [2026-05-05]
**Status: SUCCESS ✅**
- **SSE Streaming**: Implemented full token streaming from backend to frontend (Server-Sent Events).
- **Assistant UI**: Modernized chat interface (plain text, no bubbles, distraction-free).
- **Quality Audit**: Integrated `cargo-deny`, `tarpaulin`, `clippy` (workspace wide), `eslint`, and `stylelint`.
- **Performance**: Added `Criterion.rs` for micro-benchmarking and `flamegraph` support.
- **Organization**: Cleaned up project root by moving configs to `tooling/`.

### 6. Remaining Debt & Next Steps
1. **Backend**: 
- **Data Loss (HIGH)**: Ingestion/Generation wipes existing deliverables (CV, cover letter) if the hash changes or during the start of the generation pipeline. Re-scraping an existing URL causes associated documents to disappear. Requires URL-based deduplication and application instance preservation logic.
- Decouple Restitution from RAG (`generate/mod.rs`).
- Implement automated indexer for the `chunks` table.
- [FIXED] **God Module Dashboard**: `dashboard.js` (365 LOC). Modularization complete (Phase 4).

## 4. AUDIT DE STABILITÉ ET INTÉGRITÉ (2026-05-06)
*Diagnostic exhaustif du comportement du pipeline post-reset DB.*

### RED LIST (CRIT)
- **[CRIT] crates/domain/src/lib.rs** : Fuite de couche. `serde_json` utilisé directement dans le domaine. Nécessite des types métier purs.
- **[FIXED] web/assets/js/dashboard.js** : Modularization complete. Logic moved to ProfileController and OfferController.
- **[CRIT] crates/application/src/chat/mod.rs** : God Module (582 LOC). Logique de stream et persistence entremêlées.
- **[CRIT] crates/adapters/postgres/src/profil.rs:205** : Hardcoded UUID avec `unwrap()`. Risque de crash au démarrage si incohérence DB.

### KILL LIST (Cleanup immédiat)
- **crates/api/src/bin/seed_blank.rs** : Supprimé car redondant avec `seed_profile`.
- **Cargo.toml:once_cell** : À remplacer par `std::sync::OnceLock`.
- **Commentaires paraphrases** : Nettoyage du bruit visuel dans `intake/mod.rs`.

---

## 5. RÈGLES DE GOUVERNANCE (NON-NÉGOCIABLES)

1.  **No Silent Edits** : Toute modification de contrat API ou de logique métier doit être explicitement validée par un test.
2.  **Atomic Commits** : Un commit par changement logique. Pas de mélanges.
3.  **Evidence-Based** : Chaque fix doit être prouvé par un log, un curl ou un test d'intégration.
4.  **Docs First** : Ce fichier est la source de vérité. Mettre à jour après chaque Phase terminée.

---
### Dette Technique Résiduelle (Confirmée)
- [FIXED] **Indexeur de profil** : Résolu via `seed_chunks`. Intégré au pipeline canonique.
- [HIGH] **Couplage Restitution/RAG** : L'étape de recherche vectorielle (`generate/mod.rs:220`) bloque la Restitution même sans besoin de profil.
- [MED] **Modularité Frontend** : `dashboard.js` (365 LOC) modularisé. `ui.js` (458 LOC) reste à découper.
- [MED] **Scraper limité** : Échec sur SPA/Anti-bot (WTTJ, Siemens). Bypass texte brut nécessaire.
- [LOW] **Templates contaminés** : Contenu MBDA hardcodé dans `data/templates/*.json` polluant les sorties LLM.

### Tests Indéterminés (À refaire quand Quotas Claude OK)
- **[INFO] Chat Injection Contexte (1.3)** : Non prouvé sur Ollama (confusion template/offre).
- **[INFO] Chat Mutation JSON (1.4)** : Non testé (échec de formatage sur Ollama).

---
### Q&A BASELINE SESSION (2026-05-05)
*Diagnostic exhaustif du comportement du pipeline post-reset DB validé par preuves brutes (psql, logs, grep).*
