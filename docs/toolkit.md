# RecruitAI — Toolkit et Outils de Développement

Ce document répertorie les outils et commandes essentiels pour la maintenance, l'audit et l'optimisation du projet RecruitAI.

## 1. Structure et Code (La carte du projet)
- eza : Visualisation de l'architecture sans pollution.
  ```bash
  eza --tree --level=3 --ignore-glob="target|node_modules|.git|.direnv" --icons
  ```
- tokei : Comptage précis des fichiers et lignes de code.
  ```bash
  nix run nixpkgs#tokei
  ```

## 2. Poids et Optimisation (Le régime)
- dust : Identification des dossiers/fichiers volumineux.
  ```bash
  nix run nixpkgs#dust -- -X target -X .git
  ```
- cargo-bloat : Analyse du poids du binaire Rust final.
  ```bash
  nix shell nixpkgs#cargo-bloat -c cargo bloat --release --crates
  ```

## 3. Sécurité et Dépendances (Le check-up)
- cargo update : Mise à jour des dépendances.
- cargo audit : Vérification des failles de sécurité connues.
- cargo tree : Analyse de l'arbre de dépendances (doublons).
  ```bash
  cargo tree -e no-dev --duplicates
  ```
- cargo-deny : Audit des licences, vulnérabilités et doublons.

## 4. Environnement de Dev (La robustesse Nix)
- nix develop : Mesure du temps de chargement de l'environnement.
  ```bash
  time nix develop --command echo "Shell chargé"
  ```
- nix flake show : Structure des inputs/outputs du Flake.
- nix flake check : Validation formelle du Flake (reproductibilité).

## 5. Outils Spécifiques à Rust
- cargo clippy : Linter pour détecter les mauvais patterns.
- cargo fmt : Formateur de code standard.
- cargo-udeps : Détection des dépendances inutilisées.
- cargo-depgraph : Génération de diagrammes de dépendances.
- cargo-modules : Génération de l'arborescence et des graphes d'architecture.
- cargo-flamegraph : Profilage CPU pour repérer les goulots d'étranglement.
- Criterion.rs : Micro-benchmarking de performance.
- cargo tarpaulin : Calcule la couverture de code par les tests.

## 6. Outils Web (HTML/CSS/JS)
- ESLint : Linter JavaScript.
- Stylelint : Linter CSS.
- PurgeCSS : Suppression du CSS inutilisé.
- Knip : Chasse aux fichiers, exports et dépendances inutilisés côté JS.
- Lighthouse : Audit web (performances, accessibilité) intégré au navigateur.

## 7. Analyse Globale et Architecture
- SonarQube / SonarCloud : Scanner global (complexité, dette technique).
- PlantUML : Génération de diagrammes techniques.

## 8. Environnement, CI/CD et Gestion de versions
- GitHub Actions / GitLab CI : Intégration et déploiement continu.
- Git : Système de contrôle de version.
- Nix : Gestionnaire de paquets et d'environnements reproductibles.
- Just (via Justfile) : Lanceur de commandes (command runner).
