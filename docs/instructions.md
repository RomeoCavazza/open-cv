# Consignes pour l'IA (Agentic Coding)

Ce document contient les règles d'engagement, le protocole de boot et les consignes de rédaction strictes pour toute IA intervenant sur ce dépôt.

## Protocole de Boot (Ordre d'allumage recommandé)

Avant toute action ou rédaction, l'agent doit impérativement s'imprégner du contexte dans cet ordre :

1. **`README.md`** : Aperçu rapide de l'architecture.
2. **`/portfolio/profil.md`** : **Source de vérité absolue** (Identité, expériences, technos, projets).
3. **`/docs/instructions.md`** : Présentes consignes et règles d'engagement.
4. **`/docs/how_it_works.md`** : Compréhension technique des outils et de la structure du repo.
5. **`/offres/liste.md`** : Pour identifier les cibles de candidature.
6. **`/offres/offres/`** : Pour analyser les données RAW des offres scrapées.
7. **`/cv/web/data.json`** : Référentiel actuel pour le design High-Fidelity.
8. **`/portfolio/projets/`** : Pour extraire les détails techniques spécifiques des side projects via leurs README respectifs.

---

## Mission 1 : Scraping d'Offres

Si l'agent est chargé de scrapper de nouvelles offres :
1. Explorer chaque lien de la liste fournie.
2. Extraire l'intégralité du contenu textuel (description, prérequis, infos utiles).
3. Restituer le texte proprement en conservant la structure.
4. Sauvegarder chaque offre dans un fichier Markdown unique dans `/offres/offres/`.
5. Utiliser des noms de fichiers clairs basés sur le poste ou l'entreprise.

---

## Mission 2 : Personnalisation de CV

L'objectif est de générer des CV personnalisés adaptés chacun à une offre spécifique.

### 1. Structure Générale
- **Format** : Markdown pur pour l'ATS ou mise à jour du `data.json` pour le Web.
- **Nommage Markdown** : Enregistrer sous `cv/markdown/new/cv-[nom-de-l-offre].md`.
- **Intégrité** : Ne jamais modifier les sections CONTACT et LANGUES sans ordre explicite.
- **Zéro Emoji** : Interdiction formelle d'utiliser des emojis dans les CV finaux.

### 2. Adaptation du Profil
- **Titre** : Doit être l'intitulé exact de l'offre (concis).
- **Pitch** : Reformuler l'accroche pour qu'elle résonne directement avec les besoins de l'entreprise.
- **Formations** : Sélectionner le parcours spécifique le plus pertinent.

### 3. Expériences & Projets
- **Pivot Technologique** : Réordonner les projets pour mettre en valeur ceux utilisant la stack de l'offre.
- **Précision** : Utiliser les metrics et détails techniques issus du `profil.md` et des README de projets.
- **Honnêteté** : Interdiction stricte de mentir ou d'inventer des faits.

### 4. Compétences (Standard Technique)
- **Structure** : Fixer à **5 thématiques** courtes et adaptées (ex: IA, DevOps, Backend).
- **Wording** : Titre de catégorie de 1 ou 2 mots maximum. Éviter le symbole "&".
- **Formatage** : Lister uniquement les outils/technos. Interdiction d'utiliser des parenthèses.
- **Priorisation** : Classer les technos par pertinence décroissante par rapport à l'offre.

---

## Critères de Qualité

- **ATS Compatible** : Optimisation pour les outils de lecture automatique.
- **Esthétique** : Respecter les séparateurs "--- SECTION ---" et les sauts de ligne pour l'aération.
- **Style** : Wording professionnel, orienté impact et résultats (verbes d'action).
- **Bootstrapping** : Toujours vérifier les sources d'autorité (profil.md) avant de générer du contenu.
