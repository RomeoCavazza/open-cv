# RecruitAI — Spécifications Techniques (Blueprint)

Ce document est la référence technique du projet. Il décrit l'architecture hexagonale, le pipeline IA et les choix technologiques structurants.

## 1. Vision et Objectifs
Le système transforme une offre d'emploi brute en un pack de candidature complet et structuré :
1. Analyse "Reverse-Engineering" : Décodage des missions réelles et des enjeux cachés.
2. CV sur mesure : JSON structuré injecté dans un moteur de rendu A4.
3. Lettre de motivation : Argumentaire ciblé découpé en sections sémantiques.

## 2. Stack Technologique
- Backend : Rust (Axum + Tokio + SQLx).
- Base de données : PostgreSQL 16 + pgvector (RAG).
- IA : Abstraction multi-modèles (Anthropic Claude, OpenAI, Ollama).
- Frontend : Vanilla JS / HTML5 / CSS3 (Zéro framework).
- Environnement : Nix (reproductibilité) + Just (automatisation).

## 3. Architecture Hexagonale (Workspace Cargo)
Le code est découpé en 5 crates pour assurer un découplage total de la logique métier :
- domain : Modèles de données purs.
- ports : Interfaces (traits) définissant les besoins du domaine.
- adapters : Implémentations concrètes.
- application : Cas d'utilisation et orchestration RAG.
- api : Point d'entrée HTTP Axum et service des fichiers statiques.

## 4. Pipeline de Génération IA
L'intelligence du système repose sur la génération structurée :
1. Retrieval : Recherche vectorielle des segments de profil pertinents.
2. Reranking : Scoring des segments par l'IA.
3. Planning : Définition de la stratégie de réponse.
4. Extraction : Génération des payloads JSON contraints.
5. Persistance : Validation métier et stockage dans PostgreSQL.

## 5. Architecture Frontend
Le frontend est minimaliste pour garantir performance et pérennité :
- Isolation via Iframes : Les documents sont rendus dans des iframes isolées.
- Polling Centralisé (Master Poller) : Une seule boucle de polling dans la fenêtre parente gère les requêtes API pour tous les documents en cours, réduisant la charge réseau.
- État Réactif via Storage : Utilisation des événements `window.onstorage` pour notifier les iframes des changements d'état sans polling redondant.
- Vanilla ES6 : Utilisation intensive de modules natifs.
- Routage Client : Gestion des vues via history.pushState.

## 6. Modèle de Données (Postgres)
- offres : Source brute + analyse IA.
- profils : Données candidat + annexes binaires.
- chunks : Fragments de profil vectorisés pour le RAG.
- instances : Lien profil-offre + historique des messages de chat.

## 7. Roadmap et Hardening (MVP stabilisé)

La priorité absolue est le **Hardening de la pipeline Data/AI** et l'optimisation de l'expérience interactive.

### PHASE A : Hardening Pipeline (Terminée)
1. **Scraping Industriel** :
    - [x] Validation du parsing sur les plateformes majeures (LinkedIn, Indeed, Welcome to the Jungle).
    - [x] Implémentation du fallback **ScrapingAnt** (Proxy/Anti-bot/Cloudflare).
    - [x] Support des "Prompts Directs" : Génération sans URL.
2. **Génération & Queueing** :
    - [x] Orchestration asynchrone pour gérer plusieurs liens à la suite.
    - [x] Mise en place d'une file d'attente (Queuing) via sémaphores.
    - [x] Limitation dure des demandes d'ingestion (max 5 par requête) avec compteur de demandes ignorées.
    - [x] Support hybride URL + prompts directs dans un même input.
    - [x] Synchronisation Sidebar : Apparition instantanée.

### PHASE B : Intelligence Interactive (Terminée)
1. **Chatbar & RAG Optimization** :
    - [x] Vérification de l'injection Contextuelle : S'assurer que le profil et les chunks RAG sont transmis à 100%.
    - [x] **JSON Mutations** : Permettre au LLM de modifier directement la structure JSON des documents via le chat.
    - [x] Comportement LLM : Ajustement du ton et de l'efficacité en phase de "refining".
2. **UI "Alive" & Versioning** :
    - [x] Micro-animations d'attente (status planning/reasoning/generating).
    - [x] **Mini-Versioning** : Système de snapshots et Undo persistant en base de données.

## 8. Validation Q&A End-to-End
Pour garantir une stabilité durable, les scénarios suivants sont validés :
- [x] Ingestion d'une offre protégée par Cloudflare.
- [x] Génération concurrente en arrière-plan.
- [x] Limite d'ingestion (>5 demandes) avec rejet explicite du surplus.
- [x] Ingestion hybride (liens + demandes textuelles) dans une seule soumission.
- [x] Modification d'un paragraphe du CV via le chat et validation du JSON résultant.
- [x] Export PDF via l'interface.

