# Resume Builder

Générateur local de candidatures pour transformer des offres brutes en CV et lettres personnalisés.

![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS3](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![JS](https://img.shields.io/badge/JavaScript-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)
![Nix](https://img.shields.io/badge/NixOS-5277C3?style=for-the-badge&logo=nixos&logoColor=white)

Ce projet est un moteur de rendu de candidatures (CV et Lettres de Motivation) automatisé et ultra-personnalisé. Il permet de transformer des offres d'emploi brutes en dossiers de candidature haute fidélité en utilisant une approche **"Data-Driven"**.

## Aperçus

| Curriculum Vitae | Lettre de Motivation |
| :---: | :---: |
| ![Resume Preview](docs/assets/preview-resume.png) | ![Letter Preview](docs/assets/preview-letter.png) |

---

## Architecture du Projet

```text
.
├── data/               # Source de vérité (JSON/Markdown)
├── docs/               # Documentation et guides d'utilisation
├── scripts/            # CLI et tooling Python
├── web/                # Frontend statique
│   ├── resume/         # Moteur de rendu CV
│   └── cover-letter/   # Moteur de rendu lettre
├── shell.nix           # Environnement de développement
└── README.md
```

---

## Fonctionnement

Le workflow est divisé en quatre étapes clés :

1.  **Scrapage** : Récupération automatique des offres d'emploi depuis diverses plateformes.
2.  **Analyse** : Extraction des mots-clés, des compétences requises et des missions principales.
3.  **Personnalisation** : Génération de données JSON spécifiques à chaque offre à partir du profil utilisateur et des exigences du poste (stockées dans `data/instances/`).
4.  **Rendu** : Visualisation dynamique via le moteur web intégré permettant un export PDF parfait.

### Installation

```bash
# Entrer dans l'environnement de développement (si via Nix)
nix-shell

# Lancer le serveur de visualisation
python3 -m http.server 8000
```

---

## Stack Technique

- **Scripts** : Python 3 (scraping, traitement de données, génération).
- **Frontend** : HTML5 moderne, CSS3 (Flexbox/Grid), JavaScript natif (pour l'injection de données).
- **Formatage** : JSON pour les données, Markdown pour les sources d'offres.
- **Environnement** : Nix pour la reproductibilité.

---

## Workflow de Production

```mermaid
graph LR
    A[Offre d'emploi] -->|Scrape| B[Markdown]
    B -->|Personnalise| C[JSON Instance]
    D[Profil Global] -->|Injection| C
    C -->|Moteur Web| E[HTML/CSS]
    E -->|Print| F[Export PDF]
```

---
*Ce projet a été conçu pour industrialiser la recherche d'alternance tout en maintenant une qualité de personnalisation artisanale.*
