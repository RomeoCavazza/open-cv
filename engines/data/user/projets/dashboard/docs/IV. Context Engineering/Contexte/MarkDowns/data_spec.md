# Vue d'ensemble des endpoints utilisés

**Base URL :** "https://api.football-data.org/v4"
**Compétition :** FL1 (Ligue 1)
**Authentification :** header "X-Auth-Token: {API_KEY}" obligatoire sur tous les appels.

| Endpoint | Rôle fonctionnel |
|---|---|
| "/v4/competitions/FL1" | Métadonnées de la compétition (nom, code, zone, saison courante, matchday courant) |
| "/v4/competitions/FL1/standings" | Classement général : positions, points, bilan V/N/D, buts marqués/encaissés |
| "/v4/competitions/FL1/matches" | Liste des matchs : dates, équipes, scores, statut, journée |
| "/v4/competitions/FL1/teams" | Liste des équipes engagées (noms, identifiants, crests) |
| "/v4/competitions/FL1/scorers" | Meilleurs buteurs de la compétition *(optionnel, hors MVP actuel)* |

**Contraintes plan gratuit :**
- 10 requêtes/minute maximum
- Pas de données live ("LIVE" / "IN_PLAY" non utilisables)
- Pas de lineups, pas d'événements détaillés (buts individuels, cartons)


# Structure logique des datasets nécessaires

Sans inventer de champs — uniquement des groupes logiques à partir des endpoints.

### Dataset "CompetitionMeta"
Source : "/v4/competitions/FL1"
Filtres disponibles : aucun
Données attendues (à confirmer) :
- Nom de la compétition
- Code (FL1)
- Saison en cours (année de début)
- Matchday courant

### Dataset "Standings"
Source : "/v4/competitions/FL1/standings"
Filtres disponibles : "matchday={MATCHDAY}", "season={YEAR}"
Données attendues par équipe (à confirmer) :
- Position au classement
- Nom de l'équipe
- Matchs joués
- Victoires / Nuls / Défaites
- Buts marqués / encaissés / différence
- Points

## Dataset "MatchesAll"
Source : "/v4/competitions/FL1/matches"
Filtres disponibles : "matchday={MATCHDAY}", "status={STATUS}", "dateFrom", "dateTo", "season={YEAR}"
Statuts possibles (doc officielle) : "SCHEDULED | LIVE | IN_PLAY | PAUSED | FINISHED | POSTPONED | SUSPENDED | CANCELLED"
Données attendues par match (à confirmer) :
- Journée (matchday)
- Équipe domicile / visiteur
- Score final (domicile / extérieur)
- Statut du match
- Date

> ⚠️ Pagination par défaut : "limit=10" selon la doc. Prévoir un filtre "season={YEAR}" pour récupérer tous les matchs de la saison.

## Dataset "Teams" *(optionnel)*
Source : "/v4/competitions/FL1/teams"
Filtres disponibles : "season={YEAR}"
Utile pour : nombre total d'équipes, noms officiels, crests

## Dataset "Scorers" *(optionnel, hors MVP actuel)*
Source : "/v4/competitions/FL1/scorers"
Filtres disponibles : "limit={LIMIT}", "season={YEAR}"
Utile pour : section top buteurs si activée ultérieurement


# Mapping dataset → composants du dashboard

| Composant dashboard | Dataset source | Logique |
|---|---|---|
| Header — Nom compétition | "CompetitionMeta" | Nom + saison courante |
| KPI — Nombre d'équipes | "Teams" | "count(Teams)" |
| KPI — Matchs joués | "MatchesAll" | "count(status=FINISHED)" |
| KPI — Total buts marqués | "MatchesAll" | "sum(buts_domicile + buts_extérieur)" sur matchs FINISHED |
| KPI — Moyenne buts/match | "MatchesAll" | "total_buts / matchs_joués" |
| KPI — Journée en cours | "CompetitionMeta" | Matchday courant |
| Tableau classement | "Standings" | Position, équipe, pts, V/N/D, diff |
| Bar chart — Top 5 points | "Standings" | Top 5 par points (tri DESC) |
| Bar chart — Meilleures attaques | "Standings" | Top 5 par buts marqués (tri DESC) |
| Bar chart — Meilleures défenses | "Standings" | Top 5 par buts encaissés (tri ASC) |
| Histogramme — Buts par journée | "MatchesAll" | "sum(buts)" groupé par "matchday" |


# Agrégations simples nécessaires

Toutes compatibles Antigravity (count, sum, average, tri, group).

- **Nombre d'équipes** — "count(Teams)".
- **Matchs joués** — filtrer "MatchesAll" sur "status=FINISHED", puis "count".
- **Total buts** — sur matchs FINISHED, "sum(buts_domicile + buts_extérieur)".
- **Moyenne buts/match** — "total_buts / matchs_joués".
- **Buts par journée** — grouper "MatchesAll" par "matchday", puis "sum(buts)" par groupe.
- **Top 5 attaques** — à partir de "Standings", trier par buts marqués DESC, garder 5.
- **Top 5 défenses** — à partir de "Standings", trier par buts encaissés ASC, garder 5.
- **Top 5 points** — à partir de "Standings", trier par points DESC, garder 5.