- [x] Génération via dashboard global (restitution, cv, cover letter).
- [x] Génération via slots vides individuels.
- [x] Régénération via icônes d'écrasement.
- [x] Rendu immédiat post-génération (disparition du skeleton via BackgroundPollManager).
- [x] Cohérence du chat avec injection complète du `JSON.profile`.

## 9. Phase C : Production & Scalabilité (Roadmap)

Une fois le MVP stabilisé, la trajectoire de croissance s'articule autour de trois axes :

### 9.1 Robustesse & Background Jobs
*   **Problème** : `tokio::spawn` est volatile. Un crash serveur = perte des générations en cours.
*   **Solution** : Implémenter une file d'attente persistante en base de données (`background_jobs`).
*   **Impact** : Fiabilité 100% et possibilité de reprendre les tâches après redémarrage.

### 9.2 Refactoring UI (Alpine.js / HTMX)
*   **Problème** : Le Vanilla JS devient verbeux pour les interactions complexes.
*   **Solution** : Migrer les composants interactifs (sidebar, chat, notifications) vers **Alpine.js**.
*   **Impact** : Code frontend réduit de 40%, meilleure maintenabilité et réactivité accrue.

### 9.3 Cloud Readiness & Docker
*   **Packaging** : Création d'une image Docker multi-stage (binaire statique < 10MB).
*   **Déploiement** : Configuration d'une pipeline CI/CD pour déploiement automatique sur Fly.io ou VPS NixOS.
*   **Observabilité** : Utilisation de la vue SQL `v_llm_costs_daily` pour piloter les marges.

---

## 10. Évolution du Frontend (Pistes de Réflexion)

Bien que l'approche **Vanilla JS** soit actuelle et ultra-performante, deux technologies s'alignent parfaitement avec la philosophie "minimaliste et robuste" du projet pour réduire la verbosité du code de manipulation du DOM :

### 10.1 HTMX (Le choix du Server-Side)
- **Concept** : Permet d'effectuer des requêtes AJAX, de gérer des WebSockets et des Server-Sent Events directement via des attributs HTML.
- **Avantage pour RecruitAI** : Au lieu de recevoir du JSON et de reconstruire le DOM en JS (ex: la sidebar des offres), Axum pourrait renvoyer directement un fragment HTML.
- **Bénéfice** : Suppression de 50% du code JS côté client. Cohérence totale avec la puissance du backend Rust.

### 10.2 Alpine.js (Le "Tailwind du JS")
- **Concept** : Un framework déclaratif ultra-léger (8kb) qui s'utilise directement dans le HTML pour gérer les états locaux (modals, onglets, toggles).
- **Avantage pour RecruitAI** : Remplacerait les `document.getElementById` verbeux pour la gestion des pillules LLM (`llm-pill`) et des états d'affichage des boutons.
- **Bénéfice** : Code frontend plus lisible, déclaratif et plus facile à maintenir sans ajouter de complexe de build (pas de npm/webpack nécessaire).

## 11. Matrice de Maîtrise Compétences (E/M/A/N)

Pour mieux comprendre la profondeur réelle d'une offre (et estimer les "skills supposés"), RecruitAI adopte une matrice à 4 niveaux :

| Niveau | Définition opérationnelle |
| :--- | :--- |
| **E** | Savoir agir dans un contexte complexe, faire preuve de créativité, trouver de nouvelles solutions, former d'autres agents, être référent dans le domaine. |
| **M** | Mettre en œuvre la compétence de manière régulière, corriger et améliorer le processus, conseiller les autres agents, optimiser le résultat. |
| **A** | Savoir effectuer, de manière occasionnelle ou régulière, correctement les activités, sous le contrôle d'un autre agent, savoir repérer les dysfonctionnements. |
| **N** | Disposer de notions de base, de repères généraux sur l'activité ou le processus (vocabulaire de base, principales tâches, connaissance du processus global). |

### 11.1 Usage dans l'analyse d'offre

Pour chaque offre, la restitution doit pouvoir produire un tableau de lecture :

| Skill / Domaine | Niveau requis estimé | Indices textuels de l'offre | Niveau candidat estimé | Écart |
| :--- | :--- | :--- | :--- | :--- |
| Gestion de projet SI | M | "pilotage", "coordination", "instances" | A | -1 |
| Analyse stratégique | M | "alignement", "trajectoire", "SSSI" | A | -1 |
| Outils data/IA | A | "reporting", "indicateurs", "outils numériques" | M | +1 |

Échelle d'écart recommandée :
- `+2` : surqualifié
- `+1` : au-dessus du besoin
- `0` : aligné
- `-1` : gap modéré
- `-2` : gap fort

### 11.2 Règle produit

