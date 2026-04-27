# Modele de donnees - Hello World RTC

## 1. Approche polyglotte

| Base de donnees | Utilisation                      | Justification                            |
| --------------- | -------------------------------- | ---------------------------------------- |
| **PostgreSQL**  | Identites, permissions, structure | Integrite transactionnelle, contraintes FK |
| **MongoDB**     | Historique des messages          | Volume eleve, semi-structure, scalabilite |

---

## 2. Entites PostgreSQL

### `users`

| Colonne         | Type      | Description                    |
| --------------- | --------- | ------------------------------ |
| `id`            | UUID      | Cle primaire                   |
| `username`      | VARCHAR   | Pseudonyme                     |
| `email`         | VARCHAR   | Email (unique)                 |
| `password_hash` | VARCHAR   | Mot de passe hashe             |
| `status`        | ENUM      | Online/Offline/DND/Invisible   |
| `avatar_url`    | VARCHAR?  | URL avatar (optionnel)         |
| `created_at`    | TIMESTAMP | Date de creation               |

### `servers`

| Colonne      | Type      | Description          |
| ------------ | --------- | -------------------- |
| `id`         | UUID      | Cle primaire         |
| `name`       | VARCHAR   | Nom du serveur       |
| `owner_id`   | UUID      | FK → users.id        |
| `created_at` | TIMESTAMP | Date de creation     |
| `updated_at` | TIMESTAMP | Date de modification |

### `server_members`

| Colonne     | Type      | Description         |
| ----------- | --------- | ------------------- |
| `server_id` | UUID      | FK → servers.id     |
| `user_id`   | UUID      | FK → users.id       |
| `role`      | ENUM      | OWNER/ADMIN/MEMBER  |
| `joined_at` | TIMESTAMP | Date d'adhesion     |

**PK composee** : `(server_id, user_id)`

### `channels`

| Colonne      | Type      | Description          |
| ------------ | --------- | -------------------- |
| `id`         | UUID      | Cle primaire         |
| `server_id`  | UUID      | FK → servers.id      |
| `name`       | VARCHAR   | Nom du canal         |
| `position`   | INT       | Ordre d'affichage    |
| `created_at` | TIMESTAMP | Date de creation     |
| `updated_at` | TIMESTAMP | Date de modification |

### `invites`

| Colonne      | Type       | Description                    |
| ------------ | ---------- | ------------------------------ |
| `id`         | UUID       | Cle primaire                   |
| `server_id`  | UUID       | FK → servers.id                |
| `code`       | VARCHAR    | Code unique                    |
| `created_by` | UUID       | FK → users.id                  |
| `expires_at` | TIMESTAMP? | Expiration (optionnel)         |
| `max_uses`   | INT?       | Utilisations max (optionnel)   |
| `uses_count` | INT        | Compteur                       |
| `created_at` | TIMESTAMP  | Date de creation               |

---

## 3. Collections MongoDB

### `channel_messages`

| Champ        | Type       | Description                       |
| ------------ | ---------- | --------------------------------- |
| `_id`        | ObjectId   | ID MongoDB                        |
| `message_id` | UUID       | ID unique (reference cross-DB)    |
| `channel_id` | UUID       | Reference canal                   |
| `server_id`  | UUID       | Reference serveur                 |
| `author_id`  | UUID       | Reference auteur                  |
| `content`    | TEXT       | Contenu du message                |
| `created_at` | TIMESTAMP  | Date d'envoi                      |
| `edited_at`  | TIMESTAMP? | Date d'edition                    |
| `deleted_at` | TIMESTAMP? | Date de suppression (soft delete) |
| `deleted_by` | UUID?      | Utilisateur ayant supprime        |

**Index** : `(channel_id, created_at)`

---

## 4. Relations

```text
┌──────────┐     owns      ┌──────────┐
│   User   │──────────────▶│  Server  │
└──────────┘               └──────────┘
     │                          │
     │ joins                    │ contains
     ▼                          ▼
┌──────────────┐          ┌──────────┐
│ ServerMember │          │ Channel  │
└──────────────┘          └──────────┘
                               │
                               │ has
                               ▼
                    ┌────────────────────┐
                    │  ChannelMessage    │
                    │    (MongoDB)       │
                    └────────────────────┘
```

### Resume des relations

- `User` ↔ `Server` via `ServerMember` (many-to-many)
- `User` → `Server` via `owner_id` (one-to-many)
- `Server` → `Channel` (one-to-many)
- `Server` → `Invite` (one-to-many)
- `Channel` → `channel_messages` (one-to-many, cross-DB)

---

## 5. Enumerations

### `UserStatus`

| Valeur      | Description      |
| ----------- | ---------------- |
| `ONLINE`    | Connecte         |
| `OFFLINE`   | Deconnecte       |
| `DND`       | Ne pas deranger  |
| `INVISIBLE` | Invisible        |

### `MemberRole`

| Valeur   | Description              |
| -------- | ------------------------ |
| `OWNER`  | Proprietaire du serveur  |
| `ADMIN`  | Administrateur           |
| `MEMBER` | Membre standard          |

---

## 6. Notes importantes

### Cross-database

- Les references entre PostgreSQL et MongoDB utilisent des **UUID partages**
- Pas de FK cross-DB, les relations sont resolues par la **couche metier**

### Soft delete

- Les messages ne sont jamais supprimes physiquement
- `deleted_at` et `deleted_by` permettent de tracer la suppression
