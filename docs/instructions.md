# Usage Instructions

This guide explains how to start and use the application builder locally.

## 1. Prerequisites

Make sure **Nix** is installed on your system. The project uses Nix flakes to provide a reproducible development environment with Rust, Cargo, Just, PostgreSQL, and related tooling.

## 2. Enter the Environment

From the project root, run:
```bash
nix develop
```
This downloads and configures the required dependencies. When it completes, you should see the `alternance dev shell ready` message.

## 3. Initialize the Database

If this is your first time running the project, initialize the local PostgreSQL data directory:
```bash
just db-init
```
This creates the hidden `.pg/` directory at the project root.

## 4. Daily Workflow

Once inside `nix develop`, the daily workflow is simple:

1. **Start PostgreSQL**
   ```bash
   just db-up
   ```
2. **Apply migrations**
   ```bash
   just migrate
   ```
3. **Start the API server**
   ```bash
   just dev
   ```
   `just dev` uses `cargo watch`, so the server reloads automatically when Rust files change.

The server starts on **http://localhost:8000**.

## 5. Access the Application

Open your browser and go to **[http://localhost:8000](http://localhost:8000)**.
The API serves the static `/web` directory directly.

## 6. Environment Variables

The project uses a `.env` file to store the keys required for generation.
1. Copy the example file: `cp .env.example .env`
2. Edit `.env` and add your keys:
   ```env
   ANTHROPIC_API_KEY=sk-ant-api03-...
   ```

## Useful Commands

- `just db-down`: stop the PostgreSQL server cleanly.
- `rm -rf target`: remove build artifacts if you want to reclaim disk space.
- `rm -rf .pg`: remove the local database only after stopping Postgres with `just db-down`.
- `cargo check --workspace`: verify that the Rust workspace still compiles.
