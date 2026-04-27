# Day 19 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day19/](solutions_day19/)

## Objectifs
- Manipuler formulaires et `$_POST` en PHP, valider/normaliser les entrées.
- Générer du HTML côté serveur à partir de gabarits.
- Structurer la logique en fonctions testables.

## Actions
- `task01` — `whoami`: construire un message selon `name`/`age` fournis.
- `task02` — Formulaire enrichi: message étendu avec `curriculum` (uppercased).
- `task04` — `display_menu`: retourner un menu HTML.
- `task05` — `render_body`: sélectionner un fragment HTML selon `page` (fallback inconnu).

## Leçons
- Validation côté serveur: vérifier présence/format (`is_numeric`, bornes, `isset`).
- Séparation rendu/logique: fonctions qui retournent HTML vs side effects.
- Hygiène d’entrées: normaliser (trim/sanitize en pratique), éviter concat naïve si variables externes.
