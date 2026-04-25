# Alternance : Pipeline de Candidature

Dépôt centralisant l'écosystème de recherche d'alternance de Roméo Cavazza.

## 🏗️ Architecture

- `/cv-web` : Moteur de rendu CV haute fidélité (HTML/CSS/JS).
- `/cv` : Banque de CV générés (Markdown).
- `/offres` : Analyse et suivi des offres d'emploi.
- `/portfolio` : Source de vérité (profil.md) et détails des projets.
- `/docs` : Documentation technique et consignes agent.
- `/scripts` : Outils d'automatisation (Python, Nix).

## 🤖 Boot for AI

Si vous êtes un agent IA arrivant sur ce repo, suivez cet ordre de lecture pour une immersion optimale :
1. `./README.md` (Vous êtes ici)
2. `./portfolio/profil.md` (Source de vérité absolue)
3. `./docs/instructions.md` (Protocole et règles d'engagement)
4. `./docs/how_it_works.md` (Détails techniques et scripts)

## ⚖️ Principes Fondamentaux

- **CV-as-Code** : La donnée est séparée de la présentation.
- **Source de Vérité Unique** : Tout contenu provient du dossier `/portfolio`.
- **Zéro Emoji** : Sobriété professionnelle dans tous les livrables.
- **Automation First** : Utilisation de scripts pour le scraping et le formattage.
