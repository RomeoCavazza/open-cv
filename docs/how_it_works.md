# Fonctionnement du Dépôt

Ce document détaille l'architecture technique et le fonctionnement opérationnel de l'écosystème de candidature.

## Architecture du Dépôt

### 1. Autodocumentation & Sources
- `/portfolio/profil.md` : **Source de vérité absolue**. Contient l'identité, l'historique complet et les compétences.
- `/portfolio/projets/` : Banque de fichiers Markdown détaillant chaque projet technique. C'est ici que l'IA puise les détails spécifiques.
- `/docs/` : Documentation structurelle et consignes pour l'agent.

### 2. Candidatures & Offres
- `/offres/liste.md` : Liste centralisée des offres cibles.
- `/offres/offres/` : Banque de fichiers Markdown contenant le texte brut de chaque offre scrapée.

### 3. Moteurs de CV
- `/cv/markdown/` : Stockage des CV générés en format Markdown (standard ATS).
- `/cv/web/` : **Moteur High-Fidelity**. Système basé sur HTML/CSS/JS pour générer des CV "Premium".
    - `data.json` : Le référentiel de données utilisé par le moteur web.
    - `index.html` / `style.css` / `script.js` : Logique de rendu et design.

---

## Scripts & Automatisation

Le dépôt utilise une stack technique légère pour l'automatisation :

### Scraping & Automatisation
- **Script principal** : `/scripts/scrape_offres.py`
- **Configuration YAML** : `/config/scrape-offres.yaml`
- **Source des URLs** : `/config/offres-urls.raw.json`
- **Utilitaire CV** : `/scripts/cv_tool.py` (Standardisation Markdown)

### Conventions du Scraper
- Les offres générées sont stockées dans `/offres/offres/`.
- Les snapshots HTML et les rapports JSON sont désactivés par défaut.
- L'option `--overwrite` est recommandée pour mettre à jour les offres existantes.

### Environnement (Nix)
- `shell.nix` : Définit l'environnement reproductible (Python, dépendances, outils PDF).
- Pour entrer dans l'environnement : `nix-shell`

---

## Fonctionnement du CV Web

Le CV Web est conçu pour remplacer les éditeurs graphiques par une approche "CV-as-Code".

### Commandes Générales
1. **Lancement du serveur** (nécessaire pour charger `data.json` via fetch) :
   ```bash
   cd cv/web
   python3 -m http.server 8000
   ```
2. **Export PDF** :
   - Ouvrir `http://localhost:8000`.
   - Utiliser le bouton "Download PDF" ou `Ctrl+P`.
   - **Important** : Désactiver les marges et activer les graphiques d'arrière-plan dans les options d'impression du navigateur.

---

## Commandes Utiles

| Action | Commande |
| :--- | :--- |
| Entrer dans l'environnement | `nix-shell` |
| Lancer le scraper | `python scripts/scrape_offres.py --config config/scrape-offres.yaml` |
| Servir le CV Web | `python3 -m http.server 8000 --directory cv/web` |
| Standardiser un CV MD | `python3 scripts/cv_tool.py [chemin_du_fichier.md]` |
