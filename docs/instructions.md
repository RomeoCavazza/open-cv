# Consignes d'Utilisation

Ce document définit les protocoles et les règles de gestion des données pour l'automatisation des CV.

## Protocole de Prise de Contexte

L'agent doit consulter les fichiers dans l'ordre suivant :

1. **`README.md`** : Vue d'ensemble de l'architecture.
2. **`/data/user/profile.md`** : Données personnelles de référence.
3. **`/docs/how_it_works.md`** : Détails techniques sur le fonctionnement des scripts.
4. **`/data/offres/liste.json`** : Liste des cibles de candidature.
5. **`/data/instances/`** : Dossiers de travail pour chaque candidature.

---

## Mission 1 : Collecte d'Offres

1. Extraire le contenu textuel des offres (descriptions, prérequis).
2. Sauvegarder chaque offre au format Markdown dans `/data/offres/raw/`.

---

## Mission 2 : Personnalisation

### 1. Structure
- **Format** : Mise à jour des fichiers `resume.json` et `cover-letter.json` dans chaque instance.
- **Règles strictes** : Ne pas modifier les sections CONTACT et LANGUES sans instruction spécifique.
- **Style** : Professionnel, sans emojis.

### 2. Adaptation du Profil
- **Titre** : Reprendre l'intitulé de l'offre.
- **Accroche** : Adapter le paragraphe d'introduction en fonction de l'entreprise.
- **Formation** : Adapter l'intitulé du Master Epitech selon le domaine.

### 3. Expériences & Projets
- **Pertinence** : Prioriser les projets utilisant la stack technologique demandée.
- **Précision** : Utiliser les détails techniques issus du profil et des README de projets.
- **Données** : Ne pas inventer d'expériences.

### 4. Compétences
- **Structure** : Organiser en 5 catégories thématiques.
- **Formatage** : Lister les technologies sans parenthèses.

---

## Critères de Validation

- **Cohérence** : Vérifier que les informations sont synchronisées entre le CV et la lettre de motivation.
- **Aération** : Veiller à la lisibilité des documents exportés.
- **Sources** : Toujours utiliser `/data/user/profile.md` comme base de données principale.
