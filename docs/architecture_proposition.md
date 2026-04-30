# Architecture Canonique

Ce dépôt est un générateur local de candidatures. La structure cible actuelle est volontairement simple.

```txt
alternance/
├── data/
│   ├── instances/
│   ├── offres/
│   │   ├── liste.json
│   │   ├── liste.md
│   │   └── raw/
│   ├── templates/
│   └── user/
│       ├── old-cv/
│       ├── profile.md
│       └── projets/
├── docs/
├── scripts/
└── web/
```

## Règles

- `data/` est la seule source canonique des données métier.
- `scripts/` contient les workflows Python et la résolution des chemins.
- `web/` affiche les rendus et lit directement dans `/data/`.
- `data/templates/` ne contient que les templates actifs de génération.
- `data/user/old-cv/` contient l'historique des anciens CV.
- `data/user/projets/` contient les projets embarqués en submodules Git.

## Hors Périmètre Actuel

- pas d'API locale
- pas de base de données
- pas de couche RAG
- pas de duplication `web/data`

## Convention Offres

- `data/offres/liste.json` : index canonique
- `data/offres/liste.md` : édition manuelle
- `data/offres/raw/*.md` : texte brut canonique des offres
