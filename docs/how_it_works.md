# Fonctionnement du Dépôt

Ce document décrit l'organisation des fichiers et les processus de génération des documents de candidature.

## Organisation du Dépôt

### 1. Données Utilisateur
- `/data/user/profile.md` : Profil de référence contenant l'identité, l'historique et les compétences.
- `/data/user/projets/` : Projets personnels et techniques embarqués en submodules Git.
- `/data/user/old-cv/` : Historique des anciens CV Markdown servant de matière de référence.
- `/docs/` : Documentation technique et guides d'utilisation.

### 2. Candidatures & Offres
- `/data/offres/liste.json` : Index des offres cibles (Source de vérité).
- `/data/offres/liste.md` : Variante éditable humaine de la liste des offres.
- `/data/instances/` : Dossiers des instances personnalisées (un dossier par offre).
- `/data/offres/raw/` : Texte brut canonique des offres d'emploi au format Markdown.
- `/data/templates/` : Modèles JSON servant de base à la création de nouvelles instances.

### 3. Interface de Rendu Web
- `/web/` : Interface de visualisation dynamique.
  - Le dashboard (`index.html`) charge les fichiers JSON directement depuis `/data/`.
  - `/web/resume/` : Moteur de rendu des CV.
  - `/web/cover-letter/` : Moteur de rendu des lettres de motivation.

---

## Scripts de Traitement

### Gestion des Dossiers
- **Initialisation** : `python3 scripts/cv_tool.py init-all`
  - Crée l'arborescence des fichiers JSON pour chaque offre listée dans `liste.json`.
- **Synchronisation** : `python3 scripts/personalize_all.py`
  - Applique les mappings de formation (Master IA/Cloud/etc.) et harmonise le contenu des instances.

### Acquisition des Offres
- **Script de scraping** : `/scripts/scrape_offres.py`
- **Configuration** : `/scrape-offres.yaml`
  - Les offres scrapées sont enregistrées uniquement dans `/data/offres/raw/`.

---

## Utilisation de l'Interface Web

L'interface permet de prévisualiser les documents avant export.

1. **Serveur local** :
   ```bash
   # Lancer depuis la racine du projet
   python3 -m http.server 8000
   ```
2. **Visualisation** :
   - Ouvrir `http://localhost:8000/web/`
   - Sélectionner une offre dans la barre latérale pour charger les données correspondantes.
   - Utiliser la fonction d'impression du navigateur (Ctrl+P) pour générer le PDF.

---

## Commandes Principales

| Objectif | Commande |
| :--- | :--- |
| Créer les dossiers d'instances | `python3 scripts/cv_tool.py init-all` |
| Harmoniser les contenus | `python3 scripts/personalize_all.py` |
| Récupérer de nouvelles offres | `python3 scripts/scrape_offres.py --config scrape-offres.yaml` |

---

## Standard de Données JSON
Pour garantir un rendu "Limpide" et professionnel, les fichiers `resume.json` doivent respecter les formats suivants :
- **Compétences** : `"category": "Backend"`, `"items": ["FastAPI", "Rust"]`. 
  - *Note : Jamais de versions (Next.js 14 -> Next.js) ni de parenthèses.*
- **Périodes** : Utiliser `"2025 – Présent"` pour les projets en cours de maintenance.
- **Réalisations** : Un titre descriptif suivi de 2 à 3 points factuels d'une seule ligne.

---

## Règles de Structure

- `data/` est la seule source canonique des données métier.
- `web/` ne contient aucun miroir de données et lit directement dans `/data/`.
- `scripts/` centralise les chemins et les workflows CLI.
- Il n'y a pas d'API locale dans l'état actuel du dépôt.