Quand une compétence n'est pas explicitement mentionnée mais fortement implicite dans l'offre, elle est taguée **"supposée"** avec un niveau prudent (`N` ou `A` par défaut), puis justifiée dans les indices textuels.

---

## 12. Éditeur WYSIWYG de CV (Phase D)

> **Statut** : Blueprint validé — Implémentation non démarrée  
> **Décision du 16 mai 2026** : On documente tout ce soir, on code plus tard.

### 12.1 Vision

Aujourd'hui le pipeline génère `CV JSON → Template HTML statique`. Le rendu est figé.

**L'objectif** : Permettre à l'utilisateur de **personnaliser visuellement** le CV généré — couleurs, polices, disposition — directement dans le navigateur, puis d'exporter un PDF pixel-perfect. À côté du bouton "Télécharger le PDF", un bouton "Traduire" sera ajouté pour permettre de décliner instantanément le même CV dans plusieurs langues.

**Références** :
| Produit | Stack | Ce qu'on en retient |
|---------|-------|---------------------|
| **Figma** | C++/Rust → Wasm + TS | Moteur canvas ultra-performant, état centralisé |
| **Canva** | JS + Canvas API | UX drag-and-drop, templates, simplicité |
| **PowerPoint** | C++/C# (desktop) | Modèle "slide deck", formatage riche |

### 12.2 Architecture

**Règle d'or** : La vue n'est que le reflet d'un fichier de données (le State). On ne modifie jamais le DOM directement — on mute le JSON, le renderer redessine.

| Couche | Technologie | Responsabilité |
|--------|-------------|----------------|
| **UI Layer** | JS Vanilla + CSS | Écoute les interactions (clics, inputs), met à jour le State |
| **State Manager** | JS (objet JSON) | Source de vérité unique du document CV |
| **Preview Renderer** | HTML/CSS (v1) | Affichage temps réel du CV (re-render à chaque mutation) |
| **Layout Engine** | Rust → Wasm | Calcul des positions, gestion du texte (line-wrap, pagination) |
| **PDF Export** | Rust → Wasm (`genpdf`) | Génération d'un PDF haute fidélité à partir du State JSON |

### 12.3 Structure de Données (State JSON)

```json
{
  "meta": {
    "version": "1.0",
    "createdAt": "2026-05-16T22:00:00Z",
    "templateId": "modern-dark"
  },
  "theme": {
    "primaryColor": "#2A4365",
    "accentColor": "#4FD1C5",
    "textColor": "#1A202C",
    "backgroundColor": "#FFFFFF",
    "fontFamily": "Inter",
    "fontSizeBase": 10,
    "spacing": "comfortable"
  },
  "layout": {
    "columns": 1,
    "marginTop": 20,
    "marginSide": 25,
    "sectionGap": 12
  },
  "sections": [
    {
      "id": "header",
      "type": "identity",
      "visible": true,
      "data": {
        "nom_complet": "Roméo Cavazza",
        "titre": "ALTERNANCE — DATA ANALYST",
        "photo_url": "/api/profile/photo"
      }
    },
    {
      "id": "contact",
      "type": "contact",
      "visible": true,
      "data": {
        "email": "romeo.cavazza@gmail.com",
        "telephone": "+33 ...",
        "localisation": "Paris, France",
        "linkedin": "...",
        "github": "..."
      }
    },
    {
      "id": "exp-1",
      "type": "experience",
      "visible": true,
      "data": {
        "poste": "Développeur IA",
        "entreprise": "DGSI",
        "periode": "Janvier–Juin 2026",
        "bullets": ["...", "...", "..."]
      }
    },
    {
      "id": "skills",
      "type": "competences",
      "visible": true,
      "data": {
        "groupes": [
          { "categorie": "Langages", "items": ["Python", "TypeScript", "Rust"] }
        ]
      }
    }
  ]
}
```

**Pattern de mutation** :
```javascript
function updateState(path, value) {
    setNestedValue(cvState, path, value);
    render(cvState);              // Re-render
    saveToLocalStorage(cvState);  // Persistence locale
}

// Exemple : changement de couleur
colorPicker.addEventListener('input', (e) => {
    updateState('theme.primaryColor', e.target.value);
});
```

### 12.4 Approches de Rendu

#### Approche A — HTML/CSS ✅ (Retenue pour la v1)

| Aspect | Détail |
|--------|--------|
| **Principe** | Générer le CV en HTML stylisé avec des CSS Custom Properties |
| **Édition inline** | `contenteditable="true"` sur les champs textuels |
| **Theming** | Variables CSS (`--primary`, `--accent`, `--font-family`) |
| **Export PDF** | Côté Rust via Wasm |
| **Complexité** | ⭐⭐ Modérée |
| **Fidélité PDF** | ⭐⭐⭐ Bonne |

