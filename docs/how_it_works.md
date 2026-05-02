# How It Works

The system is designed as a local application that combines Rust, PostgreSQL, and a static frontend to generate AI-assisted job applications.

The goal is straightforward: turn raw job postings into a usable application package with a structured analysis, a tailored resume, and a cover letter.

## 1. Hexagonal Architecture

The core of the system is written in Rust and split into five crates following **Ports and Adapters** principles:

1. **`domain`**: pure data models such as `Offre`, `Instance`, `Resume`, and `CoverLetter`, with no infrastructure dependency.
2. **`ports`**: traits used by the application to interact with external systems, such as `LlmClient` and repository interfaces.
3. **`application`**: business logic and orchestration. This is where the RAG pipeline runs: load offer and profile, retrieve relevant chunks, and call the LLM for structured generation.
4. **`adapters`**: concrete implementations of ports, such as `llm_claude` (Anthropic), `llm_openai` (GPT), or `llm_ollama` (Local), and `postgres` for persistence.
5. **`api`**: the HTTP interface. An [Axum](https://github.com/tokio-rs/axum) server exposes REST routes and serves the static frontend.

## 2. AI Pipeline

The main strength of the backend is its structured approach to AI. Instead of relying on free-form text generation, it uses **structured JSON generation**.

- The application builds a request around the exact **JSON Schema** expected for a resume or cover letter.
- The LLM is constrained to answer according to that structure.
- Rust then deserializes the returned JSON before storing it in the database or serving it to the frontend.

The pipeline follows these steps:
1. **Offer analysis** and retrieval of the active profile.
2. **Retrieval and reranking** of relevant profile chunks.
3. **Application planning**.
4. **Deliverable generation**: analysis, resume, and cover letter.
5. **Basic validation and persistence** in the database.

## 3. Frontend

The frontend lives in `/web` and is deliberately built with **plain HTML, CSS, and JavaScript**.

- It includes a main dashboard and separate document renderers embedded through **iframes**.
- **Why iframes?** They provide CSS isolation. Resume rendering needs absolute units (`mm`) for clean A4 output, and iframes prevent the main interface styles from leaking into printable documents.
- When the user selects an offer, the frontend updates the iframe source and renders the JSON payload returned by the backend.

## 4. Data Storage

All offers and generation history are stored in **PostgreSQL 16**.
Thanks to `flake.nix` and the `Justfile`, the database can run entirely from the local `.pg/` directory without depending on a globally installed service.

Extensions such as `pgvector` prepare the system for semantic retrieval over profile chunks.
