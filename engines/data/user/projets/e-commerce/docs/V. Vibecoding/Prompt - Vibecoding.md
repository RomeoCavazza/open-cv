# Plan d'implémentation complet — DRYVIA E-Commerce

**Objectif :** Livrer une landing page complète (hero vidéo en loop, grosses typographies, navbar, footer), une boutique full fonctionnelle (listing, fiche produit, panier, paiement), en utilisant **tous** les assets et un backend propre. Résultat **ultra propre** et prêt prod.

Ce document sert de **maxi prompt d’implémentation** : chaque phase peut être exécutée séquentiellement par un dev ou un agent. Référence unique pour le périmètre fonctionnel et technique.

---

## Vue d’ensemble des phases

```mermaid
flowchart LR
    P0[Phase 0 Prépa] --> P1[Phase 1 Design]
    P1 --> P2[Phase 2 Backend]
    P2 --> P3[Phase 3 Layout]
    P3 --> P4[Phase 4 Landing]
    P4 --> P5[Phase 5 Shop]
    P5 --> P6[Phase 6 Checkout]
    P6 --> P7[Phase 7 Finition]
```

| Phase | Livrable principal |
|-------|--------------------|
| 0 | Assets dans `frontend/public/assets/`, structure backend prête |
| 1 | Design system Tailwind + polices + composants UI de base |
| 2 | API REST produits + health, typée et propre |
| 3 | Navbar sticky, panier (sheet + badge), gros footer, CartProvider |
| 4 | Landing complète : hero vidéo loop, sections, footer |
| 5 | Shop listing, fiche produit avec galerie, page panier |
| 6 | Checkout, création commande backend, paiement (Stripe ou mock) + page succès |
| 7 | Lint, typage, responsive, usage de tous les assets, prêt prod |

---

## 1. Inventaire du repo (docs & assets)

### 1.1 Documentation de référence (à respecter)

| Fichier | Rôle |
|--------|------|
| `docs/IV. Context Engineering/Contexte/MarkDowns/design_system.md` | Couleurs, typo (Montserrat / Inter), boutons, cards, dark mode |
| `docs/IV. Context Engineering/Contexte/MarkDowns/product_data.md` | Tagline, prix €129, USP, description, mapping des images |
| `docs/IV. Context Engineering/Contexte/MarkDowns/project_rules.md` | Stack (Next.js App Router, React, TS, Tailwind, Express), règles front/back |
| `docs/IV. Context Engineering/Contexte/MarkDowns/branding.md` | Nom DRYVIA, taglines, ton, positionnement |
| `docs/IV. Context Engineering/Contexte/MarkDowns/context.md` | Niche, cible, problème/solution, USP |
| `docs/II. Créations graphiques/Prompts/Product DNA.md` | Spec produit (couleurs, matériaux, branding) |

### 1.2 Assets existants et attendus

**À utiliser / placer dans `frontend/public/` :**

| Asset | Emplacement actuel / attendu | Usage |
|-------|------------------------------|--------|
| **hero.mp4** | `docs/II. Créations graphiques/Assets/hero.mp4` | Hero landing : vidéo en loop, mute, autoplay, object-cover |
| **logo-light.png** | `public/assets/logo-light.png` | Navbar (fond clair ou sur fond sombre selon design) |
| **logo-dark.png** | `public/assets/logo-dark.png` | Footer ou variante dark |
| **hero-banner.jpg** | `public/assets/hero-banner.jpg` | Fallback hero si pas de vidéo, ou section secondaire |
| **angle-front.png** | `public/assets/angle-front.png` | Vue 3/4 produit — image principale shop & fiche produit |
| **side-view.png** | `public/assets/side-view.png` | Galerie fiche produit |
| **back-view.png** | `public/assets/back-view.png` | Galerie fiche produit |
| **sole-view.png** | `public/assets/sole-view.png` | Galerie fiche produit (semelle) |
| **tech-mesh.jpg** | `public/assets/tech-mesh.jpg` | Détail technique / zoom matière |
| **gym-lifestyle.jpg** | `public/assets/gym-lifestyle.jpg` | Section lifestyle / “In the gym” sur landing |

**Action préalable :** Copier `docs/II. Créations graphiques/Assets/hero.mp4` vers `frontend/public/assets/hero.mp4`. Créer `frontend/public/assets/` et y placer tous les assets listés (générer ou fournir les images manquantes selon les prompts dans `docs/II. Créations graphiques/Prompts/`).

### 1.3 Design system (rappel)

