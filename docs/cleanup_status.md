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
- **Tests** : ✅ 8 tests d'intégration HTTP validés (Axum + MockRepos).
- **Contrat API** : ✅ Aligné sur le frontend original.
- **Frontend** : ⚠️ Toujours monolithique (`dashboard.js`), couplage global élevé via `window.*`.

## 3. ROADMAP DE REMÉDIATION

### Phase 3.5 : Désentrelacement Frontend (Cible prioritaire)
**Objectif** : Supprimer les couplages globaux avant le découpage physique des fichiers.
1.  **Bus d'Événements** : Créer `web/assets/js/events.js` (Pattern `EventTarget`).
2.  **Window Purge** : Supprimer `window.activeJobId`, `window.state`, etc. (un commit par variable).
3.  **Organisation Interne** : Regrouper les fonctions de `dashboard.js` par blocs logiques (UI, Offres, Profil).
4.  **Security** : Remplacer les derniers `innerHTML` par du `textContent` ou des templates.

### Phase 4 : Modularisation (Le Split)
- Extraction de `OfferController.js`, `ProfileController.js`, `ChatController.js`.
- Communication inter-modules via `events.js` uniquement.

### Phase 5-7 : Durcissement & Production
- **IA** : Fix bug RAG 500 (profil vide) → passage en 422.
- **Data** : Tests Postgres réels (Round-trip JSONB).
- **Polish** : Skeletons, transitions fluides et export PDF final.

## 4. RÈGLES DE GOUVERNANCE (NON-NÉGOCIABLES)

1.  **No Silent Edits** : Toute modification de contrat API ou de logique métier doit être explicitement validée par un test.
2.  **Atomic Commits** : Un commit par changement logique. Pas de mélanges.
3.  **Evidence-Based** : Chaque fix doit être prouvé par un log, un curl ou un test d'intégration.
4.  **Docs First** : Ce fichier est la source de vérité. Mettre à jour après chaque Phase terminée.

---
### Dette Technique Résiduelle
- [LOW] `test_get_annexes_200` : Vérifie uniquement le statut, pas la structure du body.
- [HIGH] `POST /api/ingest` : Échoue en 500 si le profil est vide.
- [INFO] Tag `backend-stable-2026-05-05` déplacé via `--force`.
