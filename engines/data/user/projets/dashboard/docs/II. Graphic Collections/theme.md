# theme.md — Design system Dashboard Ligue 1

Document **source de vérité** pour le design du dashboard mono-page. Issu de l’audit des captures FootX et du prompt canonique `prompt_design.md`. À utiliser pour configurer les tokens et composants dans Antigravity (ou en CSS/JS).

---

## Vue d’ensemble du document

```mermaid
flowchart TB
    A[1. Synthèse FootX] --> B[2. Palette HEX]
    B --> C[3. Design tokens]
    C --> D[4. Typo]
    D --> E[5. Layout]
    E --> F[6. Composants UI]
    F --> G[7. Règles globales]
    G --> H[8. Adaptation mono-page]
    H --> I[9. Checklist Antigravity]
```

---

## 1. Synthèse visuelle FootX (captures + lien)
À partir des 5 captures (footx-data.png, footx-landing.png, footx-ranking.png, footx-results.png, footx-upcoming.png) et du lien FootX Ligue 1, l’UI suit une logique **sport-tech dark** : densité d’information élevée, lecture rapide, hiérarchie très “data”.

Constats observables :
- **Dark mode dominant** : background très sombre, surfaces en gris anthracite.
- **Accents très saturés** utilisés pour signaler l’important (valeurs clés, états, highlights).
- **Composants “cards” et “rows”** : blocs rectangulaires, radius faible à moyen, séparateurs fins.
- **Tableaux lisibles** : colonnes bien séparées, chiffres alignés, contraste suffisant.
- **Listes de matchs** : structure horizontale répétable (équipes/score/date), logos/crests, infos compactes.
- **Micro-signaux** : badges, pastilles, variations de couleur pour statuts / résultats.
- **Graphiques minimalistes** : peu d’ornement, lecture immédiate sur fond sombre.

Objectif : reproduire ce style dans un dashboard mono-page Antigravity (voir annexe projet.md) avec simplification (moins de variations, composants standards).

---

## 2. Palette de couleurs (HEX estimés + rôle + fallback)
> Valeurs HEX **estimées** visuellement depuis les captures + le lien FootX. À valider ensuite par sampling (pipette) si nécessaire.

| Rôle | Description | HEX estimé | Fallback |
|---|---|---:|---:|
| Background principal | fond global | `#0B0D10` | `#0A0A0A` |
| Surface 1 | cards / blocs | `#161A1F` | `#14181D` |
| Surface 2 | tableaux / sections | `#1F242B` | `#1D2228` |
| Border / separators | traits fins | `#2A2F36` | `#2C2C2C` |
| Accent primaire | highlight principal (néon/énergie) | `#00E676` | `#00C96B` |
| Accent secondaire | second highlight | `#2979FF` | `#2F6DFF` |
| Success | état positif | `#4CAF50` | `#43A047` |
| Danger | erreur / négatif | `#FF5252` | `#FF3B3B` |
| Warning | attention | `#FFEB3B` | `#FFD60A` |
| Text primary | texte principal | `#F5F7FA` | `#FFFFFF` |
| Text secondary | labels / meta | `#AEB4BE` | `#B0B0B0` |
| Text muted | infos secondaires | `#7A808A` | `#808080` |

---

## 3. Design tokens (noms + valeurs)
### Background
- `bg.primary = #0B0D10`

### Surfaces
- `surface.1 = #161A1F`
- `surface.2 = #1F242B`

### Borders
- `border.default = #2A2F36`
- `border.subtle = rgba(255,255,255,0.06)` *(optionnel si Antigravity supporte RGBA)*

### Accents
- `accent.primary = #00E676`
- `accent.secondary = #2979FF`

### Status
- `status.success = #4CAF50`
- `status.danger  = #FF5252`
- `status.warning = #FFEB3B`

### Text
- `text.primary   = #F5F7FA`
- `text.secondary = #AEB4BE`
- `text.muted     = #7A808A`

---

## 4. Typographie (échelle + règles)
> Ne pas imposer une police spécifique si Antigravity ne permet pas un choix fin : viser une sans-serif moderne et lisible, avec priorité à la **lisibilité des chiffres**.

### Échelle recommandée (dashboard dense)
- **H1** (titre page) : 28–32px — bold — `text.primary`
- **H2** (titres sections) : 18–22px — semibold — `text.primary`
- **H3** (sous-titres) : 16–18px — semibold — `text.primary`
- **Body** (texte standard) : 14–15px — regular — `text.secondary`
- **Caption / meta** (dates, labels) : 12–13px — regular — `text.muted`
- **Numbers / KPI** : 24–32px — bold — `text.primary`

