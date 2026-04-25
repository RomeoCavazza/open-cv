# Alternance — Pipeline de Candidature de Haute Fidélité

Ce dépôt contient l'écosystème complet pour ma recherche d'alternance (Master Robotique & IoT). Il centralise les offres, le portfolio et le système de génération de CV automatisé.

## 🚀 CV-as-Code (`cv-web`)

Le dossier `cv-web` contient un système de génération de CV haute fidélité basé sur les technologies web (HTML/CSS/JS). Ce système remplace mon ancien workflow Canva par une approche automatisée et plus précise.

### Structure du projet
- `data.json` : **Source de vérité**. Contient toutes les informations (expériences, formations, compétences). C'est le seul fichier à modifier pour mettre à jour le contenu.
- `index.html` : Structure sémantique du CV.
- `style.css` : Design "Premium" avec header en capsule, photo en relief et grille A4 stricte.
- `script.js` : Moteur de rendu dynamique.

### Comment générer un PDF ?
1. Lancer un serveur local dans le dossier `cv-web` :
   ```bash
   python3 -m http.server 8000
   ```
2. Ouvrir `http://localhost:8000` dans un navigateur (Chrome/Edge recommandé).
3. Cliquer sur le bouton **"Download PDF (A4)"** ou faire `Ctrl+P`.
4. **Paramètres d'impression critiques** :
   - Destination : Enregistrer au format PDF.
   - Taille du papier : **A4**.
   - Marges : **Aucune** (très important pour respecter le design).
   - Graphiques d'arrière-plan : **Activés**.

## 📁 Organisation du Dépôt
- `/cv-web` : Moteur de CV haute fidélité.
- `/offres` : Suivi et analyse des offres d'alternance (Markdown).
- `/portfolio` : Projets techniques détaillés.
- `/scripts` : Outils de scraping et d'automatisation.

## 🛠️ Workflow de Personnalisation
Pour chaque offre :
1. Analyser l'offre dans `/offres`.
2. Mettre à jour les mots-clés et le pitch dans `cv-web/data.json`.
3. Générer le PDF correspondant.

---
*Maintenu par Roméo Cavazza — 2026*
