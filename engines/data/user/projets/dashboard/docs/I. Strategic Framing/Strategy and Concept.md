# projet.md — Stratégie & concept Dashboard Ligue 1

Ce document est le **périmètre stratégique** du projet : objectif, MVP, sections du dashboard, blocs fonctionnels et contraintes techniques. Il sert d’annexe aux prompts **architecture.md**, **data.md** et **theme.md**.

---

## Positionnement du document dans le workflow

```mermaid
flowchart LR
    A[projet.md] --> B[architecture.md]
    A --> C[data.md]
    A --> D[theme.md]
    B --> E[Build]
    C --> E
    D --> E
```

| Document | Rôle |
|----------|------|
| **projet.md** (ce fichier) | Scope, sections, contraintes API et Antigravity. |
| architecture.md | Layout, mapping UI → collections, ordre de build. |
| data.md | Endpoints, datasets, agrégations, flux de chargement. |
| theme.md | Palette, tokens, typo, composants (design). |

---

## 1. Description claire du projet

Création d’un **dashboard mono-page**, moderne et sombre (inspiration FootX), permettant de visualiser de manière synthétique les données de la **Ligue 1 (FL1)** via l’API officielle :

[https://api.football-data.org/v4/competitions/FL1](https://api.football-data.org/v4/competitions/FL1)

Authentification requise via header `X-Auth-Token`.

Le dashboard doit présenter une vue condensée de la compétition, orientée data-visualisation, et entièrement implémentable dans **Antigravity (no-code)**.

Aucun élargissement fonctionnel : uniquement une vue globale de la compétition.

---

## 2. Périmètre exact du MVP (mono-page uniquement)

Le MVP inclut exclusivement :

* Une vue unique présentant :

  * Informations générales de la compétition
  * Classement actuel
  * Statistiques globales simples
  * Visualisations compatibles avec le plan gratuit
* Aucune navigation interne
* Aucune page équipe, match ou joueur
* Aucune donnée live
* Aucune donnée avancée (lineups, événements détaillés)

---

## 3. Sections du dashboard (structure claire)

1. Header compétition
2. KPIs globaux
3. Classement général (tableau)
4. Visualisations statistiques
5. Footer minimal (source API)

---

## 4. Liste des blocs fonctionnels

### A. Header compétition

* Nom de la compétition
* Saison active
* Identité visuelle statique (logo Ligue 1 intégré manuellement)

---

### B. KPIs globaux (cards)

* Nombre total d’équipes
* Nombre total de matchs joués
* Total de buts marqués
* Moyenne de buts par match
* Journée en cours

Ces valeurs devront être calculées à partir des données disponibles via les endpoints officiels.

---

### C. Tableau de classement

Tableau simple contenant :

* Position
* Nom de l’équipe
* Points
* Victoires
* Nuls
* Défaites
* Différence de buts

Les noms exacts des champs seront validés après analyse réelle de l’endpoint :

/v4/competitions/FL1/standings

---

### D. Visualisations statistiques

Graphiques compatibles Antigravity :

* Bar chart horizontal : Top 5 équipes (points)
* Bar chart horizontal : Meilleures attaques
* Bar chart horizontal : Meilleures défenses
* Histogramme simple : Répartition des buts par journée

Les données nécessaires seront extraites de :

* /v4/competitions/FL1/matches
* /v4/competitions/FL1/standings

La structure exacte sera confirmée après inspection réelle des réponses JSON.

---

### E. Footer

Mention obligatoire :

“Données issues de l’API football-data.org v4”

---

## 5. Liste des types de données nécessaires

Sans inventer les champs exacts :

* Informations générales de la compétition
* Saison en cours
* Liste des équipes
* Classement (points, résultats, statistiques globales)
* Liste des matchs (scores, journées)
* Données nécessaires aux calculs d’agrégation (totaux et moyennes)

La validation précise des champs sera effectuée après analyse réelle des endpoints v4.

---

## 6. Contraintes techniques

### Contraintes Antigravity

* Utilisation exclusive des composants standards :

  * Table
  * KPI card
  * Bar chart
  * Histogramme
* Transformations simples uniquement (count, sum, average, tri).
* Mono datasource API.
* Une seule vue principale.
* Aucun calcul complexe côté client.

### Contraintes API football-data (plan gratuit)

* 10 requêtes par minute maximum.
* Pas de données live.
* Pas de lineups ou événements détaillés.
* Nécessité d’utiliser des endpoints séparés :

  * Competition
  * Standings
  * Matches
* Gestion efficace des appels (éviter les refresh inutiles).

---

## 7. Structure logique visuelle (schéma layout)

```
---------------------------------------------------------
| HEADER COMPÉTITION                                     |
| Nom | Saison | Identité visuelle                       |
---------------------------------------------------------
| KPIs GLOBAUX                                          |
| [Équipes] [Matchs] [Buts] [Moyenne] [Journée]         |
---------------------------------------------------------
| CLASSEMENT GÉNÉRAL                                    |
| Tableau complet standings                              |
---------------------------------------------------------
| VISUALISATIONS                                        |
| - Top 5 équipes (points)                              |
| - Meilleures attaques                                 |
| - Meilleures défenses                                 |
| - Buts par journée                                    |
---------------------------------------------------------
| FOOTER                                                |
| Source API                                            |
---------------------------------------------------------
```

---

## 8. Checklist opérationnelle

### A. Préparation

* [ ] Générer clé API football-data
* [ ] Tester endpoints FL1
* [ ] Inspecter structure JSON réelle
* [ ] Identifier champs nécessaires

---

### B. Construction

* [ ] Créer page unique Antigravity
* [ ] Connecter API avec header X-Auth-Token
* [ ] Construire header
* [ ] Ajouter 5 KPI cards
* [ ] Construire tableau standings
* [ ] Ajouter 4 visualisations
* [ ] Ajouter footer source

---

### C. Validation

* [ ] Respect strict mono-page
* [ ] Respect plan gratuit
* [ ] Vérifier lisibilité dark mode
* [ ] Vérifier absence de fonctionnalités hors MVP
* [ ] Vérifier consommation API maîtrisée