> Les noms exacts des champs (buts_domicile, buts_extérieur, etc.) seront confirmés après inspection réelle des réponses JSON.


# Stratégie d'optimisation des appels API

**Contrainte :** 10 calls/minute (plan gratuit).

| Type | Endpoints | Calls |
|---|---|---|
| Obligatoires | "CompetitionMeta" + "Standings" + "MatchesAll" | **3** |
| Optionnels | "Teams" + "Scorers" | 0 à 2 |
| Total max | — | **5** (largement sous la limite) |

**Règles :**
- Charger les données **une seule fois** au démarrage (cold start).
- **Pas de polling** automatique (données non live).
- "Teams" peut être évité si le count d'équipes est dérivable de "Standings".
- "Scorers" uniquement si un composant buteurs est présent dans le dashboard.


# Collections Antigravity

| Nom collection | Endpoint source | Rôle |
|---|---|---|
| "competition_meta" | "/v4/competitions/FL1" | Header + matchday courant |
| "standings_fl1" | "/v4/competitions/FL1/standings" | Classement + bar charts attaque/défense/points |
| "matches_fl1" | "/v4/competitions/FL1/matches" | KPIs matchs/buts + histogramme journée |
| "teams_fl1" *(optionnel)* | "/v4/competitions/FL1/teams" | KPI nombre d'équipes |
| "scorers_fl1" *(optionnel)* | "/v4/competitions/FL1/scorers" | Top buteurs (hors MVP) |

Chaque collection = 1 datasource Antigravity avec header "X-Auth-Token".


# Flux de chargement recommandé

"""
1. competition_meta   →  Header visible immédiatement (nom, saison, journée)
2. standings_fl1      →  Tableau classement + 3 bar charts disponibles
3. matches_fl1        →  KPIs matchs/buts/moyenne + histogramme buts/journée
4. teams_fl1 (opt.)   →  KPI nombre d'équipes
"""

**Logique :** les composants visuellement prioritaires (header, classement) sont alimentés dès les 2 premiers appels. Les KPIs et graphiques issus de "matches_fl1" suivent au 3e appel.


# Points de validation après inspection réelle des réponses JSON

Via Postman ("03_DATA/postman/postman_collection.customization") :

- [x] Confirmer le nom exact du champ "points" dans "standings" → **"points"** ✓ (standings_FL1.json)
- [x] Confirmer les champs V/N/D ("won" / "draw" / "lost" ou autre convention) → **"won" / "draw" / "lost"**
- [x] Confirmer les champs "buts marqués" et "buts encaissés" dans "standings" → **"goalsFor" / "goalsAgainst"**
- [ ] Confirmer la structure du score dans "matches" ("fullTime", "halfTime" ?) → ⚠️ matches_FL1.json contient des données BL1 — à refaire avec FL1
- [ ] Confirmer le nom du champ "matchday" dans "matches" → ⚠️ idem, à refaire avec FL1
- [x] Vérifier si le matchday courant est disponible dans "competition_meta" directement → **"currentSeason.currentMatchday"** = 22
- [ ] Vérifier la pagination sur "/matches" (limit=10 par défaut → ajouter "season=" pour tout récupérer) → ⚠️ à vérifier sur le vrai appel FL1
- [x] Vérifier si "Teams" est nécessaire ou si le count d'équipes est dans "standings" → **"count: 18"** dans teams_FL1.json ; dérivable aussi via "standings[0].table.length"


# Checklists opérationnelles data

## A. Setup
- [x] Clé API générée sur football-data.org
- [x] Variable "url = https://api.football-data.org/" configurée dans Postman
- [x] Header "X-Auth-Token" ajouté à la collection Postman
- [x] Test d'un appel simple vers "/v4/competitions/FL1" pour valider la connexion

## B. Inspection des réponses JSON
- [x] Appeler "/v4/competitions/FL1" → "competition_FL1.json" ✓
- [x] Appeler "/v4/competitions/FL1/standings" → "standings_FL1.json" 
- [ ] Appeler "/v4/competitions/FL1/matches" → ⚠️ "matches_FL1.json" contient BL1 — refaire avec "FL1" dans l'URL Postman
- [x] Appeler "/v4/competitions/FL1/teams" → "teams_FL1.json" — "count: 18"

## C. Configuration des agrégations dans Antigravity
- [ ] "count" sur "teams_fl1" pour le KPI nombre d'équipes
- [ ] "count(status=FINISHED)" sur "matches_fl1" pour matchs joués
- [ ] "sum(buts_domicile + buts_extérieur)" sur matchs FINISHED
- [ ] Calcul de la moyenne buts/match
- [ ] "group by matchday" + "sum(buts)" pour l'histogramme
- [ ] "sort DESC" + "limit 5" sur buts marqués (bar chart attaques)
- [ ] "sort ASC" + "limit 5" sur buts encaissés (bar chart défenses)
- [ ] "sort DESC" + "limit 5" sur points (bar chart Top 5 points)

## D. Validation finale
- [ ] Nombre d'appels au chargement ≤ 5
- [ ] Toutes les agrégations réalisables avec les champs réels
- [ ] Cohérence entre "standings_fl1", "matches_fl1" et "competition_meta"
- [ ] Dashboard mono-page entièrement alimenté (voir "projet.md")