```css
.cv-container {
    --primary: var(--theme-primary, #2A4365);
    --accent: var(--theme-accent, #4FD1C5);
    font-family: var(--theme-font, 'Inter', sans-serif);
}
.cv-header { background: var(--primary); color: white; }
.cv-skill-tag { border: 1px solid var(--accent); }

@media print {
    .cv-container { width: 210mm; min-height: 297mm; }
    .editor-sidebar { display: none; }
}
```

#### Approche B — Canvas + Wasm (Évolution future, non retenue pour le MVP)

| Aspect | Détail |
|--------|--------|
| **Principe** | Rust dessine dans un `<canvas>` via Wasm |
| **Complexité** | ⭐⭐⭐⭐⭐ Très élevée |
| **Fidélité PDF** | ⭐⭐⭐⭐⭐ Pixel-perfect |

### 12.5 Export PDF via Rust/Wasm

C'est le cas d'usage principal de Rust + WebAssembly.

**Flux** : `JS State JSON` → `wasm-bindgen` → `Rust (genpdf)` → `Vec<u8>` → `Blob download`

#### Setup du crate `cv-engine`

**`crates/cv-engine/Cargo.toml`** :
```toml
[package]
name = "cv-engine"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde-wasm-bindgen = "0.6"
genpdf = "0.2"
```

**`crates/cv-engine/src/lib.rs`** (Prototype vertical) :
```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CvTheme {
    #[serde(rename = "primaryColor")]
    pub primary_color: String,
    #[serde(rename = "textColor")]
    pub text_color: String,
    pub font_family: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CvDocument {
    pub meta: serde_json::Value,
    pub theme: CvTheme,
    pub sections: Vec<serde_json::Value>,
}

#[wasm_bindgen]
pub fn generate_pdf(js_cv_state: JsValue) -> Result<Vec<u8>, JsValue> {
    let cv_data: CvDocument = serde_wasm_bindgen::from_value(js_cv_state)
        .map_err(|e| JsValue::from_str(&format!("Erreur parsing: {}", e)))?;

    // TODO: Remplacer par genpdf layout réel
    let mock_pdf_bytes = vec![0x25, 0x50, 0x44, 0x46]; // %PDF header
    Ok(mock_pdf_bytes)
}
```

**Côté JS** (consommation du Wasm) :
```javascript
import init, { generate_pdf } from './pkg/cv_engine.js';

async function run() {
    await init();
    document.getElementById('export-pdf-btn').addEventListener('click', () => {
        const pdfBytes = generate_pdf(cvState);
        const blob = new Blob([pdfBytes], { type: 'application/pdf' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `CV_${cvState.sections[0].data.nom_complet}.pdf`;
        a.click();
    });
}
run();
```

**Commande de compilation** :
```bash
wasm-pack build crates/cv-engine --target web --out-dir ../../web/pkg
```

### 12.6 Intégration avec le Pipeline RecruitAI

1. L'utilisateur ingère une offre → le LLM génère le CV JSON
2. Le hardcoding layer (`helpers.rs`) injecte identité, formations, langues
3. L'utilisateur ouvre l'éditeur WYSIWYG avec le CV pré-rempli
4. Il ajuste les couleurs, reformule un bullet point, réorganise les sections
5. Il exporte en PDF haute fidélité via Rust/Wasm

### 12.7 Roadmap d'Implémentation

| Phase | Contenu | Durée estimée |
|-------|---------|---------------|
| **Phase 1 — Foundation** | Schéma JSON final, setup `wasm-pack`, crate `cv-engine`, Hello World Rust↔JS | 1-2 semaines |
| **Phase 2 — Preview HTML/CSS** | Page éditeur (split panel), State Manager JS, rendu HTML/CSS, color/font pickers | 1-2 semaines |
| **Phase 3 — Édition Inline** | `contenteditable`, sync bidirectionnelle State↔DOM, drag-and-drop sections | 1 semaine |
| **Phase 4 — Export PDF** | `generate_pdf()` en Rust via `genpdf`, polices embarquées, téléchargement blob | 1-2 semaines |
| **Phase 5 — Polish** | Intégration API, sauvegarde auto, templates de thèmes, Undo/Redo | 1 semaine |

### 12.8 Décisions Validées (16 mai 2026)

| Question | Option retenue | Justification |
|----------|----------------|---------------|
| **Approche de rendu v1** | HTML/CSS | Vitesse de dev, accessibilité, `contenteditable` natif |
| **Crate PDF** | `genpdf` (abstraction de `printpdf`) | Layout et sauts de page automatiques, indispensable pour le format CV |
| **Persistence** | LocalStorage (Draft) + API (Sync) | Sauvegarde locale à chaque frappe, persistée en DB sur action explicite ou debounced |
| **Par quoi commencer ?** | Prototype vertical Rust ↔ JS | Valider immédiatement le passage du JSON du JS vers le Wasm |