### Règles
- Les **chiffres** dominent la hiérarchie (KPIs, points, scores).
- Utiliser `text.secondary` pour labels, et `text.muted` pour contexte (dates/statuts).
- Dans les tables : **texte à gauche**, **numériques à droite**.

---

## 5. Layout system (grid + responsive)
### Desktop (mono-page)
- Grille : **12 colonnes**
- Gouttières : 16px
- Marges latérales : 24–32px
- Rythme vertical : sections espacées régulièrement

### Mobile (compression)
- Passer en **stack** :
  - Header
  - KPIs (2 par ligne)
  - Classement (table scroll horizontale si nécessaire)
  - Graphiques empilés

### Spacing scale
- `xs = 4`
- `s  = 8`
- `m  = 16`
- `l  = 24`
- `xl = 32`

---

## 6. Composants UI (spécifications Antigravity)

### KPI Cards
- Background : `surface.1`
- Border : `1px solid border.default`
- Radius : 8px
- Padding : 16–20px
- Contenu :
  - Label (caption) `text.secondary`
  - Valeur (number) `text.primary`
- Accent (option) : top border 2–3px en `accent.primary` (à utiliser parcimonieusement)

### Data Tables (classement)
- Container : `surface.2`
- Header row : texte `text.primary` semibold
- Rows : alternance légère (si possible) entre `surface.2` et une nuance proche
- Separators : `border.default`
- Alignement :
  - texte (équipe) = gauche
  - chiffres (points, diff, V/N/D) = droite
- État hover (si dispo) : légère hausse de contraste, sans changer la palette

### List Items (matchs)
- Container : `surface.1`
- Layout horizontal :
  - crest A + nom A
  - score / status (centré)
  - nom B + crest B
  - date/heure en caption
- Border : `border.default`
- Radius : 8px
- Status :
  - FINISHED → neutre + score visible
  - SCHEDULED → date/heure plus visible
  - (si composant badge dispo) utiliser `status.*`

### Graphiques (bar / donut / histogram)
- Fond : transparent ou `surface.1`
- Série principale : `accent.primary`
- Série secondaire : `accent.secondary`
- Axes/labels : `text.muted`
- Règle : minimalisme (pas de décorations inutiles)

### Badges / Status
- Forme : pill ou radius 6–8px
- Texte : lisible, contraste élevé
- Mapping :
  - success = `status.success`
  - danger  = `status.danger`
  - warning = `status.warning`

### Buttons / Selectors (si nécessaires)
- Button background : `surface.2`
- Border : `border.default`
- Text : `text.primary`
- Hover : légère variation de contraste (sans changer la couleur)
- Selectors : mêmes règles que buttons, taille compacte

---

## 7. Règles visuelles globales
### Border radius
- Cards / rows : 8px
- Badges : 6–8px
- Inputs : 6px

### Borders
- Standard : `1px solid border.default`
- Séparateurs : mêmes règles, opacité faible si possible

### Shadows
- Très léger uniquement (dark UI) :
  - `0 2px 6px rgba(0,0,0,0.25)` (si Antigravity supporte)
- Si non : privilégier border + contraste de surface

### États
- Hover : variation subtile (contraste), pas d’animation lourde
- Selected : accent primaire (bordure ou underline), éviter le “full background” fluo

---

## 8. Adaptation spécifique mono-page
- Priorité : **scan vertical rapide** (header → KPIs → table → graphs).
- La **table de classement** est le bloc principal (largeur maximale).
- Les **KPIs** doivent tenir sur une ligne desktop (4–5 cards).
- Graphiques : 2 colonnes desktop (si place), sinon stack vertical.
- Réduire les variations : 2 surfaces max (`surface.1` / `surface.2`) + 2 accents.

---

## 9. Checklist Antigravity (pas à pas)
### A. Tokens
- [ ] Créer `bg.primary`, `surface.1`, `surface.2`, `border.default`
- [ ] Créer `accent.primary`, `accent.secondary`
- [ ] Créer `text.primary`, `text.secondary`, `text.muted`
- [ ] Créer `status.success`, `status.danger`, `status.warning`

### B. Styles texte
- [ ] Définir H1 / H2 / H3 / Body / Caption / Number
- [ ] Vérifier alignements tables (numériques à droite)

### C. Composants
- [ ] KPI card (1 modèle) + variantes accent (option)
- [ ] Table standings (1 modèle) + hover (option)
- [ ] Match row (1 modèle) + status (option)
- [ ] Graphs (bar/histo) avec mapping couleurs

### D. QA visuel
- [ ] Contraste OK (texte sur surfaces)
- [ ] Densité OK (mono-page lisible)
- [ ] Cohérence surfaces/borders/radius
- [ ] Pas d’élément visuel non “FootX-like”
