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
### Dette Technique Résiduelle (Confirmée)
- [LOW] `test_get_annexes_200` : Vérifie uniquement le statut, pas la structure du body.
- [HIGH] **Indexeur de profil absent** : La table `chunks` reste vide après reset/seed. RAG inopérant par défaut.
- [HIGH] **Couplage Restitution/RAG** : L'étape de recherche vectorielle (`generate/mod.rs:220`) bloque la Restitution même sans besoin de profil.
- [MED] **Scraper limité** : Échec sur SPA/Anti-bot (WTTJ, Siemens). Bypass texte brut nécessaire.
- [LOW] **Templates contaminés** : Contenu MBDA hardcodé dans `data/templates/*.json` polluant les sorties LLM.

### Tests Indéterminés (À refaire quand Quotas Claude OK)
- **[INFO] Chat Injection Contexte (1.3)** : Non prouvé sur Ollama (confusion template/offre).
- **[INFO] Chat Mutation JSON (1.4)** : Non testé (échec de formatage sur Ollama).

---
### Q&A BASELINE SESSION (2026-05-05)
*Diagnostic exhaustif du comportement du pipeline post-reset DB validé par preuves brutes (psql, logs, grep).*
