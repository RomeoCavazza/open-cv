# Backend Cleanup Plan

This document is the practical plan I would follow to clean the backend and rebuild it on a healthier base. It is intentionally operational: the goal is to reduce architectural noise, isolate risk, and keep behavior stable while the code is being simplified.

## 1. Goal

The backend should end up with these properties:

- clear orchestration boundaries in `application`
- thin HTTP handlers in `api`
- smaller adapters with one responsibility each
- explicit domain types instead of JSON-heavy intermediate state
- tests that protect the generation and chat flows before refactoring continues
- quality gates that catch regressions early

The main backend hotspots today are `crates/application/src/generate.rs`, `crates/application/src/chat.rs`, `crates/adapters/postgres/src/lib.rs`, and some handlers in `crates/api/src/handlers`.

## 2. Guiding Rules

1. Freeze behavior before restructuring.
2. Refactor from the outside in: tests first, then boundaries, then internals.
3. Move logic downward, not upward. API handlers must not grow.
4. Split by responsibility, not by file size.
5. Keep public contracts stable until the new structure is covered by tests.

## 3. Phase 0: Baseline and Safety Net

Before changing structure, lock down the current behavior.

### What to add

- characterization tests for generation
- characterization tests for chat history persistence
- API tests for the main handlers
- one or two data-layer tests for the most important repository paths

### Priority files

- `crates/application/src/generate.rs`
- `crates/application/src/chat.rs`
- `crates/api/src/handlers/ingest.rs`
- `crates/api/src/handlers/profile.rs`
- `crates/api/src/main.rs`

### Exit condition

- The current behavior is documented by tests and can be refactored safely.

## 4. Phase 1: Thin API Layer

The `api` crate should become a transport layer only.

### Target state

- handlers parse input
- handlers call one application use case
- handlers translate application errors into HTTP errors
- no business logic in handlers
- no persistence decisions in handlers

### What to extract

- request validation helpers
- response mapping helpers
- common error mapping
- route-level orchestration that does not belong to business logic

### Priority files

- `crates/api/src/handlers/ingest.rs`
- `crates/api/src/handlers/profile.rs`
- `crates/api/src/handlers/offres.rs`
- `crates/api/src/main.rs`

### Exit condition

- A handler can be read in one minute and the use case is visible immediately.

## 5. Phase 2: Split the Application Monoliths

This is the highest-value refactor.

### `generate.rs`

Break it into focused modules such as:

- retrieval / chunk selection
- reranking
- planning
- deliverable generation
- validation and normalization
- persistence orchestration

### `chat.rs`

Break it into focused modules such as:

- conversation loading and storage
- retrieval-augmented context assembly
- prompt construction
- output extraction and validation
- mutation handling for the profile / instance state

### Target state

- one top-level use case per behavior
- helper modules with small names and explicit responsibilities
- no “god function” that does everything in one pass

### Priority files

- `crates/application/src/generate.rs`
- `crates/application/src/chat.rs`

### Exit condition

- Each use case is readable as orchestration, not as a pile of mixed concerns.

## 6. Phase 3: Split the Postgres Adapter

The Postgres adapter should be organized by domain, not by one large file.

### Suggested split

- offer repository
- profile repository
- instance repository
- chat / message persistence
- annexes
- embeddings / chunks
- SQL helpers and row mappers

### Target state

- small modules with one repository or one aggregate each
- shared SQL helpers extracted once
- row-to-domain mapping kept close to the repository that owns it

### Priority files

- `crates/adapters/postgres/src/lib.rs`

### Exit condition

- The adapter tree reflects the domain tree instead of one giant persistence blob.

## 7. Phase 4: Clean the Port Boundaries

The ports crate should stay strict and boring.

### What to enforce

- one contract for LLMs
- one contract for persistence per aggregate boundary
- no leaking of adapter-specific types into the application layer
- no serde_json convenience creeping into business logic unless it is truly a boundary type

### Priority files

- `crates/ports/src/llm.rs`
- repository traits in `crates/ports/src`

### Exit condition

- The application compiles against stable abstractions and does not know adapter details.