- **Couleurs :** Deep Black `#0E0E11`, Neon Green `#00F2A6`, Pure White `#FFFFFF`, Steel Gray `#8A8F98`, Fresh Blue `#2FD2FF`.
- **Typo :** Montserrat (headings), Inter (body). Grosses écritures sur hero et titres de section.
- **UI :** Boutons primary (neon green, texte noir), secondary (bordure blanche, transparent). Cards sombres, bordure steel gray, hover avec glow neon green.

---

## 2. Phases d’implémentation

---

### Phase 0 — Préparation (assets & structure)

**Objectif :** Avoir tous les assets au bon endroit et une structure de dossiers propre.

- Créer `frontend/public/assets/`.
- Copier `docs/II. Créations graphiques/Assets/hero.mp4` → `frontend/public/assets/hero.mp4`.
- S’assurer que les assets suivants existent dans `frontend/public/assets/` :  
  `logo-light.png`, `logo-dark.png`, `hero-banner.jpg`, `angle-front.png`, `side-view.png`, `back-view.png`, `sole-view.png`, `tech-mesh.jpg`, `gym-lifestyle.jpg`.  
  (Si un fichier manque : utiliser un placeholder ou une image de la même taille pour ne pas casser le layout.)
- Backend : garder `server.ts` à la racine de `backend/` ou migrer vers une structure `app.ts` + `server.ts` selon `Arborescence.md`. Créer les dossiers `config`, `controllers`, `models`, `routes`, `services`, `middleware`, `utils` vides si pas encore présents.

**Livrable :** Arborescence `frontend/public/assets/` complète, backend prêt à recevoir les routes.

---

### Phase 1 — Design system & fondations frontend

**Objectif :** Tailwind aligné sur la charte, polices, variables CSS, composants UI de base.

