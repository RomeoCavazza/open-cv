<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-Jenkins_&_Automation-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-66--70-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Focus-JCasC_&_DSL-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Continuous Integration & Jenkins (MY_MARVIN)

Moving from manual builds to fully automated enterprise pipelines: mastering **Jenkins as Code**, Groovy DSL, and secure CI/CD orchestration.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **Configuration as Code**: Reproducible Jenkins setup via **JCasC** (YAML).
> - **Programmatic Jobs**: Generating industrial-grade jobs with **Job DSL** (Groovy).
> - **Automated Pipeline**: Implementing `compile → test → clean` lifecycles.
> - **RBAC Security**: Managing fine-grained access with Role-Based Strategy.

## Technical Core

| Layer | Implementation |
|---|---|
| **Server** | ![Jenkins](https://img.shields.io/badge/Engine-Jenkins-D24939?style=flat-square&logo=jenkins&logoColor=white) |
| **Config** | ![YAML](https://img.shields.io/badge/Format-YAML-000000?style=flat-square&logo=yaml&logoColor=white) ![JCasC](https://img.shields.io/badge/Method-JCasC-blue?style=flat-square) |
| **Logic** | ![Groovy](https://img.shields.io/badge/Logic-Groovy_DSL-4298B8?style=flat-square&logo=apache-groovy&logoColor=white) |
| **SCM** | ![Git](https://img.shields.io/badge/VCS-Git_/_GitHub-F05032?style=flat-square&logo=git&logoColor=white) |

### Industrial CI Pipeline

```mermaid
graph LR
    Push[Code Push] --> Trigger[SCM Polling]
    Trigger --> Compile[Stage: Compile]
    Compile --> Test[Stage: Test Run]
    Test --> Quality[Stage: JaCoCo / Results]
    Quality --> Clean[Stage: Workspace Cleanup]
```

---

## Chronological Journey

- **Day 66-67**: JCasC Discovery: Defining a complete Jenkins instance in a single `my_marvin.yml`.
- **Day 68-69**: Job DSL Mastery: Factoring repetitive tasks into a programmatic Groovy script.
- **Day 70**: Full Integration: Deploying an automated pipeline for the **MY_MARVIN** project.

---

## Skills developed

- **Reproducibility Excellence**: Eliminating "click-ops" through declarative configuration.
- **Scale Engineering**: Automating the creation of hundreds of jobs with code.
- **Defensive CI**: Enforcing security protocols and RBAC at the infrastructure level.
- **Workflow Orchestration**: Mastering the handover between SCM, Build, and QA stages.
