# plan.md — Plan d’implémentation Ligue 1 Dashboard

Ce document est le **plan d’exécution** pour construire le dashboard en local (Node + mock) ou avec l’API réelle. Il sert de checklist pour les étudiants et de référence pour l’agent de vibecoding.

---

## Flux global (Mermaid)

```mermaid
flowchart LR
    A[Pré-requis] --> B[Config Antigravity]
    B --> C[Collections]
    C --> D[Composants UI]
    D --> E[Mapping data → UI]
    E --> F[Stratégie refresh]
    F --> G[Build steps]
    G --> H[Tests]
```

---

## A. Pré-requis
- **Antigravity** access active.
- **Node.js** installed locally.
- **API Key** for football-data.org (optional for local mock, required for prod).
- Local JSON samples available in `mock/` folder (copied from `docs/postman/samples/`).

## B. Configuration Antigravity (datasource + headers)
- **Datasource Name**: `FootballData_API`
- **Base URL**: `https://api.football-data.org/v4`
- **Auth**: Header `X-Auth-Token: <API_KEY>` (stored in secure variable).
- **Headers**:
  - `Content-Type: application/json`

## C. Collections (liste + endpoints)
1. **Competition**: `GET /competitions/FL1`
   - Cache: 24h (static info).
2. **Standings**: `GET /competitions/FL1/standings`
   - Cache: 1h (updates after matches).
3. **Matches**: `GET /competitions/FL1/matches`
   - Cache: 15min (live scores if available, otherwise 1h).
4. **Teams**: `GET /competitions/FL1/teams`
   - Cache: 24h (rosters don't change often).

## D. Composants UI (header, KPI, table standings, charts)
- **Layout**: CSS Grid/Flexbox dark mode.
- **Header**: Logo Ligue 1, Current Season, Matchday.
- **KPI Cards**:
  - Current Matchday
  - Total Matches Played (calculated)
  - Active Teams
- **Main Content**:
  - **Left Col (2/3)**: Standings Table (Rank, Logo, Team, Pts, P, W, D, L, GF, GA, Diff).
  - **Right Col (1/3)**:
    - Top 5 Chart (Points).
    - Next Matches List (Upcoming).

## E. Mapping data -> UI (sans champs exacts, à valider)
*Note: Fields marked with (*) confirmed via local JSON samples.*

- **Competition**: `name`*, `emblem`*, `currentSeason.currentMatchday`*.
- **Standings**: `standings[0].table[]`*
  - `position`*
  - `team.name`*, `team.crest`*
  - `playedGames`*, `won`*, `draw`*, `lost`*, `points`*
  - `goalsFor`*, `goalsAgainst`*, `goalDifference`*
- **Matches**: `matches[]`*
  - `homeTeam.name`*, `awayTeam.name`*
  - `score.fullTime.home`*, `score.fullTime.away`* (check for nulls)
  - `utcDate`* (format DD/MM/YYYY HH:mm)
  - `status`* (SCHEDULED, TIMED, FINISHED, IN_PLAY)

## F. Stratégie refresh/quota
- **Dev/Local**: Usage of local mocks (`server.js`) to avoid hitting API limits 10 calls/min.
- **Prod**:
  - **Competition/Teams**: Manual refresh or Daily.
  - **Standings**: Hourly or User Triggered.
  - **Matches**: On load (cached 5 min).

## G. Build steps (ordre exact de construction)
1. Setup local Node.js mock server with provided JSON samples.
2. Build HTML structure (Header, Grid Layout).
3. Implement CSS (Dark Theme, Cards, Neon Accents).
4. Implement JS Fetch logic hitting `http://localhost:3000/mock/*`.
5. Render Data:
   - Header info.
   - KPI Cards.
   - Standings Table.
   - Upcoming Matches (filter `status: TIMED/SCHEDULED`).

## H. Plan de tests (checklist)
- [ ] Server starts on port 3000 without error.
- [ ] Endpoints `/mock/competition`, `/mock/standings`, etc. return valid JSON.
- [ ] Frontend loads without JS errors.
- [ ] Standings table displays 18 teams (Ligue 1 limit).
- [ ] Images (emblems/crests) load correctly (external URLs in JSON).
- [ ] Responsive check (mobile/desktop).

## I. Pack local Node (structure fichiers + commandes)
- **Structure**:
  - `server.js`
  - `mock/` (JSON files)
  - `public/`
    - `index.html`
    - `style.css`
    - `app.js`
- **Commandes**:
  - Install: `npm init -y` && `npm install express cors`
  - Run: `node server.js`
  - Open: `http://localhost:3000`
