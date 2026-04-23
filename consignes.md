# 🤖 Consignes pour l'Agent Scraper

**Mission :**
Tu dois explorer chaque lien présent dans la liste ci-dessous. Pour chaque lien :
1. Accède à la page web et scrappe **l'intégralité** du contenu textuel de l'offre (description, prérequis, infos utiles, etc.). Ne perds strictement aucun morceau.
2. Restitue l'ensemble du texte proprement (en conservant la structure).
3. Sauvegarde chaque offre dans un fichier Markdown (`.md`) unique.
4. Place tous ces fichiers `.md` (un par offre) dans le dossier suivant : `/home/tco/Bureau/alternance/offres/`. Utilise des noms de fichiers clairs, par exemple basés sur le nom du poste ou de l'entreprise.

---

# 🚀 Mission : Personnalisation Massive de CV

**Objectif :** Générer des CV personnalisés au format Markdown dans `/home/tco/Bureau/alternance/cv/new-cv`, adaptés chacun à une offre spécifique parmi celles disponibles.

---

## 📂 Ressources & Sources d'Autorité

- **Source de Vérité (Moi) :** [/portfolio/profil.md](file:///home/tco/Bureau/alternance/portfolio/profil.md) (Identité, expériences, technos, projets). **Note :** Ne pas inventer d'expériences.
- **Détail Projets :** [/portfolio/projets/](file:///home/tco/Bureau/alternance/portfolio/projets) (À consulter pour la précision technique des side projects).
- **Inspiration & Forme :** [/cv/templates/](file:///home/tco/Bureau/alternance/cv/templates) (Utiliser ces structures pour ne pas repartir de zéro).
- **Offres cibles :** [/offres/](file:///home/tco/Bureau/alternance/offres/) (Détails complets de chaque poste via scraping).

---

## 🎯 Consignes de Rédaction

### 1. Structure Générale
- **Forme :** Ne strictement RIEN changer à la mise en page (Markdown).
- **Fond :** Adaptation chirurgicale à chaque offre.
- **Identité & Langues :** Ne jamais modifier les sections **Contact** et **Langues**.

### 2. Adaptation du Profil
- **Titre de l'offre :** Doit être exact, concis, et adapté (peut être légèrement simplifié si l'intitulé d'origine est trop long).
- **Pitch / Description :** Reformuler l'accroche pour qu'elle résonne directement avec les besoins de l'entreprise.
- **Formations :** 
  - Sélectionner le **Master** le plus pertinent parmi les 6 proposés à l'Epitech.
  - Adapter la description qui suit le **Pré-MSc** pour mettre en avant les bases liées au poste.

### 3. Expériences & Projets
- **Expériences pro :** Garder le fond historique. Seul le wording peut être légèrement ajusté pour souligner une compétence clé demandée.
- **Side Projects :** Sélectionner et mettre en valeur les projets du portfolio les plus pertinents pour l'offre. 
  - Description concise et claire.
  - Nommage précis des technos employées.
  - **Interdiction de mentir.**

### 4. Compétences (Stratégie "Ultra-Intelligente")
- **Structure :** Fixer à **5 familles** de compétences (ni plus, ni moins).
- **Pertinence :** Sélectionner uniquement les technos/outils en lien avec l'offre ou mon profil réel.
- **Qualité :** Bannir les mots-clés "creux" ou vagues. Noms de technos uniquement.
- **Organisation :** L'ordre des familles et des outils doit refléter les priorités de l'offre.

---

## 🛡️ Critères de Qualité
- **Accessibilité :** Wording professionnel niveau RH.
- **Performance :** Optimisation pour les outils de lecture automatique (**ATS Compatible**).
- **Intégrité :** Alignement strict avec le contenu du `profil.md`.
