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

- **Backend** : ✅ Stable, Architecturalement pur.
- **Tests** : ✅ 64 tests validés (Workspace total, 100% pass).
- **Frontend** : ✅ Modularisé. `ui.js` allégé.
- **Dette Critique** : ✅ Bug Ingestion 500 résolu via typage d'erreurs plus précis.
- **Architecture** : ✅ Couche Domaine purifiée. Outillage au vert.

## 3. DETTES TECHNIQUES PRIORISÉES (RED LIST)

### HIGH (Critique / Bloquant)
1. **[INGESTION] Correction du bug 500** : ✅ Résolu. Les erreurs de contenu pauvre sont désormais des `400 Bad Request`. Logs ajoutés.
2. **[UI] Refacto ui.js** : ✅ Terminé. Découpé en `components/` et `modules/`.
3. **[RAG] Couplage Restitution/RAG** : ✅ Résolu. Pipeline optimisé dans `application/generate/mod.rs`.

### MED (Amélioration / Modularité)
1. **[CODE] crates/application/src/chat/mod.rs** : ✅ Terminé. Module découpé en `types.rs`, `persistence.rs` et `streaming.rs`. Orchestration clarifiée.
2. **[DATA] Templates MBDA** : ✅ Terminé. Les templates dans `data/templates/` sont désormais génériques (placeholders).
3. **[SCRAPE] Anti-bot** : Échec sur SPA/Anti-bot (WTTJ, Siemens). Bypass texte brut nécessaire.
   Décision d'architecture : évaluer **ScrapingAnt** comme fallback premium ciblé par domaine, et non comme chemin par défaut. Usage recommandé seulement après échec du scraper natif, HTML vide/incomplet ou détection d'un challenge JS/anti-bot.

### LOW (Hygiène / Polish)
Aucune dette low ouverte.

---

4. **[UX] Chat Unifié** : Unifier le chat global et par instance pour une continuité conversationnelle. L'IA doit pouvoir orchestrer plusieurs offres via un seul fil de discussion (Idée validée, à faire en phase finale).

## 4. RÈGLES DE GOUVERNANCE (NON-NÉGOCIABLES)

1.  **No Silent Edits** : Toute modification de contrat API ou de logique métier doit être explicitement validée par un test.
2.  **Atomic Commits** : Un commit par changement logique. Pas de mélanges.
3.  **Evidence-Based** : Chaque fix doit être prouvé par un log, un curl ou un test d'intégration (verbatim requis).
4.  **Docs First** : Ce fichier est la source de vérité. Mettre à jour après chaque Phase terminée.
5.  **Git Protocol** : `git add <files>` explicites. Pas de `git add .`.
