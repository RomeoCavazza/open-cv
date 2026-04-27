# Days 58–59 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: (à compléter si nécessaire)

## Objectifs
- Mettre en place un serveur web complet avec Apache2, PHP et MariaDB.  
- Déployer une application full-stack (frontend et backend).  
- Sécuriser les services avec nftables et authentification Apache2.  
- Automatiser les sauvegardes de base de données.  
- Configurer HTTPS avec certificat auto-signé.  

## Actions

### Day 58 - Configuration serveur et déploiement
- **ex00 - Server machine** : Création d'une nouvelle VM Debian 13 sans interface graphique (ne pas réutiliser la machine du jour précédent)
- **ex01 - Binaries required** : Installation des binaires nécessaires (curl, symfony, composer, php >= 8.0, node, mariadb-server) avec vérification des versions
- **ex02 - Web server** : Installation et configuration d'Apache2 pour répondre sur le port 8080
- **ex03 - Database** : Installation de mariadb-server avec mot de passe root `Nn3Zalig64`, configuration pour accessibilité depuis l'extérieur de la VM
- **ex04 - Configure database** : Création de l'utilisateur `data-backend` avec tous les droits sur toutes les bases de données, mot de passe `EwUrk9046`
- **ex05 - Phpmyadmin** : Installation de phpMyAdmin, accessible sur `http://[yourLocalIP]:8080/phpmyadmin`
- **ex06 - Frontend** : Déploiement du frontend fourni dans `/var/www/frontend`, accessible sur `http://[yourIP]:8080/`

### Day 59 - Backend, sécurité et automatisation
- **ex01 - Backend** : Déploiement du backend fourni dans `/var/www/backend`, configuration Apache2 sur port 8080, installation des modules PHP requis, API disponible sur `http://localhost:8080/api` (virtualhost, déploiement Symfony)
- **ex02 - Admin Security** : Sécurisation de `scriptadmin.php` avec les fonctionnalités Apache2 (basic auth), accessible sur `http://localhost:8080/admin`, interdiction de modifier le code de l'application
- **ex03 - Security** : Configuration nftables dans `/etc/nftables.conf` (autoriser SSH in/out, HTTP out, autres autorisations nécessaires pour frontend/backend, bloquer les autres ports, persistance au reboot), possibilité de mettre à jour les packages avec apt
- **ex04 - Local backup** : Création du script `/backup/backup.sh` pour sauvegarder la base de données et la compresser dans `/backup/`
- **ex05 - Recurring scratches** : Configuration d'un cron pour exécuter automatiquement le script de backup toutes les heures en tant qu'utilisateur root
- **ex06 - HTTPS admin** : Activation du protocole HTTPS pour le panel admin avec certificat auto-signé (voir Task 02)
- **ex07 - END** : Vérification finale, message de succès affiché sur l'application frontend JS si tout fonctionne correctement

## Leçons
- **Serveur web** : Configuration Apache2, virtualhosts, ports personnalisés, déploiement d'applications
- **Base de données** : Installation MariaDB, création d'utilisateurs, gestion des droits, accessibilité réseau
- **Déploiement** : Frontend et backend, gestion des dépendances (composer, npm), modules PHP
- **Sécurité Apache2** : Authentification basique, protection des sections admin, configuration .htaccess
- **Pare-feu** : nftables avancé, règles pour services web, persistance, gestion des flux
- **Automatisation** : Scripts bash, cron, sauvegarde automatique de bases de données
- **HTTPS** : Certificats auto-signés, configuration SSL/TLS pour Apache2
- **DevOps** : Déploiement complet d'une application, monitoring, maintenance
