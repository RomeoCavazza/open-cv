# Instructions d'Utilisation

Ce guide explique comment démarrer et utiliser le Builder de Candidatures IA-Native en local.

## 1. Prérequis

Assure-toi d'avoir **Nix** installé sur ton système. Le projet utilise les Nix Flakes pour garantir un environnement de développement 100% reproductible (Rust, Cargo, Just, PostgreSQL, etc.).

## 2. Entrer dans l'Environnement

Dans ton terminal, à la racine du projet, lance :
```bash
nix develop
```
*Ceci va télécharger et configurer les dépendances nécessaires. Une fois terminé, tu verras le message `alternance dev shell ready`.*

## 3. Initialisation de la Base de Données

Si c'est la toute première fois que tu lances le projet, tu dois initialiser le dossier de la base de données PostgreSQL locale :
```bash
just db-init
```
*(Cela va créer un dossier caché `.pg/` à la racine pour stocker les données).*

## 4. Lancement au Quotidien

Une fois dans le `nix develop`, la routine est simple :

1. **Démarrer le serveur PostgreSQL :**
   ```bash
   just db-up
   ```
2. **Appliquer les migrations** (création des tables, si elles ont changé) :
   ```bash
   just migrate
   ```
3. **Lancer le serveur API (backend Rust) :**
   ```bash
   just dev
   ```
   *La commande `just dev` utilise `cargo watch`, donc le serveur se recharge automatiquement si tu modifies le code Rust.*

Le serveur démarrera sur **http://localhost:8000**.

## 5. Accéder à l'Application

Ouvre ton navigateur et rends-toi sur : **[http://localhost:8000](http://localhost:8000)**.
L'API sert directement le dossier `/web` statique.

## 6. Variables d'Environnement

Le projet utilise un fichier `.env` pour stocker les clés nécessaires à la génération.
1. Copie le fichier d'exemple : `cp .env.example .env`
2. Édite `.env` pour y ajouter tes clés :
   ```env
   ANTHROPIC_API_KEY=sk-ant-api03-...
   ```

## Commandes Utiles (Récapitulatif)

- `just db-down` : Arrête proprement le serveur PostgreSQL en arrière-plan.
- `rm -rf target` : Supprime les artefacts de compilation si tu veux récupérer de l'espace disque.
- `rm -rf .pg` : Supprime la base locale uniquement après avoir arrêté Postgres avec `just db-down`.
- `cargo check --workspace` : Vérifie que le code Rust compile sans erreur.
