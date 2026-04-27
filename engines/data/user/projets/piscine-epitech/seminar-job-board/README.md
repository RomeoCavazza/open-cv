<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Job_Board-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-21--30-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Scale-Industrial_Project-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Industrial Web Project (Job Board)

The first large-scale structural project: architecting a complete recruitment platform from scratch, integrating complex business logic with a dynamic interface.

---

> [!IMPORTANT]
> **MVP Scope**: 
> - **User Management**: Authentication, profiles (Candidates/Recruiters).
> - **Job Lifecycle**: Creation, editing, deletion, and advanced application tracking.
> - **API-First Design**: Decoupled REST architecture for future scalabilty.
> - **Security Baseline**: Protection against CSRF, SQL injection, and XSS.

## Technical Core

| Layer | Implementation |
|---|---|
| **Backend** | ![PHP 8](https://img.shields.io/badge/Server-PHP_8-777BB4?style=flat-square&logo=php&logoColor=white) ![PDO](https://img.shields.io/badge/DB_Access-PDO-4479A1?style=flat-square) |
| **Database** | ![MySQL](https://img.shields.io/badge/Engine-MySQL-4479A1?style=flat-square&logo=mysql&logoColor=white) |
| **API** | ![REST API](https://img.shields.io/badge/Architecture-REST_API-02569B?style=flat-square) ![JSON](https://img.shields.io/badge/Format-JSON-000000?style=flat-square&logo=json&logoColor=white) |
| **Client** | ![HTML5/CSS3](https://img.shields.io/badge/View-HTML5_/_CSS3-E34F26?style=flat-square) ![JS ES6](https://img.shields.io/badge/Interaction-JavaScript_ES6-F7DF1E?style=flat-square) |

### System Architecture

```mermaid
graph TD
    Client[Frontend: JS & HTML] <--> API[REST API Enpoints]
    API <--> Logic[PHP Controller & MVC]
    Logic <--> DB[(MySQL Database)]
```

---

## Chronological Journey

- **Day 21-22**: Structural design: Database schemas and UML modeling.
- **Day 23-24**: Core backend: User authentication and session management.
- **Day 25-26**: Business logic: Job creation workflow and application system.
- **Day 27-28**: API Development: Building endpoints for remote data access.
- **Day 29-30**: Frontend integration: Orchestrating the UI with the backend API.

---

## Skills developed

- **Architectural Vision**: Thinking in terms of MVC and decoupled systems.
- **Data Integrity**: Designing robust relational schemas and complex SQL queries.
- **Secure Coding**: Implementing industrial-grade security protocols.
- **Project Orchestration**: Managing a 10-day intensive development lifecycle.