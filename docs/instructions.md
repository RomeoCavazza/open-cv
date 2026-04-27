# Consignes d'Utilisation (Industrialisation Haute-Fidélité)

Ce document définit les protocoles et les règles de gestion des données pour l'automatisation des CV avec un standard de qualité "Zéro Bloat".

## Protocole de Prise de Contexte
L'agent doit consulter les fichiers dans l'ordre suivant :
1. **`README.md`** : Vue d'ensemble de l'architecture.
2. **`/engines/data/user/profile.md`** : Données personnelles de référence.
3. **`/docs/how_it_works.md`** : Détails techniques sur le fonctionnement des scripts.
4. **`/engines/data/offres/liste.json`** : Liste des cibles de candidature.
5. **`/engines/data/offres/json/`** : Dossiers de travail pour chaque candidature.

---

## Mission 1 : Collecte d'Offres
1. Extraire le contenu textuel des offres (descriptions, prérequis).
2. Sauvegarder chaque offre au format Markdown dans `/engines/data/offres/markdown/`.

---

## Mission 2 : Personnalisation (Standard "Limpide")

### 1. Structure du CV (Strict)
- **Hiérarchie** : 2 Expériences Pro (3 bullets) / 2 Projets Perso (2 bullets). Point barre.
- **Format** : Mise à jour des fichiers `resume.json` et `cover-letter.json` dans chaque instance.
- **Pitch** : 1 à 2 lignes. Posture : Proactif ("mettre mes compétences au service de...") et humble.
- **Brevité** : **Une seule ligne** par bullet point. Supprimer les adjectifs (robuste, excellent).

### 2. Adaptation & Wording
- **Style "Limpide"** : Valeur ajoutée d'abord, stack technique discrète entre parenthèses à la fin.
- **Terminologie** : Utiliser "Applications" (pas "Apps"), "Messagerie instantanée" (pas "Client de").
- **Dates** : Pour les projets actifs, utiliser systématiquement **"2025 – Présent"**.

### 3. Compétences (Anti-Bloat)
- **Catégories** : Titres de 1 ou 2 mots maximum (Backend, Web, Cloud, Data, IA).
- **Formatage** : Pas de "&", pas de versions (Next.js, pas Next.js 14), pas de parenthèses (FastAPI, pas FastAPI (Python)).
- **Focus** : Hard skills uniquement. Exclure Agile, Clean Code, SOLID (doivent transparaître dans les projets).

---

## Critères de Validation
- **Sobriété** : Vérifier l'absence de jargon "Startup" ou de termes marketing vides.
- **Densité** : Le document doit être aéré mais techniquement dense.
- **Sources** : Toujours utiliser `/engines/data/user/profile.md` comme source de vérité unique.
