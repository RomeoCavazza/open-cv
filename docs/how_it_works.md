# Fonctionnement du Dépôt

Ce document décrit l'organisation des fichiers et les processus de génération des documents de candidature.

## Organisation du Dépôt

### 1. Données Utilisateur
- `/engines/data/user/profile.md` : Profil de référence contenant l'identité, l'historique et les compétences.
- `/engines/data/user/projets/` : Dossiers Markdown détaillant les réalisations techniques.
- `/docs/` : Documentation technique et guides d'utilisation.

### 2. Candidatures & Offres
- `/engines/data/offres/liste.json` : Index des offres cibles.
- `/engines/data/offres/raw/` : Texte brut des offres d'emploi au format Markdown.

### 3. Interface de Rendu Web
- `/engines/web/` : Interface de visualisation dynamique.
    - Le dashboard (`index.html`) charge les fichiers JSON depuis `/engines/data/instances/` via le paramètre `id`.
    - `/engines/web/data/` : Point d'accès (liens symboliques) vers le dossier `/engines/data`.
- `/engines/output/` : Emplacement des fichiers exportés.
- `/engines/data/templates/` : Modèles JSON servant de base à la création de nouvelles instances.

---

## Scripts de Traitement

### Gestion des Dossiers
- **Initialisation** : `python3 engines/scripts/cv_tool.py init-all`
  - Crée l'arborescence des fichiers pour chaque offre listée.
- **Synchronisation** : `python3 engines/scripts/personalize_all.py`
  - Applique les mappings de formation, met à jour les noms d'entreprises et harmonise le contenu des instances.

### Acquisition des Offres
- **Script de scraping** : `/engines/scripts/scrape_offres.py`
- **Configuration** : `/scrape-offres.yaml`

---

## Utilisation de l'Interface Web

L'interface permet de prévisualiser les documents avant export.

1. **Serveur local** :
   ```bash
   python3 -m http.server 8000 --directory engines/web
   ```
2. **Visualisation** :
   - Sélectionner une offre dans la barre latérale pour charger les données correspondantes.
   - Utiliser la fonction d'impression du navigateur pour générer le PDF.

---

## Commandes Principales

| Objectif | Commande |
| :--- | :--- |
| Créer les dossiers d'instances | `python3 engines/scripts/cv_tool.py init-all` |
| Harmoniser les contenus | `python3 engines/scripts/personalize_all.py` |
| Récupérer de nouvelles offres | `python engines/scripts/scrape_offres.py --config scrape-offres.yaml` |

---

## Standard de Données JSON
Pour garantir un rendu "Limpide" et professionnel, les fichiers `resume.json` doivent respecter les formats suivants :
- **Compétences** : `"category": "Backend"`, `"items": ["FastAPI", "Rust"]`. 
  - *Note : Jamais de versions (Next.js 14 -> Next.js) ni de parenthèses.*
- **Périodes** : Utiliser `"2025 – Présent"` pour les projets en cours de maintenance.
- **Réalisations** : Un titre descriptif suivi de 2 à 3 points factuels d'une seule ligne.
