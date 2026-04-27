# Day 56 — Objectifs et Enseignements

## Objectifs

- Installer et configurer une machine virtuelle Debian sécurisée.  
- Maîtriser la gestion des utilisateurs et groupes Linux.  
- Configurer SSH avec authentification par clés et restrictions.  
- Mettre en place fail2ban pour protéger les services.  
- Configurer nftables comme pare-feu.

## Enseignements

- **Installation Debian** : Partitions séparées pour sécurité et organisation (/home, /var isolés)
- **Gestion utilisateurs/groups** : UID/GID personnalisés, appartenance aux groupes, permissions
- **SSH sécurisé** : Authentification par clés, changement de port, désactivation root, restrictions par utilisateur
- **fail2ban** : Protection contre les attaques par force brute, intégration avec nftables
- **nftables** : Pare-feu moderne, règles persistantes, gestion des flux entrant/sortant
- **Sécurité système** : Principes de moindre privilège, défense en profondeur, isolation des services
