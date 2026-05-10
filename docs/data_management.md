# Gestion des Données et Seeding

Ce document décrit les outils et scripts disponibles pour gérer la base de données locale, réinitialiser l'application et charger des données de test.

## Outils de Base

Le projet utilise **PostgreSQL** (stocké localement dans `.pg/` via Nix).
Les commandes principales passent par le `Justfile`.

### 1. Réinitialisation complète (db-reset)
Vide toutes les tables applicatives (offres, instances, messages, annexes, profils).
```bash
just db-reset
```
*Note : Cette commande redémarre Postgres s'il est arrêté.*

### 2. Création d'un état vierge (seed-blank)
Crée un profil actif minimal ("Nouveau Candidat") sans aucune expérience ni projet. Utile pour repartir de zéro.
```bash
cargo run -p api --bin seed_blank
```

## Chargement du Profil Réel

Pour importer vos données personnelles (Expériences, Formations, Projets) depuis les fichiers Markdown et JSON locaux.

### 1. Fichier Source
Le profil est lu depuis : `data/user/profil_to_upload.md`.
Ce fichier suit un format Markdown spécifique parsé par le script de seed.

### 2. Commande d'import
```bash
just seed-profile
# ou
cargo run -p api --bin seed_profile
```

## Chargement des Offres et Instances

Pour charger des offres d'emploi de test et des candidatures associées.

```bash
just seed-data
# ou
cargo run -p api --bin seed_offers_instances
```

## Workflow Recommandé pour Nettoyage

Pour vider l'app et remettre uniquement votre profil à jour :
```bash
just db-reset
cargo run -p api --bin seed_blank
just seed-profile
```
