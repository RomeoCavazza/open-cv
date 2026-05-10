# Audit Technique & Hardening Roadmap - RecruitAI

Date de l'audit : 9 mai 2026
Statut global : **Sain & Industriel**
Architecture : **Hexagonale (Workspace Rust)**

## 1. État des Lieux & Dette Technique

### 1.1 Backend (Rust)

| Composant | Problématique | Risque | Priorité |
| :--- | :--- | :--- | :--- |
| **Gestion des Tâches** | `tokio::spawn` utilisé pour la génération asynchrone sans persistance. | Si le serveur redémarre, les générations en cours sont perdues sans reprise. | **Haute** |
| **Logique métier SQL** | La classification des offres (`fn_infer_offre_category`) est figée dans un trigger SQL via Regex. | Difficile à tester unitairement et à faire évoluer sans migration DB. | **Moyenne** |
| **Couplage d'État** | Un seul `status` global par `Instance` dans la DB. | Impossible de savoir si c'est le CV ou la Lettre qui a échoué précisément. | **Moyenne** |
| **Erreurs de Troncature** | Risque de saturation de contexte LLM sur les gros profils. | `LlmError::Truncated` non géré de manière élégante (échec sec). | **Basse** |
| **Optimisation Binaire** | Taille actuelle : **6.2 MiB**. | Aucun risque réel, performance excellente, mais divergence avec les docs précédents. | **Basse** |

### 1.2 Frontend (Vanilla JS)

| Composant | Problématique | Risque | Priorité |
| :--- | :--- | :--- | :--- |
| **Réactivité** | Manipulation directe du DOM via `getElementById`. | Code verbeux, difficile à maintenir si le nombre de livrables augmente. | **Moyenne** |
| **Remédiation** | *Envisager Alpine.js ou HTMX pour simplifier la réactivité (voir Blueprint Section 9).* | | |
| **Gestion d'État** | Fragmentation entre `window.state` et `localStorage`. | **Résolu** : Synchronisation réactive via `GEN_STARTED` & storage events. | **Check** |
| **Centralisation** | Logique de polling historiquement dupliquée. | **Résolu** : Master Poller implémenté (reste optimisation `postMessage`). | **Check** |
| **Data-Binding** | Champs de profil pré-remplis par défaut (fallbacks hardcodés). | **Résolu** : Suppression des valeurs par défaut dans le JS pour respecter la nudité de la DB. | **Check** |

---

## 2. Analyse de l'Architecture

### 2.1 Points Forts (Core Assets)
- **Découplage Total** : L'abstraction via `crates/ports` permet de switcher de `Claude` à `Ollama` ou de `Postgres` à `In-memory` (pour les tests) en 2 lignes de code.
- **RAG Performant** : L'usage combiné de `pgvector` (cosine similarity) et `pg_trgm` (fuzzy search) garantit une pertinence contextuelle de haut niveau.
- **Légèreté** : L'absence de frameworks (React/Next) côté front assure un temps de chargement instantané (< 100ms).

### 2.2 Points de Vigilance (Risques)
- **Atrocité du SQL métier** : Le trigger de catégorisation est une "boîte noire". Il devrait être migré vers une `OffreService` en Rust pour profiter du pattern matching.
- **Atomicité des Livrables** : Le backend devrait supporter la mise à jour partielle des statuts (`resume_status: Ready`, `cover_letter_status: Generating`) pour une UX parfaite.

### 2.3 État de la Synchronisation UI (Feedback Ingestion/Génération)
- **Synchronisation Optimiste** : L'usage coordonné de `GEN_STARTED` et du `localStorage` garantit une cohérence visuelle instantanée entre la Sidebar (icône Double Span) et l'Iframe (Skeleton interne des templates). La coordination a été stabilisée en réintégrant le skeleton immédiat via `srcdoc` dans `view.js`.
- **Intégrité du Profil** : Les mécanismes de chargement/sauvegarde du profil ont été audités. Les champs vides sont désormais correctement rendus comme tels, évitant les "fantômes" de session.
- **Points d'attention (Optimisation)** :
    - **Redondance du Polling** : Le parent (`Master Poller`) et l'Iframe effectuent actuellement des requêtes de statut en parallèle. Bien que l'impact réseau soit négligeable, une communication via `postMessage` permettrait d'éliminer cette redondance.
    - **Poids du Rendu Sidebar** : La fonction `loadOffers()` reconstruit l'intégralité du DOM de la sidebar à chaque rafraîchissement. Pour des volumes importants d'offres (> 100), une approche par manipulation chirurgicale du DOM ou l'usage d'Alpine.js sera préférable pour maintenir la fluidité.

---

## 3. Plan d'Action (Hardening Roadmap)

### Étape 1 : Robustesse Système (Immediate)
- [ ] **Job Persistence** : Introduire une table `background_jobs` pour que `tokio::spawn` puisse reprendre après un crash.
- [ ] **Validation de Troncature** : Implémenter un découpage (chunking) intelligent des profils trop longs avant l'envoi au LLM.

### Étape 2 : Excellence UX (En cours)
- [x] **Master Poller** : Centraliser le polling réseau dans `dashboard.js` (Terminé).
- [x] **Audio Feedback** : Notification sonore globale via `storage events` (Terminé).
- [x] **Ingestion Flexible** : Autoriser les prompts courts (Terminé).
- [x] **Unification des flux** : Les prompts directs sont traités comme des offres normales sans labels isolés (Terminé).
- [x] **Fix Flash Skeleton** : Blocage de la navigation iframe pendant la génération pour éviter les flashs blancs (Terminé).
- [x] **Optimisation Audio** : Notification sonore unique en fin de lot complet (Terminé).
- [ ] **Partial Refresh** : Permettre de recharger un seul document sans rafraîchir toute l'instance.

### Étape 3 : Nettoyage & Standardisation (Terminée)
- [x] **Fusion des Migrations** : Schéma V1 unifié et table d'Undo (Terminé).
- [x] **Snapshot & Undo** : Persistance réelle des versions précédentes pour le chat (Terminé).

### Étape 4 : Scalability & Production (Next Steps)
- [ ] **Job Persistence** : Migration vers une table de jobs Postgres pour garantir la reprise après crash.
- [ ] **Refactoring UI** : Introduction d'Alpine.js pour simplifier la manipulation du DOM.
- [ ] **Dockerization** : Création de l'image de production et déploiement cloud.

---

## 4. Conclusion de l'Auditeur
Le projet RecruitAI dispose d'une fondation Rust exceptionnelle. Les faiblesses actuelles se situent uniquement dans la couche de "confort" (gestion de tâches, réactivité UI). **L'investissement dans l'architecture hexagonale paye déjà ses fruits en rendant ces améliorations simples à implémenter sans refactorisation lourde.**
