<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-DevOps_&_SysAdmin-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Days-56--60-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Focus-Infrastructure_as_Code-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Seminar: Systems, Networking & DevOps

Moving from software to infrastructure: mastering the art of automated deployment, secure networking, and planetary-scale system administration.

---

> [!IMPORTANT]
> **Core Objectives**: 
> - **IaC Excellence**: Automating server configuration with **Ansible**.
> - **Network Engineering**: Mastering DHCP, DNS, and secure routing.
> - **Security Baseline**: Implementing fail2ban, nftables, and SSH hardening.
> - **Virtualization**: Deploying and managing complex VM architectures with **VirtualBox**.

## Technical Core

| Layer | Implementation |
|---|---|
| **OS** | ![Debian](https://img.shields.io/badge/OS-Debian-A81D33?style=flat-square&logo=debian&logoColor=white) |
| **Automation** | ![Ansible](https://img.shields.io/badge/IaC-Ansible-EE0000?style=flat-square&logo=ansible&logoColor=white) |
| **Networking** | ![BIND9](https://img.shields.io/badge/DNS-BIND9-4285F4?style=flat-square) ![KEA](https://img.shields.io/badge/DHCP-KEA-34A853?style=flat-square) |
| **Web** | ![Nginx](https://img.shields.io/badge/Server-Nginx-009639?style=flat-square&logo=nginx&logoColor=white) ![Apache2](https://img.shields.io/badge/Server-Apache2-D22128?style=flat-square&logo=apache&logoColor=white) |

### Infrastructure Workflow

```mermaid
graph LR
    Dev[Local Machine] --> Ansible[Ansible Playbook]
    Ansible --> VM1[VM: Web Server]
    Ansible --> VM2[VM: Database]
    VM1 <--> VM2
    Internet((Internet)) <--> VM1
```

---

## Chronological Journey

- **Day 56**: Virtualization & Debian basics: setting up the baseline environment and SSH security.
- **Day 57**: Advanced Networking: DHCP (Kea), DNS (Bind9), and internal routing with nftables.
- **Day 58-59**: Web Stack & DB: Apache2, Nginx, MariaDB, and secure application deployment.
- **Day 60**: Final Automation: Complete site orchestration with complex **Ansible** roles.

---

## Skills developed

- **Infrastructural Vision**: Understanding how bits flow across hardware and software layers.
- **Automation First**: Eliminating manual tasks through robust IaC playbooks.
- **Defensive Engineering**: Building secure-by-design systems against external threats.
- **System Stability**: Implementing logging, monitoring, and reliable service orchestration.
