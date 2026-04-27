<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Rust_Systems-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-71--95-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Performance-Memory_Safety-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Systems Programming & Modern Web (Rust)

Entering the elite tier of systems engineering: mastering **Ownership**, **Borrowing**, and **Lifetimes** to build zero-cost abstractions with absolute memory safety.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **Ownership Mastery**: Deep dive into the borrow checker and memory management.
> - **Full-Stack Rust**: Building decoupled architectures with Actix/Rocket and Yew/Leptos.
> - **WASM Compilation**: Bringing high-performance code to the browser.
> - **Database Synergy**: Type-safe database interactions with PostgreSQL.

## Technical Core

| Layer | Implementation |
|---|---|
| **Language** | ![Rust](https://img.shields.io/badge/Language-Rust-000000?style=flat-square&logo=rust&logoColor=white) |
| **Backend** | ![Actix](https://img.shields.io/badge/API-Actix_Web-2496ED?style=flat-square) ![Rocket](https://img.shields.io/badge/API-Rocket-FF4B4B?style=flat-square) |
| **Frontend** | ![Leptos](https://img.shields.io/badge/UI-Leptos_/_WASM-f78516?style=flat-square) ![Yew](https://img.shields.io/badge/UI-Yew_/_WASM-4EAA25?style=flat-square) |
| **Database** | ![PostgreSQL](https://img.shields.io/badge/DB-PostgreSQL-336791?style=flat-square&logo=postgresql&logoColor=white) |

### Decoupled Rust Architecture

```mermaid
graph TD
    Client[Frontend: WASM / Leptos] <--> API[Backend: Actix Web]
    API <--> Logic[Business Logic & Traits]
    Logic <--> DB[(PostgreSQL)]
    Build[Cargo / Docker] --> API
```

---

## Chronological Journey

- **Day 71-73**: **Bootstrap**: Ownership fundamentals, borrowing, lifetimes, and pattern matching.
- **Day 74-76**: **Core API**: Building RESTful services with Actix and robust error handling.
- **Day 77-80**: **Project: Hello World**: A complete, containerized full-stack Rust application.

---

## Skills developed

- **Memory Fearlessness**: Writing complex systems without fearing Segfaults or Race Conditions.
- **Zero-Cost Abstractions**: Leveraging traits and generics for high-level logic with C-level speed.
- **Type-Safe Full-Stack**: Ensuring data integrity from the DB schema to the UI state.
- **Modern Build Lifecycle**: Mastering Cargo, cross-compilation, and Rust Dockerization.
