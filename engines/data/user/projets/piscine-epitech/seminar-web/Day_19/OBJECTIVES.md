# Day 19 — Objectifs et Enseignements

## Objectifs

- Manipuler formulaires et `$_POST` en PHP, valider/normaliser les entrées.
- Générer du HTML côté serveur à partir de gabarits.
- Structurer la logique en fonctions testables.

## Enseignements

- Validation côté serveur: vérifier présence/format (`is_numeric`, bornes, `isset`).
- Séparation rendu/logique: fonctions qui retournent HTML vs side effects.
- Hygiène d’entrées: normaliser (trim/sanitize en pratique), éviter concat naïve si variables externes.
