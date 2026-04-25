# Consignes pour l'IA (Agentic Coding)

Ce document contient les règles d'engagement et le protocole que tout LLM/Agent doit suivre en arrivant sur ce dépôt.

## Protocole de Boot (Ordre recommandé)

Pour garantir une compréhension parfaite du contexte, l'agent doit explorer le dépôt dans cet ordre :

1. **`README.md`** : Aperçu rapide de l'architecture.
2. **`/portfolio/profil.md`** : Source de vérité sur l'identité et les compétences de Roméo.
3. **`/docs/how_it_works.md`** : Compréhension technique des outils et de la structure.
4. **`/offres/liste.md` & `/offres/offres/`** : Analyse des cibles actuelles.
5. **`/cv-web/data.json`** : État actuel du CV "High-Fidelity".

---

## Mission Principale : Personnalisation de CV

L'objectif est de transformer les données brutes du profil en un CV parfaitement adapté à une offre spécifique.

### Principes de Rédaction
- **Précision** : Utiliser les détails techniques et les metrics issus du portfolio.
- **Honnêteté** : Ne jamais inventer d'expériences.
- **Sobriété** : Interdiction totale d'utiliser des emojis dans les documents finaux (CV).
- **Adaptation** : Le titre du CV et le pitch doivent résonner avec les mots-clés de l'offre.

### Guidelines Techniques
- **Format Markdown** : Pour les versions ATS, sauvegarder dans `cv/markdown/new/` sous le format `cv-[nom-offre].md`.
- **Format Web (High-Fidelity)** : Mettre à jour `/cv-web/data.json` pour refléter les changements.
- **Sections Contact/Langues** : Ne JAMAIS modifier ces sections sans instruction explicite.
- **Compétences** : Limiter à 5 thématiques claires, sans parenthèses, classées par pertinence.

---

## Critères de Qualité

1. **Cohérence Visuelle** : Respecter la structure des templates existants.
2. **Impact** : Utiliser un vocabulaire orienté résultats (Action Verbs).
3. **Zéro Bloat** : Ne pas ajouter d'informations inutiles qui ne servent pas l'offre visée.
4. **Bootstrapping** : Toujours vérifier les README des projets dans `/portfolio/projets/` avant de détailler une expérience spécifique.
