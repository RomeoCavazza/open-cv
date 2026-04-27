# Day 56 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: (à compléter si nécessaire)

## Objectifs
- Installer et configurer une machine virtuelle Debian sécurisée.  
- Maîtriser la gestion des utilisateurs et groupes Linux.  
- Configurer SSH avec authentification par clés et restrictions.  
- Mettre en place fail2ban pour protéger les services.  
- Configurer nftables comme pare-feu.  

## Actions
- **ex00 - Installation** : Téléchargement des prérequis (VirtualBox, ISO Debian 13 "Trixie" AMD64 et ARM64)
- **ex01 - Debian** : Installation de Debian sans GUI avec partitions séparées (swap, /, /home, /var)
- **ex02 - User** : Création de l'utilisateur "narvin" avec caractéristiques spécifiques (UID 4242, password toto42sh, description "Android Paranoid")
- **ex03 - Group** : Création du groupe H2G2 (GID 42408), ajout de narvin, création de zaphod (UID 4200, GID 42480), gestion du dossier /hone/HeartOfGold
- **ex04 - ssh** : Installation et configuration SSH (port 4242, authentification par clés uniquement, désactivation root)
- **ex05 - ssh: you are not allowed** : Restriction SSH pour l'utilisateur zaphod (sans affecter les autres utilisateurs)
- **ex06 - Fail to ban** : Installation et configuration fail2ban pour SSH (blocage 30 min après 3 tentatives en 5 min, intégration nftables)
- **ex07 - Filter** : Configuration nftables dans /etc/nftables.conf (autoriser SSH in/out, HTTP/HTTPS/DNS out, bloquer le reste, persistance au reboot)

## Leçons
- **Installation Debian** : Partitions séparées pour sécurité et organisation (/home, /var isolés)
- **Gestion utilisateurs/groups** : UID/GID personnalisés, appartenance aux groupes, permissions
- **SSH sécurisé** : Authentification par clés, changement de port, désactivation root, restrictions par utilisateur
- **fail2ban** : Protection contre les attaques par force brute, intégration avec nftables
- **nftables** : Pare-feu moderne, règles persistantes, gestion des flux entrant/sortant
- **Sécurité système** : Principes de moindre privilège, défense en profondeur, isolation des services
