# Resume Builder

Local application builder that turns raw job postings into tailored resumes, structured analyses, and cover letters.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-4B5563?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white)
![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS3](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)
![Nix](https://img.shields.io/badge/NixOS-5277C3?style=for-the-badge&logo=nixos&logoColor=white)

[![Backend CI](https://github.com/RomeoCavazza/open-cv/actions/workflows/backend.yml/badge.svg)](https://github.com/RomeoCavazza/open-cv/actions/workflows/backend.yml)
[![Frontend CI](https://github.com/RomeoCavazza/open-cv/actions/workflows/frontend.yml/badge.svg)](https://github.com/RomeoCavazza/open-cv/actions/workflows/frontend.yml)

This project is a local application-generation engine. It ingests job postings, structures them, connects them to a candidate profile stored in the database, and produces high-fidelity deliverables through a **Rust + Axum** backend, a **PostgreSQL** database, and structured calls to **AI models** (Claude, GPT, or local models).

## Previews

| Resume | Cover Letter |
| :---: | :---: |
| ![Resume Preview](docs/assets/preview-resume.png) | ![Letter Preview](docs/assets/preview-letter.png) |

---

## Project Architecture

```text
.
├── crates/             # Rust workspace: domain, ports, application, adapters, api
├── docs/               # Documentation and usage guides
├── migrations/         # SQL schema source of truth
├── web/                # Static frontend and document renderers
│   ├── resume/         # Resume renderer
│   ├── cover-letter/   # Cover letter renderer
│   ├── restitution/    # Job analysis renderer
│   └── templates/      # JSON rendering fallbacks
├── flake.nix           # Nix development environment
├── Justfile            # Common commands
└── README.md
```

---

## How It Works

The workflow is driven by the Rust backend and can be summarized in five main steps:

1. **Ingestion**: a job posting is sent to the API, deduplicated, normalized, and stored in `offres`.
2. **Analysis**: the posting is structured to extract responsibilities, stack, and key signals.
3. **Context selection**: the active profile and its chunks are loaded from PostgreSQL.
4. **Generation**: the application produces a structured analysis, a tailored resume, and a targeted cover letter.
5. **Rendering**: the static frontend loads the JSON payloads and displays them through printable HTML renderers.

### Installation

```bash
# Enter the development environment
nix develop

# Initialize local Postgres (first time only)
just db-init

# Start Postgres
just db-up

# Apply migrations
just migrate

# Start the Axum API
just dev
```

The application is then available at `http://localhost:8000`.

---

## Technical Stack

- **Backend**: Rust, Axum, Tokio, hexagonal architecture.
- **Database**: PostgreSQL 16, `sqlx`, `pgvector`, `pgcrypto`, `pg_trgm`.
- **AI**: Multi-provider LLM support (Anthropic Claude, OpenAI, Ollama).
- **Frontend**: native HTML, CSS, and JavaScript, with iframes to isolate document rendering.
- **Environment**: Nix, Just, Cargo workspace.

---

## System Workflow

```mermaid
flowchart LR
    User[User]
    UI[Static web frontend]
    API[Backend API Rust Axum]
    APP[Application use cases]
    LLM[LLM API / Local AI]
    PG[(PostgreSQL)]
    Render[HTML renderers: Resume, Cover Letter, Analysis]

    User --> UI
    UI --> API
    UI --> Render
    API --> APP
    APP --> PG
    APP --> LLM
    PG --> API
    API --> UI
```

---

## Backend / Frontend Diagram

```mermaid
flowchart TD
    subgraph Phase1["1. Local bootstrap"]
        A[nix develop] --> B[just db-init / just db-up]
        B --> C[just migrate]
        C --> D[just dev]
    end

    subgraph Phase2["2. Job intake"]
        E[User pastes a URL or raw job text] --> F["POST /api/ingest"]
        F --> G[Axum parses the request]
        G --> H[Intake use case]
        H --> I[Store offer in PostgreSQL]
    end

    subgraph Phase3["3. Generation"]
        J["POST /api/instances/:slug/generate"] --> K[GenerateApplicationUseCase]
        K --> L[Load active profile and chunks]
        L --> M[Select relevant context]
        M --> N[AI generates analysis, resume, and cover letter]
        N --> O[Persist instance in PostgreSQL]
        O --> P["GET /api/instances/:slug"]
    end

    subgraph Phase4["4. Rendering"]
        Q[Frontend updates iframe target] --> R["resume/index.html"]
        Q --> S["cover-letter/index.html"]
        Q --> T["restitution/index.html"]
        P --> Q
    end

    PG[(PostgreSQL)]
    LLM[(AI Models / API)]

    I --> PG
    L --> PG
    O --> PG
    N --> LLM
```

---

## Workspace Board

This board captures the current product direction at a glance: the intake dashboard, the application workspace with live document preview, the profile editor, the visual system, and the technical notes that connect the frontend to the Rust backend and database model.

It is intended as a working overview of the project rather than a polished marketing mockup. It shows how the interface, rendering pipeline, and implementation details fit together in the same workspace.

![Workspace Board](docs/assets/canva.png)

---

## Documentation

- [docs/README.md](docs/README.md): documentation index
- [docs/instructions.md](docs/instructions.md): setup and common commands
- [docs/how_it_works.md](docs/how_it_works.md): technical overview
- [docs/blueprint.md](docs/blueprint.md): architecture and generation pipeline
- [docs/design.md](docs/design.md): visual direction for the interface

---

*This project is built around a local Rust backend to industrialize the application workflow without losing the quality of tailored deliverables.*
