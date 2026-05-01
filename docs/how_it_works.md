# Comment ça marche ? (Architecture sous le capot)

Le système est conçu comme une application locale combinant Rust, PostgreSQL et un frontend statique pour générer des candidatures assistées par IA.

L'objectif est simple : transformer des offres d'emploi brutes en une candidature exploitable avec restitution, CV adapté et lettre de motivation.

## 1. L'Architecture Hexagonale (Backend Rust)

Le cœur du système est écrit en Rust, divisé en 5 "crates" (paquets) selon les principes de l'**Architecture Hexagonale (Ports & Adapters)** :

1. **`domain`** : Les modèles de données purs (`Offre`, `Instance`, `Resume`, `CoverLetter`). Zéro dépendance externe.
2. **`ports`** : Les interfaces (traits) que l'application utilise pour interagir avec l'extérieur (ex: `LlmClient`, `InstanceRepository`).
3. **`application`** : La logique métier (les "Use Cases"). C'est ici qu'on orchestre l'IA : "Prendre cette offre, chercher le profil de l'utilisateur, demander à Claude de rédiger un CV, sauvegarder".
4. **`adapters`** : Les implémentations concrètes des ports (ex: `llm_claude` pour interroger l'API Anthropic, `postgres` pour parler à la base de données via SQLx).
5. **`api`** : L'interface HTTP. Un serveur [Axum](https://github.com/tokio-rs/axum) qui expose les routes REST et sert le Frontend statique.

## 2. Le Pipeline IA (Génération Structurée)

La vraie force de ce backend, c'est sa gestion de l'IA. Au lieu de demander au LLM de générer du texte libre, on utilise la **génération structurée JSON**.

- L'application construit une requête en fournissant le **JSON Schema** exact du composant (CV ou Lettre) attendu.
- Le LLM (`Claude 3.5 Sonnet` par défaut) est contraint de répondre en respectant strictement cette structure (`serde_json::Value`).
- Le système désérialise statiquement ce JSON en Rust pour s'assurer de son intégrité avant de le stocker en BDD ou de le fournir au frontend.

Le pipeline passe par plusieurs étapes :
1. **Analyse de l'offre** et récupération du profil actif.
2. **Recherche / reranking** des chunks utiles du profil.
3. **Plan de candidature**.
4. **Génération des livrables** : restitution, CV et lettre.
5. **Validation minimale puis persistance** en base.

## 3. Le Frontend

Le frontend vit dans le dossier `/web` et est délibérément **Vanilla (HTML/JS/CSS purs)** pour l'instant.

- Il est composé d'un dashboard principal et de documents intégrés dans des **iframes**.
- **Pourquoi des Iframes ?** Pour garantir une isolation CSS parfaite. Le rendu du CV a besoin d'unités absolues (`mm`) pour générer un PDF exact au format A4 (210x297mm). Une Iframe évite que le CSS de l'interface graphique (barre latérale) n'interfère avec l'impression du document.
- L'utilisateur clique sur une offre, l'interface change le `src` de l'Iframe, et la vue se met à jour en injectant le JSON (généré par le backend Rust) dans les champs HTML.

## 4. Les Données (PostgreSQL Local)

Toutes les offres et les historiques de génération sont stockés dans une base **PostgreSQL 16**.
Grâce à `flake.nix` et au `Justfile`, la base peut tourner entièrement dans un dossier local (`.pg/`), sans dépendre d'un service global installé sur la machine.

Des extensions comme `pgvector` préparent la recherche sémantique sur les chunks de profil.
