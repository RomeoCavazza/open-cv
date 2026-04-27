# Day 60 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: (à compléter si nécessaire)

## Objectifs
- Découvrir l'automatisation avec Ansible.  
- Créer des rôles Ansible pour automatiser la configuration de serveurs.  
- Mettre en place une stack web complète (Nginx, PHP, MariaDB) via Ansible.  
- Comprendre l'Infrastructure as Code (IaC).  

## Actions
- **ex01 - Mandatory** : Toutes les tâches doivent être exécutées avec Ansible, création de l'inventory `host.ini` avec `[nsapoold05] 127.0.0.1 ansible_connection=local`, exécution avec `ansible-playbook -i host.ini playbook.yml` (autograder sur VM Server, peut prendre du temps)
- **ex02 - Server** : Installation d'une nouvelle VM Debian 13 (système de base uniquement), configuration du compte root avec mot de passe `bv3xv`, carte réseau en mode bridge ou NAT
- **ex03 - Hostname** : Création d'un rôle Ansible pour changer le nom du serveur en `nsapoold05`
- **ex04 - User** : Création d'un rôle Ansible pour créer l'utilisateur `narvin` avec mot de passe `NsaPool42` et répertoire home `/home/narvin`
- **ex05 - Custom message of the day** : Création d'un rôle Ansible pour changer le message de bienvenue en `Welcome to nsapoold85` lors de la connexion
- **ex06 - Base** : Création d'un rôle Ansible pour mettre à jour le dépôt distant et installer les packages : git, htop, curl, sudo, unzip, python3, python3-pip
- **ex07 - Nginx** : Création d'un rôle Ansible pour installer la dernière version de Nginx, copier le site web fourni dans le répertoire racine de Nginx, configurer le site pour écouter sur le port 80
- **ex08 - PHP** : Création d'un rôle Ansible pour installer PHP version 7.4 ou antérieure (`php <= 7.4`) avec les modules : php-cli, php-fpm, php-json, php-pdo, php-mysql, php-zip, php-gd, php-mbstring, php-curl, php-xml, php-pear, php-bcmath, php-intl, configuration du timezone dans `php.ini` à `Europe/Paris`
- **ex09 - MySQL** : Création d'un rôle Ansible pour installer la dernière version de MariaDB, copier la base de données fournie, créer l'utilisateur `nsad04` avec tous les droits sur toutes les bases de données et mot de passe `E24h7U5kA9HJhq5VM98pn7p5znJpf8AK`
- **ex10 - End** : Vérification finale, message de succès concernant l'utilisateur narvin visible sur le site web accessible à `http://[IP_SERVER]/data.php`

## Leçons
- **Ansible** : Infrastructure as Code, playbooks, rôles, inventory, modules Ansible
- **Automatisation** : Réduction de la répétitivité, configuration idempotente, gestion de l'état désiré
- **Rôles Ansible** : Structure modulaire, réutilisabilité, séparation des responsabilités
- **Nginx** : Serveur web alternatif à Apache2, configuration de sites, ports
- **PHP** : Installation de versions spécifiques, modules PHP, configuration php.ini
- **MariaDB** : Installation automatisée, import de bases de données, gestion d'utilisateurs
- **Snapshots** : Création de snapshots pour démonstrations propres, restauration rapide
- **DevOps** : Automatisation complète du déploiement, Infrastructure as Code, pratiques modernes