## 8. Phase 5: Make Domain Types More Explicit

Right now some complexity comes from using JSON too early.

### What to do

- move important payloads to explicit Rust structs
- keep JSON only at the outer boundary when needed
- avoid storing transient workflow state in loosely typed structures
- normalize data once, then pass typed values around

### Priority areas

- generation inputs and outputs
- chat payloads
- persisted instance state
- profile fragments used by RAG

### Exit condition

- The application core reads like typed business logic, not a serde pipeline.

## 9. Phase 6: Dependency and Quality Cleanup

After the structure is clearer, clean the dependency graph and keep it clean.

### What to do

- resolve duplicate crate versions flagged by `cargo deny`
- remove unused dependencies found by `cargo udeps`
- keep `cargo bloat` as a periodic check, not a guess-based panic tool
- keep `tokei` as a trend indicator, not a goal by itself

### Current gate state

- `just audit` exists and already includes fmt, clippy, cargo deny, cargo udeps, cargo bloat, and tokei.
- the lockfile still needs dependency consolidation work if we want `cargo deny` to stay fully green with duplicate-version denial.

### Exit condition

- the backend can pass the quality gates without special-casing or noise.

## 10. Recommended Order of Attack

If I were doing this for real, I would follow this sequence:

1. Add tests around generation and chat.
2. Split `generate.rs`.
3. Split `chat.rs`.
4. Thin the API handlers.
5. Split the Postgres adapter.
6. Tighten port boundaries.
7. Replace early JSON with typed structs where it helps.
8. Resolve dependency duplication and unused crates.
9. Re-run audit gates and keep them in CI.

## 11. What Success Looks Like

The backend is healthy when:

- each file has one obvious job
- the application layer is the orchestration layer, not a dumping ground
- API code is thin and predictable
- repository code is split by domain and easy to navigate
- tests protect the behavior before each refactor step
- quality gates stay green without heroic fixes

## 12. Execution Checklist

Use this as the working checklist when starting the cleanup.

### Step 1: Freeze behavior

- [ ] Add characterization tests for `crates/application/src/generate.rs`
- [ ] Add characterization tests for `crates/application/src/chat.rs`
- [ ] Add handler-level tests for `crates/api/src/handlers/ingest.rs`
- [ ] Add handler-level tests for `crates/api/src/handlers/profile.rs`
- [ ] Add at least one repository test around the most critical Postgres path

### Step 2: Split generation

- [ ] Extract retrieval / chunk selection from `generate.rs`
- [ ] Extract reranking and planning from `generate.rs`
- [ ] Extract validation and persistence orchestration from `generate.rs`
- [ ] Keep the public use case API stable while the internals move

### Step 3: Split chat

- [ ] Extract conversation loading and storage from `chat.rs`
- [ ] Extract RAG context assembly from `chat.rs`
- [ ] Extract prompt construction and output validation from `chat.rs`
- [ ] Keep mutation handling isolated and explicit

### Step 4: Thin the API

- [ ] Move any remaining business logic out of `crates/api/src/handlers`
- [ ] Add shared request / response mapping helpers where handlers repeat themselves
- [ ] Keep handlers focused on transport and error translation only

### Step 5: Split persistence

- [ ] Split `crates/adapters/postgres/src/lib.rs` by aggregate or repository
- [ ] Extract row mappers close to the repository that owns them
- [ ] Centralize SQL helpers if they are duplicated

### Step 6: Tighten boundaries

- [ ] Review `crates/ports/src/llm.rs` and related ports for accidental coupling
- [ ] Replace early `serde_json` usage with typed structs where the boundary is stable
- [ ] Make domain types explicit enough that use cases read as business logic

### Step 7: Clean dependencies

- [ ] Resolve duplicate crate versions flagged by `cargo deny`
- [ ] Remove dependencies flagged by `cargo udeps`
- [ ] Check `cargo bloat` after the structural split, not before

### Step 8: Lock it in

- [ ] Keep `just audit` green
- [ ] Keep `just test` green
- [ ] Keep backend CI running through the Nix shell
- [ ] Re-run the full audit after each major split
