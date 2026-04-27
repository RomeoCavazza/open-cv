# Architecture technique - Hello World RTC

## 1. Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND                                │
│                    Next.js + TypeScript                         │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   REST API  │  │  WebSocket  │  │      Tailwind CSS       │  │
│  └──────┬──────┘  └──────┬──────┘  └─────────────────────────┘  │
└─────────┼────────────────┼──────────────────────────────────────┘
          │                │
          ▼                ▼
┌─────────────────────────────────────────────────────────────────┐
│                         BACKEND                                 │
│                      Rust + Axum                                │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Handlers  │  │   Services  │  │      Repositories       │  │
│  │   (HTTP)    │  │  (Business) │  │        (Data)           │  │
│  └──────┬──────┘  └──────┬──────┘  └───────────┬─────────────┘  │
└─────────┼────────────────┼─────────────────────┼────────────────┘
          │                │                     │
          ▼                ▼                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                        DATABASES                                │
│                                                                 │
│  ┌─────────────────────────┐  ┌─────────────────────────────┐  │
│  │       PostgreSQL        │  │          MongoDB            │  │
│  │  (Identités, Perms)     │  │    (Messages, Logs)         │  │
│  └─────────────────────────┘  └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Principes d'architecture

### Séparation des responsabilités
- **API Layer** : Parsing, authentification, mapping erreurs → HTTP/WS
- **Services Layer** : Règles métier, RBAC, workflows
- **Repository Layer** : Accès aux données sans logique métier

### Maintenabilité
- Un fichier = Une responsabilité
- Modules indépendants et testables

### Testabilité
- Logique métier testable sans HTTP ni BDD
- Mocks/fakes pour les tests unitaires

### Évolutivité
- Composants techniques interchangeables
- Ajout de features sans réécriture

---

## 3. Architecture backend

### Couches

```
┌────────────────────────────────────────────┐
│              API / Handlers                │
│     (HTTP routes, WebSocket handlers)      │
├────────────────────────────────────────────┤
│            Services / Use-cases            │
│     (Business logic, RBAC, validation)     │
├────────────────────────────────────────────┤
│              Repositories                  │
│       (PostgreSQL + MongoDB access)        │
└────────────────────────────────────────────┘
```

### Modules

| Module | Responsabilité |
|--------|----------------|
| `auth` | Authentification, JWT, sessions |
| `servers` | Gestion des serveurs |
| `channels` | Gestion des canaux |
| `messages` | Envoi/lecture messages |
| `realtime` | WebSocket, présence, typing |

---

## 4. Workflows cross-database

### Envoi d'un message
1. Authentification JWT
2. Vérification permissions (PostgreSQL)
3. Insertion message (MongoDB)
4. Broadcast WebSocket `message.created`

### Lecture de l'historique
1. Vérification accès canal (PostgreSQL)
2. Récupération paginée (MongoDB)
3. Enrichissement profils (PostgreSQL)

### Suppression d'un message
1. Vérification rôle (PostgreSQL)
2. Soft delete (MongoDB)
3. Broadcast WebSocket `message.deleted`

---

## 5. Temps réel (WebSocket)

### Gestion en mémoire
- Présence utilisateurs
- Indicateurs de typing
- Utilisateurs connectés

### Rooms
- Room par serveur
- Room par canal

### Protocole
```json
{
  "op": "MESSAGE",
  "t": "message.created",
  "d": { ... },
  "id": "uuid"
}
```

### Events
| Event | Description |
|-------|-------------|
| `message.created` | Nouveau message |
| `message.deleted` | Message supprimé |
| `presence.online` | Utilisateur connecté |
| `presence.offline` | Utilisateur déconnecté |
| `typing.start` | Début de frappe |
| `typing.stop` | Fin de frappe |

---

## 6. Points d'attention

### Performance
- Concurrence Rust/Tokio
- Gestion WebSocket multiples
- Limitation broadcasts
- Debounce typing
- Heartbeats

### Données
- Cohérence cross-DB (UUID partagés)
- Index MongoDB : `(channel_id, created_at)`
- Politique soft delete

### Sécurité
- JWT pour REST + WebSocket handshake
- RBAC strict côté serveur
- Codes erreur clairs (401/403/409)
- Rate limiting / anti-spam
- Validation stricte payloads WebSocket

