# Job Board

Plateforme de recrutement moderne avec API REST et interface responsive.

## Aperçu du projet

**Job Board** est une application web complète permettant aux entreprises de publier des offres d'emploi et aux candidats de postuler. Le projet utilise une architecture moderne avec API REST PHP et interface JavaScript vanilla.

## Présentation du projet

[Lien vers la présentation](https://www.canva.com/design/DAG2OhYVWMI/ZWmqM93mIn-hH2ct_ZKudQ/edit?utm_content=DAG2OhYVWMI&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton)

## Technologies utilisées

- **Backend** : PHP 7.4+, MySQL/MariaDB, MySQLi
- **Frontend** : HTML5, CSS3, JavaScript ES6+
- **Outils** : Git, GitHub, DBeaver
- **Déploiement** : Railway (production)


## Structure

```
T-WEB-501-PAR_16/
├── back/api/           # API REST PHP
│   ├── auth.php        # Authentification (login/logout/profile)
│   ├── users.php       # Inscription utilisateurs
│   ├── jobs.php        # CRUD annonces
│   ├── recruteur.php   # Gestion recruteurs/entreprises
│   ├── applications.php # Candidatures
│   ├── admin.php       # Administration
│   └── config.php      # Configuration DB
├── front/web/          # Interface utilisateur
│   ├── index.html      # Page principale
│   ├── login.html      # Connexion
│   ├── signup.html     # Inscription
│   ├── recruteur.html  # Dashboard recruteur
│   ├── admin.html      # Panel admin
│   └── my-profile.html # Profil utilisateur
└── back/data/          # Schéma base de données
    └── database.sql
```

## API Endpoints

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `/auth.php` | POST/GET/PUT/DELETE | Authentification et profil |
| `/users.php` | POST | Inscription utilisateur |
| `/jobs.php` | GET/POST/PUT/DELETE | Gestion des offres d'emploi |
| `/recruteur.php` | GET/POST | Gestion entreprises |
| `/applications.php` | POST | Candidatures |
| `/admin.php` | GET/POST/PUT/DELETE | Administration |

## Quick start

### Prérequis
- PHP 7.4 ou plus récent
- Un serveur web ou utiliser le serveur intégré de PHP

### Installation
```bash
# Cloner le repository
git clone https://github.com/EpitechMscProPromo2028/T-WEB-501-PAR_16.git
cd T-WEB-501-PAR_16
```

### Configuration
Le projet utilise Railway comme base de données en production. La configuration est déjà définie dans `back/.env`.

Pour utiliser une base locale, modifiez `back/.env` :
```env
DB_HOST=localhost
DB_PORT=3306
DB_NAME=jobboard
DB_USER=root
DB_PASS=votre_mot_de_passe
```

### Lancement
```bash
# Démarrer le serveur de développement
php -S localhost:8000

# Accéder à l'application
# Interface : http://localhost:8000/front/web/
# API : http://localhost:8000/back/api/
```
---

Projet réalisé dans le cadre du module T-WEB-501 - EPITECH MSc Pro 2025
