# Fonctionnement du Dépôt

Ce document détaille l'architecture technique et le fonctionnement opérationnel de l'écosystème de candidature.

## Architecture du Dépôt

### 1. Autodocumentation & Sources
- `/data/user/profile.md` : **Source de vérité absolue**. Contient l'identité, l'historique complet et les compétences.
- `/data/user/projets/` : Banque de fichiers Markdown détaillant chaque projet technique. C'est ici que l'IA puise les détails spécifiques.
- `/docs/` : Documentation structurelle et consignes pour l'agent.

### 2. Candidatures & Offres
- `/data/offres/liste.json` : Liste centralisée des offres cibles.
- `/data/offres/raw/` : Banque de fichiers Markdown contenant le texte brut de chaque offre scrapée.

### 3. Moteurs de CV
- `/engines/web/` : **Moteur High-Fidelity**. Système basé sur HTML/CSS/JS pour générer des CV "Premium".
    - `/engines/web/resume/data.json` : Référentiel complet (contenu et labels dynamiques).
    - `cover-letter/data.json` : Référentiel de données (contenu + labels) pour la lettre.
    - `resume/` & `cover-letter/` : Dossiers contenants la logique de rendu (HTML/CSS/JS).
- `/engines/output/` : Exports générés (PDF, HTML, etc.).
- `/data/user/resume-template/` : Modèles Markdown standardisés (ATS).

---

## Scripts & Automatisation

Le dépôt utilise une stack technique légère pour l'automatisation :

### Scraping & Automatisation
- **Script principal** : `/engines/scripts/scrape_offres.py`
- **Configuration YAML** : `/scrape-offres.yaml` (à la racine)
- **Source des URLs** : `/data/offres/liste.json`
- **Utilitaire CV** : `/engines/scripts/cv_tool.py` (Standardisation Markdown)

### Conventions du Scraper
- Les offres générées (RAW Markdown) sont stockées dans `/data/offres/raw/`.
- Les snapshots HTML et les rapports JSON sont désactivés par défaut.
- L'option `--overwrite` est recommandée pour mettre à jour les offres existantes.

### Environnement (Nix)
- `shell.nix` : Définit l'environnement reproductible (Python, dépendances, outils PDF).
- Pour entrer dans l'environnement : `nix-shell`

---

## Fonctionnement du CV Web

Le CV Web est conçu pour remplacer les éditeurs graphiques par une approche "CV-as-Code".

### Commandes Générales
1. **Lancement du serveur** :
   ```bash
   cd engines/web
   python3 -m http.server 8000
   ```
2. **Export PDF** :
   - Ouvrir `http://localhost:8000` (wrapper CV + Lettre).
   - Utiliser le bouton "Download PDF" ou `Ctrl+P`.
   - **Important** : Désactiver les marges et activer les graphiques d'arrière-plan dans les options d'impression du navigateur.

---

## Commandes Utiles

| Action | Commande |
| :--- | :--- |
| Entrer dans l'environnement | `nix-shell` |
| Lancer le scraper | `python engines/scripts/scrape_offres.py --config scrape-offres.yaml` |
| Sync & Scrape | `python engines/scripts/scrape_offres.py --sync` |
| Standardiser un CV MD | `python3 engines/scripts/cv_tool.py [chemin_du_fichier.md]` |
