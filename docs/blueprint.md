# Blueprint — Backend and AI Architecture

This document describes the current architecture of the AI-native application builder (Rust + Postgres + static frontend). It is the reference document for the project's structure and technical choices.

## 1. Assumptions and Technical Choices

The system is built around three core deliverables:
1. **Job analysis**: a structured extraction containing summary, fit, implicit signals, and attention points.
2. **Tailored resume**: structured JSON designed to match the target job posting.
3. **Cover letter**: a targeted letter composed of meaningful semantic sections.

### Technical Stack

- **Backend language**: Rust
- **HTTP framework**: Axum + Tokio
- **Database**: PostgreSQL with `pgvector`, `pgcrypto`, and `pg_trgm`, accessed through `sqlx`
- **Architecture**: Hexagonal architecture (Domain, Ports, Adapters, Application, API)
- **Primary AI models**: Anthropic Claude, OpenAI GPT-4o, and local models via Ollama.
- **Embeddings**: dedicated interface in `ports`, currently using `nomic-embed-text` (local) or OpenAI embeddings.
- **Frontend**: static HTML/CSS/JS served by Axum, with iframes for document isolation and rendering.
- **Environment**: Nix + Just

## 2. Hexagonal Architecture

The architecture is split into Rust crates inside a Cargo workspace:

```
alternance/
├── crates/
│   ├── domain/           # Core business types, no infrastructure dependency
│   ├── ports/            # Traits required by the domain and application
│   ├── adapters/         # Concrete implementations of those ports
│   │   ├── postgres/     # Persistence through sqlx (JSONB + pgvector)
│   │   ├── llm_claude/   # Anthropic API client
│   │   ├── llm_openai/   # OpenAI / OpenRouter compatible client
│   │   ├── llm_ollama/   # Local LLM support
│   │   ├── scraper_http/ # Basic HTTP scraping
│   ├── application/      # Use cases such as intake and generation
│   └── api/              # Axum HTTP entrypoint
├── web/                  # Static frontend
└── migrations/           # SQL schema and migrations
```

## 3. Data Model (PostgreSQL)

The database stores the full application context. Local JSON and Markdown files are no longer treated as the source of truth.

- **`offres`**: stores the source URL, deduplication hash, raw text, and structured AI output (`JSONB`).
- **`profils`**: stores candidate profiles, with a single active profile at a time in the current setup.
- **`chunks`**: stores profile pieces (experiences, skills) with `embedding (1024)` vectors for RAG.
- **`instances`**: links a profile to an offer; stores generated analysis, resume, and cover letter.
- **`llm_calls`**: [Planned] observability table for AI costs, latency, and token usage.

## 4. AI Generation Pipeline

Application generation runs through the central `GenerateApplicationUseCase`:

1. **Retrieve**: fetch the most relevant profile chunks for the target job posting.
2. **Rerank**: let the LLM score and filter those chunks.
3. **Plan**: build an application strategy for the current offer.
4. **Analysis**: generate a structured analysis of the offer.
5. **Resume**: generate the structured resume payload.
6. **Cover Letter**: generate the structured cover letter payload.
7. **Validate / Persist**: apply basic business validation and save the instance to the database.

## 5. The `LlmClient` Trait

All AI integration is abstracted behind a single trait in `crates/ports/src/llm.rs`:

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Free-form text generation.
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError>;

    /// Structured generation: a JSON schema goes in, JSON comes out.
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError>;

    fn name(&self) -> &'static str;
}
```

This loose coupling makes it possible to swap providers or models without changing the application core. LLM calls prioritize structured output so the model returns data that matches the JSON expected by the frontend.

## 6. Frontend

The frontend is intentionally lightweight.
- **Static HTML + JS + CSS**: there is no dedicated frontend build pipeline in the repository.
- **Iframe isolation**: the resume, cover letter, and analysis renderers stay isolated from the main UI.
- **Static serving through Axum**: the backend serves the `web/` directory directly and exposes the REST API consumed by the interface.