- **Tailwind (`frontend/tailwind.config.ts`)**  
  - Couleurs : `brand-black` (#0E0E11), `brand-green` (#00F2A6), `brand-white` (#FFFFFF), `brand-gray` (#8A8F98), `brand-blue` (#2FD2FF).  
  - Mapper `primary` sur la couleur neon green.
- **Polices :** Montserrat (headings) et Inter (body) via `next/font/google` dans `layout.tsx`.
- **`globals.css`** : variables CSS pour les couleurs ci-dessus, background par défaut dark (#0E0E11).
- **Composants UI (ex. `frontend/src/components/ui/`)** :  
  `Button` (variants primary / secondary), `Card`, `Sheet` (pour panier latéral), `Badge`, `Separator`.  
  Utiliser les tokens du design system (pas de couleurs en dur).
- **Utilitaire :** `frontend/src/lib/utils.ts` (ex. `cn()` avec `clsx` + `tailwind-merge`).

**Livrable :** Design system appliqué, boutons/cards/sheet utilisables partout.

---

### Phase 2 — Backend (API produits, health, structure)

**Objectif :** API REST propre, type-safe, prête pour produits et plus tard commandes/paiement.

- **Structure :**  
  - `backend/config/db.config.ts` et `env.config.ts` (PORT, MONGODB_URI si DB).  
  - `backend/models/product.model.ts` : schéma Mongoose (ou type TypeScript seul si pas de DB) avec au minimum : name, slug, price, description, features[], imageUrls[], category.  
  - `backend/services/product.service.ts` : lecture des produits (depuis DB ou, en phase 2, depuis un JSON/seed).  
  - `backend/controllers/product.controller.ts` : GET liste, GET by slug.  
  - `backend/routes/products.routes.ts` : `/api/products`, `/api/products/:slug`.  
  - `backend/middleware/error.middleware.ts` : gestion centralisée des erreurs (404, 500).  
  - `backend/server.ts` (ou `app.ts`) : `express()`, `cors()`, `express.json()`, montage des routes, health `GET /api/health`.
- **Données initiales :** Au moins 1 produit “DRYVIA One” (données de `product_data.md`), avec slugs et chemins d’images pointant vers `/assets/angle-front.png`, etc. Si pas de DB : seed en mémoire ou fichier JSON chargé au démarrage.
- **Typage :** Interfaces TypeScript partagées pour Product (idéalement dans `backend/types` ou `models`).

**Livrable :** `GET /api/health` et `GET /api/products`, `GET /api/products/:slug` fonctionnels, code propre et typé.

---

### Phase 3 — Layout global : Navbar + Footer + Providers

**Objectif :** Navbar sticky, panier (icône + sheet), footer gros et complet, CartProvider disponible partout.

- **Header / Navbar (`frontend/src/components/layout/Header.tsx`)**  
  - Sticky, fond semi-transparent + backdrop blur (glassmorphism).  
  - Gauche : logo `logo-light.png` (lien vers `/`), hauteur ~32–40px.  
  - Centre : liens “Home”, “Shop” (`/`, `/shop`).  
  - Droite : icône panier (Lucide `ShoppingBag`) avec badge (nombre d’articles), au clic ouvre le Sheet panier.  
  - Mobile : menu burger si besoin, même logique panier.
- **CartProvider (`frontend/src/providers/CartProvider.tsx`)**  
  - Context avec : `items`, `addItem`, `removeItem`, `updateQuantity`, `clearCart`, `getTotal`, `getItemCount`.  
  - Persistance `localStorage` (clé type `dryvia-cart`).  
  - Type d’item : `{ id, name, slug, price, quantity, imageUrl }` (aligné avec le produit backend).
- **Sheet Panier (sidebar)**  
  - Ouverture depuis la navbar.  
  - Liste des lignes (image, nom, prix, quantité +/-), total, bouton “Voir le panier” → `/cart`, “Continuer mes achats” (ferme le sheet).
- **Footer (`frontend/src/components/layout/Footer.tsx`)**  
  - “Gros” footer : plusieurs colonnes (ex. Produit, Société, Légal, Contact / Newsletter).  
  - Utiliser le contenu de `branding.md` et `product_data.md` (taglines, liens factices ou “À propos”, “CGV”, “Mentions légales”, “Contact”).  
  - Logo ou nom DRYVIA, couleurs design system, liens vers Home / Shop.  
  - Optionnel : champ newsletter (sans backend pour l’instant).
- **Root layout**  
  - Intégrer `CartProvider`, `Header`, puis `children`, puis `Footer`.  
  - Metadata (title, description) depuis `branding.md` / `product_data.md`.

**Livrable :** Navigation cohérente, panier utilisable partout, footer complet.

---

### Phase 4 — Landing page complète (hero vidéo + sections + footer)

**Objectif :** Une seule page d’accueil percutante avec hero en vidéo loop, grosses écritures, toutes les sections et tous les assets utilisés.

- **Hero (full viewport)**  
  - Vidéo : `<video>` avec `src="/assets/hero.mp4"` (ou chemin Next `public/assets/hero.mp4`), `autoPlay`, `muted`, `loop`, `playsInline`, `className="object-cover w-full h-full"` dans un conteneur en position absolute/fixed couvrant tout l’écran.  
  - Overlay sombre (ex. `bg-black/50` ou `bg-brand-black/60`) pour lisibilité du texte.  
  - Contenu centré :  
    - Titre principal : tagline “Stay Dry. Train Hard.” en **très gros** (Montserrat, bold, white).  
    - Sous-titre : “The first indoor anti-sweat sneaker.” (gray).  
    - CTA : bouton “Shop Collection” (primary) → lien vers `/shop`.  
  - Fallback : si pas de vidéo, utiliser `hero-banner.jpg` en `next/image` avec même overlay et texte.
- **Sections suivantes (réutiliser les assets)**  
  - **USP / Features :** 4 blocs (Anti-Transfer Sole, Flash-Dry Mesh, Hygiene Shield, Eco-Impact) avec icônes ou petites images si dispo ; texte de `product_data.md`.  
  - **Produit phare :** une section mettant en avant “DRYVIA One” avec `angle-front.png` (ou galerie), prix €129, court texte, CTA “Découvrir” → `/shop/dryvia-one` (ou slug choisi).  
  - **Lifestyle :** section avec `gym-lifestyle.jpg` (full width ou grande card), titre + court texte “Designed for the gym.” (ou équivalent).  
  - **Autre section optionnelle :** “Tech” avec `tech-mesh.jpg` pour montrer la matière.
- **Footer**  
  - Déjà en place en Phase 3 ; s’assurer qu’il est bien visible en bas de la landing (pas de double footer).

**Livrable :** Landing complète, hero en vidéo loop, grosses typographies, tous les assets utilisés, footer cohérent.

---

### Phase 5 — Boutique : listing, fiche produit, panier page

**Objectif :** Shop fonctionnel avec données venant du backend (ou du fetch côté client vers l’API).

- **Listing (`frontend/src/app/shop/page.tsx`)**  
  - Fetch `GET /api/products` (côté client ou Server Component avec fetch).  
  - Grille responsive (1 col mobile, 2–3 cols desktop) de cartes produit.  
  - Composant `ProductCard` : image (`angle-front.png` ou première image du produit), nom, prix €129, bouton “Voir” ou “Ajouter au panier” (ou les deux). Lien vers `/shop/[slug]`.
- **Fiche produit (`frontend/src/app/shop/[slug]/page.tsx`)**  
  - Fetch `GET /api/products/:slug`.  
  - Layout 2 colonnes :  
    - Gauche : galerie (image principale + miniatures : angle-front, side-view, back-view, sole-view, tech-mesh). Clic miniature = changer l’image principale.  
    - Droite : titre, prix, description (product_data.md), liste des features (USP), sélecteur quantité, bouton “Add to Cart” qui appelle `addItem` du CartProvider et peut ouvrir le Sheet panier.  
  - Utiliser `next/image` pour toutes les images, chemins depuis l’API ou depuis `/assets/...`.
- **Page Panier (`frontend/src/app/cart/page.tsx`)**  
  - Afficher les lignes du panier (image, nom, prix unitaire, quantité avec +/-), sous-total par ligne, total global.  
  - Bouton “Passer la commande” → `/checkout` (Phase 6).  
  - Lien “Continuer mes achats” → `/shop`.

**Livrable :** Parcours complet Shop → Fiche produit → Ajout au panier → Page panier.

---

### Phase 6 — Checkout & paiement

**Objectif :** Tunnel de commande simple et paiement (réel ou mock).

- **Page Checkout (`frontend/src/app/checkout/page.tsx`)**  
  - Formulaire : email, prénom, nom, adresse (livraison), optionnel facturation identique.  
  - Récap panier (liste + total).  
  - Bouton “Payer” qui envoie les infos au backend (création commande) et redirige vers paiement.
- **Backend**  
  - Route `POST /api/orders` : accepte payload (items[], total, email, adresse, etc.), enregistre en base ou en JSON pour démo.  
  - Option Stripe : créer une session Stripe et renvoyer l’URL de paiement ; frontend redirige vers Stripe puis page “Merci” après succès.  
  - Si pas Stripe : flow “mock” (afficher une page “Commande reçue” après POST, sans vraie prise de carte).
- **Page remerciement**  
  - `frontend/src/app/checkout/success/page.tsx` : message de confirmation, numéro de commande si fourni, lien vers Home / Shop.  
  - Après succès : vider le panier (localStorage + context).

**Livrable :** Utilisateur peut “passer commande”, voir une confirmation, et (optionnel) payer via Stripe.

---

### Phase 7 — Finition & qualité

**Objectif :** Code propre, responsive, accessible, sans régression.

- Linting (ESLint) et typage (TypeScript) sans erreur.  
- Vérifier la responsivité (mobile first) sur toutes les pages.  
- Toutes les images via `next/image` avec `alt` pertinent.  
- Vérifier que tous les assets listés en 1.2 sont bien utilisés (hero.mp4, logos, hero-banner, angle-front, side-view, back-view, sole-view, tech-mesh, gym-lifestyle).  
- Supprimer les blocs “Service Status” ou texte de debug en dur si encore présents.  
- Optionnel : tests E2E (Playwright/Cypress) sur parcours critique (home → shop → produit → panier → checkout).

**Livrable :** Projet prêt pour démo / mise en ligne.

---

## 3. Récapitulatif des livrables par phase

| Phase | Livrable principal |
|-------|---------------------|
| 0 | Assets dans `frontend/public/assets/`, structure backend prête |
| 1 | Design system Tailwind + polices + composants UI de base |
| 2 | API REST produits + health, typée et propre |
| 3 | Navbar sticky, panier (sheet + badge), gros footer, CartProvider |
| 4 | Landing complète : hero vidéo loop, grosses écritures, sections, footer |
| 5 | Shop listing, fiche produit avec galerie, page panier |
| 6 | Checkout (formulaire + récap), création commande backend, paiement (Stripe ou mock) + page succès |
| 7 | Lint, typage, responsive, usage de tous les assets, prêt prod |

---

## 4. Références rapides

- **Tagline / texte hero :** “Stay Dry. Train Hard.” + “The first indoor anti-sweat sneaker.”
- **Prix :** €129.00
- **Produit :** DRYVIA One — Indoor Fitness / Cross-training
- **Chemins assets frontend :** `/assets/hero.mp4`, `/assets/logo-light.png`, `/assets/angle-front.png`, etc. (dans `public/assets/`).
- **Backend base URL (dev) :** `http://localhost:5000` — CORS autoriser `http://localhost:3000`.

Tu peux exécuter les phases dans l’ordre et utiliser ce fichier comme spec unique pour implémenter **tout** : assets, backend, navbar, panier, hero vidéo loop, landing complète avec gros footer, et boutique full fonctionnelle avec fiche produit, panier et paiement.

---

## 5. Vibecoding & Déploiement

Voici la séquence finale de vibecoding et de mise en ligne :

### 5.1 Vibecoding Session
![Live Coding](live-coding.gif)

### 5.2 Git Push
![Git Push](git-push.png)

### 5.3 Vercel Deployment
![Vercel Deploy](vercel-deploy.png)

### 5.4 Résultat Final
![Final Result](final-result.gif)
