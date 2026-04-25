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
### 3. Moteurs de CV
- `/engines/web/` : **Moteur High-Fidelity**. Système dynamique HTML/CSS/JS.
    - Le dashboard (`index.html`) utilise le paramètre `?id=job_id` pour charger dynamiquement les fichiers JSON depuis `/data/instances/`.
    - `/engines/web/data/` : Contient des symlinks vers le dossier `/data` pour permettre l'accès aux instances par le navigateur.
- `/engines/output/` : Exports générés (PDF, HTML, etc.).
- `/data/templates/` : Modèles JSON de base utilisés par `cv_tool.py init`.

---

## Scripts & Automatisation

### Gestion des Instances
- **Initialisation** : `python3 engines/scripts/cv_tool.py init-all` (Crée les dossiers d'instances pour chaque offre).
*   **Personnalisation** : `python3 engines/scripts/personalize_all.py` (Applique le mapping IA/Cloud/Embarqué, synchronise les noms d'entreprises et nettoie le jargon sur tous les fichiers).

### Scraping
- **Script principal** : `/engines/scripts/scrape_offres.py`
- **Configuration YAML** : `/scrape-offres.yaml`
- **Source des URLs** : `/data/offres/liste.json`

---

## Fonctionnement du CV Web

Le Dashboard (`engines/web/index.html`) est l'interface centrale.

1. **Lancement du serveur** :
   ```bash
   python3 -m http.server 8000 --directory engines/web
   ```
2. **Utilisation** :
   - Sélectionner une offre dans la barre latérale.
   - Le CV et la Lettre se mettent à jour instantanément.
   - Utiliser le bouton "Télécharger PDF" (ou `Ctrl+P`).

---

## Commandes Utiles

| Action | Commande |
| :--- | :--- |
| Initialiser toutes le instances | `python3 engines/scripts/cv_tool.py init-all` |
| Personnaliser tout (Audit) | `python3 engines/scripts/personalize_all.py` |
| Lancer le scraper | `python engines/scripts/scrape_offres.py --config scrape-offres.yaml` |

