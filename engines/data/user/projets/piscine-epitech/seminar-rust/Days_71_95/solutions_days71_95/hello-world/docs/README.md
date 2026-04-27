# Documentation - Hello World RTC

Application de messagerie instantanee en temps reel (type Discord).

## Structure de la documentation

```text
docs/
├── README.md                    # Ce fichier
├── specifications/              # Specifications du projet
│   ├── requirements.md          # Cahier des charges complet
│   ├── grading-criteria.md      # Criteres de notation
│   └── moscow.md                # Priorisation MoSCoW
├── architecture/                # Architecture technique
│   ├── overview.md              # Vue d'ensemble
│   ├── database.md              # Modele de donnees
│   └── folder-structure.md      # Structure des dossiers
└── uml/                         # Diagrammes UML
    ├── classes.puml             # Diagramme de classes
    ├── database-schema.puml     # Schema de base de donnees
    └── entities.md              # Description des entites
```

## Stack Technique

| Composant           | Technologie                                    |
| ------------------- | ---------------------------------------------- |
| **Frontend**        | Next.js 16 + React 19 + TypeScript             |
| **Backend**         | Rust + Axum + Tokio                            |
| **Base de donnees** | PostgreSQL (relationnel) + MongoDB (messages)  |
| **Temps reel**      | WebSockets                                     |
| **Styling**         | Tailwind CSS 4                                 |

## Fonctionnalites principales

- **Authentification** : Inscription/Connexion JWT
- **Serveurs** : Creation, rejoindre via code d'invitation
- **Canaux** : Canaux textuels par serveur
- **Messages** : Temps reel via WebSocket
- **Roles** : Owner / Admin / Member (RBAC)
- **Presence** : Utilisateurs connectes, indicateur de frappe

## Liens rapides

- [Specifications completes](./specifications/requirements.md)
- [Architecture technique](./architecture/overview.md)
- [Schema de base de donnees](./architecture/database.md)
- [Diagramme de classes](./uml/classes.puml)
