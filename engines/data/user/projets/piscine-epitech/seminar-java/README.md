<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Advanced_Java-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-31--40-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Focus-OOP_&_Architecture-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Advanced Object-Oriented Programming (Java)

Diving deep into the world of industrial-grade software engineering through Java: mastering abstraction, design patterns, and the Java Virtual Machine.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **OOP Mastery**: Deep dive into Encapsulation, Inheritance, and Polymorphism.
> - **Advanced Types**: Leveraging Generics, Interfaces, and Reflection for dynamic code.
> - **Architecture**: Implementing standard **Design Patterns** (Observer, Factory, Singleton).
> - **Build System**: Orchestrating complex dependencies with **Maven**.

## Technical Core

| Layer | Implementation |
|---|---|
| **Language** | ![Java](https://img.shields.io/badge/Language-Java-007396?style=flat-square&logo=java&logoColor=white) |
| **Build** | ![Maven](https://img.shields.io/badge/Build-Maven-C71A36?style=flat-square&logo=apache-maven&logoColor=white) |
| **Testing** | ![JUnit 5](https://img.shields.io/badge/Tests-JUnit_5-25A162?style=flat-square&logo=junit5&logoColor=white) |
| **IDE** | ![IntelliJ](https://img.shields.io/badge/IDE-IntelliJ_IDEA-000000?style=flat-square&logo=intellij-idea&logoColor=white) |

### Class Hierarchy Model

```mermaid
classDiagram
    class Character {
        <<abstract>>
        +String name
        +int hp
        +attack()
    }
    class Warrior {
        +charge()
    }
    class Mage {
        +fireball()
    }
    Character <|-- Warrior
    Character <|-- Mage
```

---

## Chronological Journey

- **Day 31**: Foundation: Basics, types, and the Java ecosystem.
- **Day 32**: Advanced classes: Inheritance, polymorphism, and the Gecko model.
- **Day 33**: Organization: Packages and modular architecture.
- **Day 34**: Abstraction: Abstract classes, Enums, and the Animal/Cat hierarchy.
- **Day 35**: **Project: Space Arena** - A complete Java application.
- **Day 36**: Interfaces: Contracts, Movable entities, and robust Exception handling.
- **Day 37**: Generics: Type safety for Solo, Pair, and Battalion collections.
- **Day 38**: Design Patterns: Factory, Composite, Observer, and Decorator.
- **Day 39**: Reflection: Inspector tool, custom Annotations, and dynamic analysis.
- **Day 40**: **Final Project: Boulangerie** - Complex system with generics and patterns.

---

## Skills developed

- **Architectural Thinking**: Designing self-documenting and maintainable object trees.
- **Strong Typing**: Leveraging the compiler to catch errors before execution.
- **Generic Engineering**: Building high-reusable library-style components.
- **Reflective Power**: Understanding the JVM's dynamic capabilities.