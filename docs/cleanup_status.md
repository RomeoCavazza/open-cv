# RecruitAI Cleanup Status

> [!IMPORTANT]
> ## RÈGLES NON-NÉGOCIABLES (post-incident untangle JS)
> 
> 1. **Isolation du Scope** : Tout fichier édité en dehors du scope déclaré de la phase courante = STOP, remontée à l'utilisateur, attente de validation.
> 
> 2. **Traitement des Bugs** : Tout bug rencontré pendant une refacto = STOP, remontée, ticket séparé. Jamais de "fix opportuniste" ou silencieux.
> 
> 3. **Transparence Factuelle** : Tout résumé de phase doit lister exactement les fichiers édités via une sortie brute (ex: `git diff --stat`), pas une description en prose.
> 
> 4. **Vérification Explicite** : Aucune phase n'est "terminée" tant que les vérifs demandées (greps, tests) n'ont pas été affichées en sortie brute dans la console.

## État des Lieux au 2026-05-04 22:55

### Points Critiques
*   **JS Untangle** : À REFAIRE. Les globaux ont été restaurés pour repartir sur une base saine.
*   **Régressions Backend** : Corrigées et verrouillées par tests.
*   **Dette de Test** : Réduite (6 tests d'intégration HTTP opérationnels).

### Plan de Remédiation (Priorité 0)
1. **REVERT** : Suppression du fallback 404 et de l'auto-création dans `profile.rs`. [FAIT]
2. **FIX REGRESSION** : Alignement du contrat `restitution` (regression Phase 3). [FAIT]
3. **TESTS** : Ajout de 6 tests d'intégration HTTP (Profil, Ingest, Payloads). [FAIT]
4. **RE-UNTANGLE** : Nettoyage réel de `dashboard.js` (Phase 3.5 JS). [À FAIRE]
5. **VALIDE** : Test manuel rigoureux dashboard/backend. [À FAIRE]

## Dette Technique Connue
*   **Tests Adapters (Postgres)** : La sérialisation/désérialisation `ProfilContent` <-> `JSONB` n'est pas couverte par les tests API (Mocks). Nécessite des tests d'intégration dédiés dans `crates/adapters/postgres/`.
*   **Historique Git** : Le commit `dc63bdd` ne compile pas (mismatch `restitution`/`analysis`). À squash avec `fd729e7` lors d'un futur rebase de nettoyage pour préserver l'intégrité du `git bisect`.
