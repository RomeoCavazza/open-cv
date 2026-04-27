<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Docker_&_Orchestration-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-60--67-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Focus-Microservices-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Containerization & Microservices

Mastering the paradigm shift of containerization: isolating applications, managing dependencies, and orchestrating complex distributed systems with Docker and Docker Compose.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **Container Mastery**: Building optimized images with multi-stage Dockerfiles.
> - **Orchestration**: Designing complex inter-service networks with **Docker Compose**.
> - **Architecture**: Deploying the **Popeye** project (distributed voting app).
> - **Persistence**: Managing volumes, bind mounts, and database state.

## Technical Core

| Layer | Implementation |
|---|---|
| **Engine** | ![Docker](https://img.shields.io/badge/Engine-Docker-2496ED?style=flat-square&logo=docker&logoColor=white) |
| **Orchestration** | ![Docker Compose](https://img.shields.io/badge/Orchestrator-Docker_Compose-2496ED?style=flat-square&logo=docker&logoColor=white) |
| **Languages** | ![Node.js](https://img.shields.io/badge/Runtime-Node.js-339933?style=flat-square&logo=nodedotjs&logoColor=white) ![Python](https://img.shields.io/badge/Logic-Python_3-3776AB?style=flat-square&logo=python&logoColor=white) |
| **Databases** | ![PostgreSQL](https://img.shields.io/badge/DB-PostgreSQL-336791?style=flat-square&logo=postgresql&logoColor=white) ![Redis](https://img.shields.io/badge/Cache-Redis-DC382D?style=flat-square&logo=redis&logoColor=white) |

### Microservices Architecture (Project Popeye)

```mermaid
graph TD
    User((User)) --> Vote[Vote App: Flask]
    Vote --> Redis[(Redis)]
    Worker[Worker: Java] --> Redis
    Worker --> DB[(PostgreSQL)]
    Result[Result App: Node.js] --> DB
    Result --> FinalUser((Final User))
```

---

## Chronological Journey

- **Day 60-62**: Docker Fundamentals: basic images, containers, and volumes.
- **Day 63-64**: **Bootstrap Project**: Containerizing a Node.js stack with PostgreSQL.
- **Day 65-67**: **Project Popeye**: Orchestrating a 5-tier microservices architecture.

---

## Skills developed

- **Isolation Excellence**: Resolving "it works on my machine" through containerization.
- **Distributed Thinking**: Understanding communication protocols between isolated services.
- **Optimized Builds**: Implementing multi-stage builds to minimize image size.
- **Infrastructure as Logic**: Defining complex stack dependencies in YAML.

