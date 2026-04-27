# Description des entites - Hello World RTC

## Enumerations

### `UserStatus`
Statut de presence d'un utilisateur :

| Valeur | Description |
|--------|-------------|
| `ONLINE` | Connecte et disponible |
| `OFFLINE` | Deconnecte |
| `DND` | Ne pas deranger |
| `INVISIBLE` | Invisible pour les autres |

### `MemberRole`
Role d'un utilisateur dans un serveur :

| Valeur | Description | Permissions |
|--------|-------------|-------------|
| `OWNER` | Proprietaire du serveur | Toutes + transfert propriete |
| `ADMIN` | Administrateur | Gestion canaux, invitations, moderation |
| `MEMBER` | Membre standard | Lecture/envoi messages |

---

## Entites PostgreSQL

### `User`
Represente un compte utilisateur.

| Attribut | Type | Description |
|----------|------|-------------|
| `id` | UUID | Identifiant unique |
| `email` | String | Email de connexion (unique) |
| `password_hash` | String | Mot de passe chiffre (bcrypt) |
| `username` | String | Pseudonyme affiche |
| `avatar_url` | String? | URL de l'avatar (optionnel) |
| `status` | UserStatus | Statut de presence |
| `created_at` | DateTime | Date de creation du compte |

---

### `Server`
Represente un serveur (communaute).

| Attribut | Type | Description |
|----------|------|-------------|
| `id` | UUID | Identifiant unique |
| `name` | String | Nom du serveur |
| `owner_id` | UUID | FK → User proprietaire |
| `created_at` | DateTime | Date de creation |
| `updated_at` | DateTime | Date de modification |

**Contrainte metier** : Un seul Owner par serveur.

---

### `ServerMember`
Table de liaison entre `User` et `Server`.

| Attribut | Type | Description |
|----------|------|-------------|
| `server_id` | UUID | FK → Server |
| `user_id` | UUID | FK → User |
| `role` | MemberRole | Role dans le serveur |
| `joined_at` | DateTime | Date d'adhesion |

**Cle primaire composee** : `(server_id, user_id)`

**Contrainte** : Un utilisateur ne peut pas etre deux fois dans le meme serveur.

---

### `Channel`
Canal textuel appartenant a un serveur.

| Attribut | Type | Description |
|----------|------|-------------|
| `id` | UUID | Identifiant unique |
| `server_id` | UUID | FK → Server parent |
| `name` | String | Nom du canal |
| `position` | Int | Ordre d'affichage |
| `created_at` | DateTime | Date de creation |
| `updated_at` | DateTime | Date de modification |

---

### `Invite`
Invitation pour rejoindre un serveur.

| Attribut | Type | Description |
|----------|------|-------------|
| `id` | UUID | Identifiant unique |
| `server_id` | UUID | FK → Server concerne |
| `code` | String | Code d'invitation unique |
| `created_by` | UUID | FK → User createur |
| `expires_at` | DateTime? | Date d'expiration (optionnel) |
| `max_uses` | Int? | Nombre max d'utilisations (optionnel) |
| `uses_count` | Int | Compteur d'utilisations |
| `created_at` | DateTime | Date de creation |

---

## Entites MongoDB

### `ChannelMessage`
Message envoye dans un canal.

| Attribut | Type | Description |
|----------|------|-------------|
| `_id` | ObjectId | ID MongoDB interne |
| `message_id` | UUID | ID unique (reference cross-DB) |
| `server_id` | UUID | Reference serveur |
| `channel_id` | UUID | Reference canal |
| `author_id` | UUID | Reference auteur |
| `content` | String | Contenu du message |
| `created_at` | DateTime | Date d'envoi |
| `edited_at` | DateTime? | Date d'edition (optionnel) |
| `deleted_at` | DateTime? | Date de suppression logique |
| `deleted_by` | UUID? | Utilisateur ayant supprime |

**Soft delete** : Le message n'est jamais supprime physiquement.

**Index** : `(channel_id, created_at)` pour les requetes paginees.

---

## Relations

```
User ──────┬───────────────────┬──────────────────┐
           │ owns              │ joins            │ writes
           ▼                   ▼                  ▼
        Server ◄───── ServerMember        ChannelMessage
           │                                      ▲
           │ contains                             │
           ▼                                      │
        Channel ──────────────────────────────────┘
           │
           │ has
           ▼
        Invite
```

### Resume
- Un `User` peut etre **proprietaire** de plusieurs `Server`
- Un `User` peut **appartenir** a plusieurs `Server` via `ServerMember`
- Un `Server` **contient** plusieurs `Channel`
- Un `Server` peut avoir plusieurs `Invite`
- Un `Channel` **contient** plusieurs `ChannelMessage`
- Un `User` peut **ecrire** plusieurs `ChannelMessage`
