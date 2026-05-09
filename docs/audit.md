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
| **Gestion d'État** | Fragmentation entre `window.state` et `localStorage`. | Risque de désynchronisation visuelle entre les iframes. | **Moyenne** |
| **Centralisation** | Logique de polling historiquement dupliquée. | **Résolu** : Migration vers l'architecture *Master Poller* réactive. | **Check** |

---

## 2. Analyse de l'Architecture

### 2.1 Points Forts (Core Assets)
- **Découplage Total** : L'abstraction via `crates/ports` permet de switcher de `Claude` à `Ollama` ou de `Postgres` à `In-memory` (pour les tests) en 2 lignes de code.
- **RAG Performant** : L'usage combiné de `pgvector` (cosine similarity) et `pg_trgm` (fuzzy search) garantit une pertinence contextuelle de haut niveau.
- **Légèreté** : L'absence de frameworks (React/Next) côté front assure un temps de chargement instantané (< 100ms).

### 2.2 Points de Vigilance (Risques)
- **Atrocité du SQL métier** : Le trigger de catégorisation est une "boîte noire". Il devrait être migré vers une `OffreService` en Rust pour profiter du pattern matching.
- **Atomicité des Livrables** : Le backend devrait supporter la mise à jour partielle des statuts (`resume_status: Ready`, `cover_letter_status: Generating`) pour une UX parfaite.

---

## 3. Plan d'Action (Hardening Roadmap)

### Étape 1 : Robustesse Système (Immediate)
- [ ] **Job Persistence** : Introduire une table `background_jobs` pour que `tokio::spawn` puisse reprendre après un crash.
- [ ] **Validation de Troncature** : Implémenter un découpage (chunking) intelligent des profils trop longs avant l'envoi au LLM.

### Étape 2 : Excellence UX (En cours)
- [x] **Master Poller** : Centraliser le polling réseau dans `dashboard.js` (Terminé).
- [x] **Audio Feedback** : Notification sonore globale via `storage events` (Terminé).
- [ ] **Partial Refresh** : Permettre de recharger un seul document sans rafraîchir toute l'instance.

### Étape 3 : Nettoyage & Standardisation
- [ ] **Migration Regex** : Sortir les Regex de catégorisation du SQL pour les mettre dans `crates/application`.
- [ ] **Consolidation d'État** : Migrer `window.state` vers un `Store` minimaliste partagé.

---

## 4. Conclusion de l'Auditeur
Le projet RecruitAI dispose d'une fondation Rust exceptionnelle. Les faiblesses actuelles se situent uniquement dans la couche de "confort" (gestion de tâches, réactivité UI). **L'investissement dans l'architecture hexagonale paye déjà ses fruits en rendant ces améliorations simples à implémenter sans refactorisation lourde.**
