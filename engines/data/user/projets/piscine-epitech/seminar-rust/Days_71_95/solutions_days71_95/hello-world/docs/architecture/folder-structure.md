# Structure des dossiers - Hello World RTC

## Vue d'ensemble

```
helloworld-rtc/
├── README.md
├── .gitignore
├── docs/                        # Documentation
├── backend/                     # Rust + Axum
└── frontend/                    # Next.js + TypeScript
```

---

## Backend (Rust / Axum)

```
backend/
├── Cargo.toml
├── .env.example
│
├── migrations/                  # SQLx migrations (Postgres)
│   ├── 0001_init.sql
│   ├── 0002_indexes.sql
│   └── 0003_seed.sql           # optionnel
│
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── config.rs
│   ├── router.rs
│   │
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── rate_limit.rs       # optionnel
│   │   └── request_id.rs       # optionnel
│   │
│   ├── kernel/
│   │   ├── mod.rs
│   │   ├── ids.rs
│   │   ├── error.rs
│   │   ├── auth_ctx.rs
│   │   ├── pagination.rs
│   │   ├── time.rs
│   │   ├── events.rs
│   │   └── validation.rs
│   │
│   ├── modules/
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── dto.rs
│   │   │   ├── ports.rs
│   │   │   ├── usecases.rs
│   │   │   ├── http.rs
│   │   │   └── domain.rs
│   │   │
│   │   ├── servers/
│   │   │   └── ... (même structure)
│   │   │
│   │   ├── channels/
│   │   │   └── ...
│   │   │
│   │   ├── messages/
│   │   │   └── ...
│   │   │
│   │   ├── dm/
│   │   │   └── ...
│   │   │
│   │   ├── contacts/
│   │   │   └── ...
│   │   │
│   │   └── realtime/
│   │       ├── mod.rs
│   │       ├── protocol.rs
│   │       ├── gateway.rs
│   │       ├── presence.rs
│   │       ├── typing.rs
│   │       ├── bus.rs
│   │       └── mapping.rs
│   │
│   └── infra/
│       ├── db/
│       │   ├── mod.rs
│       │   │
│       │   ├── pg/
│       │   │   ├── mod.rs
│       │   │   ├── pool.rs
│       │   │   ├── repos/
│       │   │   │   ├── user_repo.rs
│       │   │   │   ├── server_repo.rs
│       │   │   │   ├── channel_repo.rs
│       │   │   │   ├── dm_repo.rs
│       │   │   │   ├── contacts_repo.rs
│       │   │   │   ├── invite_repo.rs
│       │   │   │   ├── membership_repo.rs
│       │   │   │   └── message_meta_repo.rs
│       │   │   └── unit_of_work.rs
│       │   │
│       │   └── mongo/
│       │       ├── mod.rs
│       │       ├── client.rs
│       │       ├── collections/
│       │       │   ├── channel_messages.rs
│       │       │   └── dm_messages.rs
│       │       ├── indexes.rs
│       │       └── message_history_repo.rs
│       │
│       ├── auth/
│       │   ├── mod.rs
│       │   ├── jwt.rs
│       │   ├── password.rs
│       │   └── sessions.rs
│       │
│       └── observability/
│           ├── mod.rs
│           ├── logging.rs
│           └── metrics.rs          # optionnel
│
└── tests/
    ├── unit/
    │   ├── servers_transfer_owner.rs
    │   ├── servers_leave_owner_forbidden.rs
    │   ├── messages_delete_rules.rs
    │   └── permissions_matrix.rs
    └── integration/
        ├── postgres_repos.rs
        ├── mongo_history.rs
        └── ws_contract.rs
```

---

## Frontend (Next.js)

```
frontend/
├── package.json
├── next.config.ts
├── tsconfig.json
├── postcss.config.mjs
├── .env.local.example
│
├── app/
│   ├── globals.css
│   ├── layout.tsx
│   ├── page.tsx
│   │
│   ├── (auth)/
│   │   ├── login/page.tsx
│   │   └── signup/page.tsx
│   │
│   └── (app)/
│       ├── servers/page.tsx
│       ├── servers/[serverId]/page.tsx
│       ├── servers/[serverId]/channels/[channelId]/page.tsx
│       ├── dm/page.tsx
│       └── dm/[dmId]/page.tsx
│
├── components/
│   ├── layout/
│   │   ├── SidebarServers.tsx
│   │   ├── SidebarChannels.tsx
│   │   ├── SidebarDMs.tsx
│   │   ├── Topbar.tsx
│   │   └── MemberList.tsx
│   │
│   ├── chat/
│   │   ├── MessageList.tsx
│   │   ├── MessageItem.tsx
│   │   ├── Composer.tsx
│   │   ├── TypingIndicator.tsx
│   │   └── PresenceBadge.tsx
│   │
│   ├── modals/
│   │   ├── CreateServerModal.tsx
│   │   ├── JoinServerModal.tsx
│   │   ├── CreateChannelModal.tsx
│   │   ├── InviteModal.tsx
│   │   └── ManageRolesModal.tsx
│   │
│   └── ui/
│       ├── Button.tsx
│       ├── Input.tsx
│       ├── Modal.tsx
│       ├── Toast.tsx
│       └── Skeleton.tsx
│
├── lib/
│   ├── api/
│   │   ├── http.ts
│   │   ├── auth.ts
│   │   ├── servers.ts
│   │   ├── channels.ts
│   │   ├── messages.ts
│   │   ├── dm.ts
│   │   └── contacts.ts
│   │
│   ├── ws/
│   │   ├── protocol.ts
│   │   └── useChatSocket.ts
│   │
│   ├── state/
│   │   ├── authStore.ts
│   │   ├── chatStore.ts
│   │   └── uiStore.ts
│   │
│   └── utils/
│       ├── time.ts
│       ├── validation.ts
│       └── debounce.ts
│
├── public/
│   ├── favicon.ico
│   ├── logo.png
│   └── background.png
│
└── styles/
    └── globals.css
```

---

## Documentation

```
docs/
├── README.md
├── specifications/
│   ├── requirements.md
│   ├── grading-criteria.md
│   └── moscow.md
├── architecture/
│   ├── overview.md
│   ├── database.md
│   └── folder-structure.md
└── uml/
    ├── classes.puml
    ├── database-schema.puml
    └── entities.md
```

