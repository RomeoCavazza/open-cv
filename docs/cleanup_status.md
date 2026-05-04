# RecruitAI Governance & Status

> [!IMPORTANT]
> ## RÈGLES DE CONDUITE (Post-incident Untangle JS)
> 
> 1. **Isolation du Scope** : Tout fichier édité en dehors du scope déclaré de la phase courante = STOP, remontée à l'utilisateur, attente de validation.
> 
> 2. **Traitement des Bugs** : Tout bug rencontré pendant une refacto = STOP, remontée, ticket séparé. Jamais de "fix opportuniste" ou silencieux.
> 
> 3. **Transparence Factuelle** : Tout résumé de phase doit lister exactement les fichiers édités via une sortie brute (ex: `git diff --stat`), pas une description en prose.
> 
> 4. **Vérification Explicite** : Aucune phase n'est "terminée" tant que les vérifs demandées (greps, tests) n'ont pas été affichées en sortie brute dans la console.

---

## ÉTAT DES LIEUX (au 2026-05-04 23:00)

### Historique des Phases
*   **Phase 1 (Sécurité)** : [7236338] Remplacement des `unwrap()` par des `Result`. [FAIT]
*   **Phase 2 (Nettoyage)** : [2a20870] Suppression du code/CSS/deps morts. [FAIT]
*   **Phase 3 (Domaine)** : [b4ba435] Typage fort du `ProfilContent`. [FAIT]
*   **Phase 4 (Backend)** : [dc63bdd] Découpage de `intake.rs` en modules. [FAIT]
*   **Tests d'Intégration** : [b5654f6] 6 tests HTTP validant les contrats API. [FAIT]

### Points Critiques
*   **JS Untangle** : À REFAIRE. Les globaux ont été restaurés pour repartir sur une base saine.
*   **Régressions Backend** : Corrigées et verrouillées par tests.
*   **Dette de Test** : Réduite (6 tests d'intégration HTTP opérationnels).

### Plan de Remédiation (Reste à faire)
1. **RE-UNTANGLE JS** : Nettoyage réel de `dashboard.js` (Phase 3.5 JS). Supprimer les `window.*`.
    - *Note* : `events.js` a été supprimé par `git restore`, à recréer — voir l'historique git pour le code minimal (37 LOC, CustomEvent + cleanup pattern).
2. **SPLIT DASHBOARD** : Découpage de `dashboard.js` en contrôleurs modulaires.
3. **TESTS POSTGRES** : Tests de round-trip `JSONB` <-> `ProfilContent` dans `crates/adapters/postgres/`.
4. **SQUASH HISTORIQUE** : Squash du commit `dc63bdd` (cassé) avec `fd729e7` lors d'un nettoyage d'historique.
5. **TIGHTEN TEST #6** : Renforcer l'assertion du test d'ingestion pour vérifier le code exact (400 vs 422).
6. **STATIC UNWRAPS** : Nettoyer les `unwrap()` restants dans les initialiseurs `static Lazy` (schemas).
7. **VALIDE** : Test manuel rigoureux dashboard/backend complet.

### Dette Technique Connue
*   **Mocks IA** : Les stubs dans les tests d'intégration ne détectent pas les changements de schémas LLM.
*   **Couplage Frontend** : `window.state` et `dashboard.js` (870 LOC) toujours présents.

---

## AUDIT PROMPT — RUST + VANILLA WEB

### RÔLE
Tu audites un repo. Pas de compliments, pas de "globalement c'est bien". Chaque observation = problème + localisation (fichier:ligne) + fix concret + effort (S/M/L).

### RÈGLES DE SORTIE (NON-NÉGOCIABLES)
- Format pour CHAQUE finding :
  `[SEVERITY] path/file.rs:LINE — Problème (1 phrase) — Fix (1 phrase) — Effort: S/M/L`
- SEVERITY = CRIT (crash / sécu / data loss) | HIGH (bug latent / dette qui scale) | MED (refacto) | LOW (style)

### SEUILS DURS
- `>300 LOC` (Rust) / `>250 LOC` (JS) = HIGH
- `>5 imports croisés` entre modules = signal de god module
- Struct avec >7 champs sans justification = à éclater
- Fichier nommé `utils.rs`, `helpers.rs`, `common.rs` → renommer ou supprimer

### CHECKS RUST / WEB
1. **Unwrap** → `Result`.
2. **`.clone()`** → justifier.
3. **Fuite de couches** : `sqlx`, `axum` dans `domain` = CRIT.
4. **`innerHTML`** dynamique = CRIT.
5. **Globales implicites** (`window.x`) = HIGH.
