# RecruitAI Cleanup : Audit & Roadmap

## 1. HISTORIQUE DES RÉPARATIONS (Session 2026-05-04/05)
*Socle stabilisé et validé.*

- **[FIXED] Crash API (Claude)** : Sécurisation des headers HTTP (suppression des `unwrap` critiques).
- **[FIXED] Régression Assets** : Restauration du service statique via `nest_service` et fallback SPA dans `lib.rs`.
- **[FIXED] Alignement API** : Support du verbe **PUT** sur le profil et correction de la route des annexes (`/api/profile/active/annexes`).
- **[FIXED] Validation** : Renforcement de la validation JSON en entrée.
- **[DONE] Phase 3.5 & 4** : Modularisation du frontend. `dashboard.js` (168 LOC) découpé en contrôleurs.
- **[DONE] Hygiène std** : Remplacement de `once_cell` par `std::sync::OnceLock` dans `application`.
- **[DONE] Hygiène intake** : Nettoyage du bruit visuel dans `crates/application/src/intake/mod.rs`.
- **[FIXED] Data Loss Bug** : La régénération partielle n'efface plus le CV ni la lettre existants. Correctif métier dans `generate/` + normalisation des `jsonb null` côté Postgres.

## 2. ÉTAT ACTUEL (2026-05-06)

- **Backend** : ✅ Stable & Architecturalement pur.
- **Tests** : ✅ 64 tests validés (Workspace total).
- **Frontend** : ⚠️ Modularisé mais `ui.js` (546 LOC) en surpoids.
- **Dette Critique** : Bug Ingestion 500 encore à reproduire et circonscrire.
- **Architecture** : ✅ Couche Domaine purifiée (découplage `serde_json`).

## 3. DETTES TECHNIQUES PRIORISÉES (RED LIST)

### HIGH (Critique / Bloquant)
1. **[BUG] Ingestion 500** : Erreur intermittente sur `/api/offres/ingest`. Cause non confirmée à ce stade.
2. **[UI] Refacto ui.js** : 546 LOC. Doit être segmenté en modules spécialisés (Toasts, Modals, Shared components).
3. **[RAG] Couplage Restitution/RAG** : L'étape de recherche vectorielle bloque la Restitution même sans besoin de profil.

### MED (Amélioration / Modularité)
1. **[CODE] crates/application/src/chat/mod.rs** : God Module (582 LOC). Stream et persistence entremêlés.
2. **[DATA] Templates MBDA** : Hardcodés dans `data/templates/*.json`. Doivent être génériques.
3. **[SCRAPE] Anti-bot** : Échec sur SPA/Anti-bot (WTTJ, Siemens). Bypass texte brut nécessaire.

### LOW (Hygiène / Polish)
Aucune dette low ouverte.

---

## 4. RÈGLES DE GOUVERNANCE (NON-NÉGOCIABLES)

1.  **No Silent Edits** : Toute modification de contrat API ou de logique métier doit être explicitement validée par un test.
2.  **Atomic Commits** : Un commit par changement logique. Pas de mélanges.
3.  **Evidence-Based** : Chaque fix doit être prouvé par un log, un curl ou un test d'intégration (verbatim requis).
4.  **Docs First** : Ce fichier est la source de vérité. Mettre à jour après chaque Phase terminée.
5.  **Git Protocol** : `git add <files>` explicites. Pas de `git add .`.
