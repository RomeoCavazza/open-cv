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

## État des Lieux au 2026-05-04 22:40

### Points Critiques
*   **JS Untangle** : À REFAIRE. Les globaux ont été restaurés pour repartir sur une base saine.
*   **Régressions Backend** : Corrigées (revert fallback/auto-create).
*   **Dette de Test** : Absence de tests d'intégration HTTP.

### Plan de Remédiation (Priorité 0)
1. **REVERT** : Suppression du fallback 404 et de l'auto-création dans `profile.rs`. [FAIT]
2. **FIX REGRESSION** : Alignement du contrat `restitution` (regression Phase 3). [FAIT]
3. **TESTS** : Ajout de 3 tests d'intégration HTTP (GET profile 200/404, POST profile 200/404, POST ingest). [À FAIRE]
4. **RE-UNTANGLE** : Nettoyage réel de `dashboard.js` sans "compatibilité temporaire". [À FAIRE]
5. **VALIDE** : Test manuel rigoureux. [À FAIRE]
