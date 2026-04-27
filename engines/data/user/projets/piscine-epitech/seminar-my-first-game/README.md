<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Game_Development-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-41--55-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Engine-libGDX-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Advanced Game Engineering (libGDX)

Mastering complex project architecture through the lens of 2D game development: implementing SOLID principles, design patterns, and high-performance rendering.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **Engine Mastery**: 2D rendering, input handling, and lifecycle management with **libGDX**.
> - **Architecture**: Strict adherence to **SOLID** principles in a highly dynamic environment.
> - **Build & Deploy**: Orchestrating cross-platform builds with **Gradle**.
> - **Quality Assurance**: 100% logic coverage with **JUnit** and **JaCoCo**.

## Technical Core

| Layer | Implementation |
|---|---|
| **Language** | ![Java](https://img.shields.io/badge/Language-Java-007396?style=flat-square&logo=java&logoColor=white) |
| **Engine** | ![libGDX](https://img.shields.io/badge/Engine-libGDX-e34f26?style=flat-square&logo=libgdx&logoColor=white) |
| **Build** | ![Gradle](https://img.shields.io/badge/Build-Gradle-02303a?style=flat-square&logo=gradle&logoColor=white) |
| **Coverage** | ![JaCoCo](https://img.shields.io/badge/Coverage-JaCoCo-f78516?style=flat-square) ![JUnit 5](https://img.shields.io/badge/Tests-JUnit_5-25A162?style=flat-square&logo=junit5&logoColor=white) |

### Game Architecture Logic

```mermaid
graph TD
    Input[Input Handler] --> Logic[Game Logic / Systems]
    Logic --> Rendering[libGDX Renderer]
    State[State Management] <--> Logic
    Asset[Asset Manager] --> Rendering
```

---

## Chronological Journey

- **Day 41-43**: libGDX discovery: rendering loops, textures, and sprites.
- **Day 44-46**: Input & Interaction: managing keyboard/mouse and physics basics.
- **Day 47-49**: Advanced Logic: Entity Component System (ECS) concepts and state machines.
- **Day 50-52**: Quality Sync: Implementing unit tests and code coverage reports.
- **Day 53-55**: Final Polish: Asset management, UI elements, and cross-platform export.

---

## Skills developed

- **Modular Design**: Breaking down complex systems into decoupled entities.
- **Performance Tuning**: Optimizing memory usage and render calls for 60FPS.
- **Quality Engineering**: Writing testable code in a non-deterministic environment.
- **Pattern Mastery**: Implementing Singleton, Factory, and State patterns in a real-world scenario.

