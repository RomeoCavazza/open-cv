# Blueprint : Vers une Architecture SaaS "Personal Career Engineer"

Ce document esquisse la vision stratégique pour faire évoluer ce dépôt local vers une plateforme SaaS industrielle, destinée aux profils techniques exigeants.

## 1. Vision Produit
Passer d'un générateur de templates à un **architecte de carrière agentique**. L'objectif est d'utiliser l'IA non pas pour remplir des trous, mais pour raisonner sur la pertinence technique d'un profil par rapport à un écosystème d'ingénierie complexe.

## 2. Intelligence Layer (PyTorch & LoRA)
L'utilisation de modèles généralistes (GPT-4/Claude) introduit souvent du "bloat" rédactionnel. 
- **Fine-tuning LoRA** : Entraînement de modèles légers (Llama-3 8B, Mistral v0.3) sur un corpus de CV haute-fidélité et de réalisations techniques réelles.
- **Style Injection** : Garantir que le "tone of voice" reste technique, factuel et concis (Standard Limpide).
- **R&D via Jupyter** : Utilisation de notebooks pour le prototypage des fonctions de perte customisées et l'évaluation de la perplexité sur le jargon technique (ex: distinction fine entre "Data Science" et "Data Engineering").

## 3. Data & Storage (Qdrant)
Remplacer les fichiers JSON statiques par une architecture **RAG (Retrieval Augmented Generation)**.
- **Vector Database** : Stockage des expériences et projets sous forme de vecteurs d'embeddings dans Qdrant.
- **Semantic Matching** : Pour chaque offre, le système ne se contente pas de mots-clés, mais identifie les briques de ton historique les plus proches sémantiquement des enjeux du poste.
- **Hybrid Search** : Combiner la recherche vectorielle (sens) avec la recherche full-text (mots-clés techniques précis).

## 4. Orchestration Agentique (LangChain / LangGraph)
Passer d'un script linéaire (`personalize_all.py`) à un graphe d'agents cycliques.
- **Agent Analyste** : Décompose l'offre d'emploi en "besoins critiques".
- **Agent Architecte** : Sélectionne les 2 expériences et 2 projets les plus impactants dans Qdrant.
- **Agent Rédacteur** : Génère le contenu en respectant les contraintes de tokens et de layout.
- **Agent Critique (Loop)** : Vérifie le résultat final. Si le wording est jugé "bloat" ou trop long, il renvoie des consignes de correction à l'Agent Rédacteur.

## 5. Interface & SaaS (Layered integration)
- **Backend** : FastAPI pour servir les inférences et gérer l'orchestration.
- **Frontend** : Dashboard interactif avec une barre latérale (Sidebar) permettant un "pair-programming" de sa candidature en temps réel.
- **Multi-tenancy** : Isolation stricte des données (Profil, Projets) pour chaque utilisateur.

---

> **Note stratégique** : L'avantage concurrentiel de cette architecture réside dans la qualité du rendu final (moteur HTML/CSS existant) couplée à une intelligence spécialisée (LoRA) qui comprend réellement les enjeux bas-niveau et haute-performance.
