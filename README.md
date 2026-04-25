# Alternance : Pipeline de Candidature

Dépôt centralisant l'écosystème de recherche d'alternance de Roméo Cavazza.

## Architecture

- `/engines/web` : Moteur de rendu CV et lettres (HTML/CSS/JS).
- `/engines/output` : Exports générés (PDF, HTML, etc.).
- `/data/offres` : Analyse, suivi et données RAW des offres d'emploi.
- `/data/user` : Source de vérité (profile.md) et projets versionnés.
- `/docs` : Documentation technique et consignes agent.
- `/scrape-offres.yaml` : Paramétrage principal du scraper.
- `/data/offres/liste.json` : Source de vérité des URLs (synchronisable depuis liste.md).
- `/engines/scripts` : Outils d'automatisation (Python, Nix).

## Boot for AI

Si vous êtes un agent IA arrivant sur ce repo, suivez cet ordre de lecture pour une immersion optimale :
1. `./README.md` (Vous êtes ici)
2. `./data/user/profile.md` (Source de vérité absolue)
3. `./docs/instructions.md` (Protocole et règles d'engagement)
4. `./docs/how_it_works.md` (Détails techniques et scripts)

## Principes Fondamentaux

- **CV-as-Code** : La donnée est séparée de la présentation.
- **Source de Vérité Unique** : Tout contenu provient du dossier `/data/user`.
- **Zéro Emoji** : Sobriété professionnelle dans tous les livrables.
- **Automation First** : Utilisation de scripts pour le scraping et le formattage.
