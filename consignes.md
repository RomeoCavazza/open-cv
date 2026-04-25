# 🤖 Consignes pour l'Agent Scraper

**Mission :**
Tu dois explorer chaque lien présent dans la liste ci-dessous. Pour chaque lien :
1. Accède à la page web et scrappe **l'intégralité** du contenu textuel de l'offre (description, prérequis, infos utiles, etc.). Ne perds strictement aucun morceau.
2. Restitue l'ensemble du texte proprement (en conservant la structure).
3. Sauvegarde chaque offre dans un fichier Markdown (`.md`) unique.
4. Place tous ces fichiers `.md` (un par offre) dans le dossier suivant : `/home/tco/Bureau/alternance/offres/offres/`. Utilise des noms de fichiers clairs, par exemple basés sur le nom du poste ou de l'entreprise.

## Relance du Scraper

### Source de configuration
- YAML principal : `/home/tco/Bureau/alternance/config/scrape-offres.yaml`
- Source brute des URLs : `/home/tco/Bureau/alternance/config/offres-urls.raw.json`
- Script Python : `/home/tco/Bureau/alternance/scripts/scrape_offres.py`
- Environnement Nix : `/home/tco/Bureau/alternance/shell.nix`

### Commandes

```bash
cd /home/tco/Bureau/alternance
nix-shell
python scripts/scrape_offres.py --config /home/tco/Bureau/alternance/config/scrape-offres.yaml --overwrite
```

### Convention actuelle
- Les offres générées vont dans : `/home/tco/Bureau/alternance/offres/offres/`
- Les snapshots HTML et le rapport JSON sont désactivés par défaut pour éviter le bloat
- Si besoin temporaire, on peut réactiver `html_dir` ou `report` dans le YAML

---

# 🚀 Mission : Personnalisation Massive de CV

**Objectif :** Générer des CV personnalisés au format Markdown dans `/home/tco/Bureau/alternance/cv/markdown/new/`, adaptés chacun à une offre spécifique parmi celles disponibles.

## 🧭 Protocole de Boot (Ordre d'allumage)
Avant toute rédaction, l'IA doit impérativement s'imprégner du contexte dans cet ordre :
1. Lire /home/tco/Bureau/alternance/offres/liste.md pour voir les cibles.
2. Lister /home/tco/Bureau/alternance/offres/offres/ pour vérifier les données RAW.
3. Lire /home/tco/Bureau/alternance/portfolio/profil.md (Source de vérité absolue).
4. Analyser /home/tco/Bureau/alternance/cv/markdown/modele/ et /old/ pour comprendre la structure.
5. Explorer les README de /home/tco/Bureau/alternance/portfolio/projets/ pour les détails techniques.

## 📂 Ressources & Sources d'Autorité
- **Source de Vérité (Moi) :** [/portfolio/profil.md](file:///home/tco/Bureau/alternance/portfolio/profil.md) (Identité, expériences, technos, projets).
- **Détail Projets :** [/portfolio/projets/](file:///home/tco/Bureau/alternance/portfolio/projets) (Précision technique des side projects).
- **Inspiration & Forme :** [/cv/markdown/old/](file:///home/tco/Bureau/alternance/cv/markdown/old/) (Structures thématiques par catégorie : IA, Dev, IoT...).
- **Conteneur cible :** [/cv/markdown/new/](file:///home/tco/Bureau/alternance/cv/markdown/new/) (Destination des fichiers générés).

## 🎯 Consignes de Rédaction

### 1. Structure Générale
- **Format :** Markdown pur compatible avec le template standard.
- **Nommage :** Enregistrer sous `cv-[nom-de-l-offre].md`.
- **Intégrité :** Ne jamais modifier les sections CONTACT et LANGUES.
- **Zéro Emoji :** Interdiction d'utiliser des emojis dans le CV final.

### 2. Adaptation du Profil
- **Titre de l'offre :** Doit être exact et concis.
- **Pitch / Description :** Reformuler l'accroche pour qu'elle résonne directement avec les besoins de l'entreprise.
- **Formations :** Sélectionner le Master Epitech le plus pertinent parmi les 6 choix.

### 3. Expériences & Projets
- **Pivot technologique :** Réordonner les side projects pour mettre en valeur ceux qui utilisent la stack de l'offre.
- **Précision :** Utiliser les metrics et détails techniques issus du profil.md et des README.
- **Honnêteté :** Interdiction de mentir ou d'inventer des faits.

### 4. Compétences (Standard Technique)
- **Structure :** Fixer à **5 thématiques** courtes et adaptées à l'offre (ex: IA, DevOps, Backend).
- **Wording :** Titre de catégorie de 1 ou 2 mots maximum. Éviter le symbole "&".
- **Formatage :** Lister uniquement les outils/technos. Interdiction d'utiliser des parenthèses.
- **Priorisation :** Classer les technos par pertinence décroissante par rapport à l'offre cible.

---

## 🛡️ Critères de Qualité
- **ATS Compatible :** Optimisation pour les outils de lecture automatique.
- **Esthétique :** Respecter les séparateurs "--- SECTION ---" et les sauts de ligne pour l'aération.
- **Style :** Wording professionnel et orienté impact.
