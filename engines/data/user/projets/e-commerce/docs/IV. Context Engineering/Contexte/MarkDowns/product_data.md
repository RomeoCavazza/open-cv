# Product Data — DRYVIA

Source **unique** des données produit pour la landing, le shop et la fiche produit. Tous les textes, prix, USP et chemins d’images doivent provenir de ce fichier (pas de Lorem Ipsum).

---

## Mapping données → UI

```mermaid
flowchart LR
    A[product_data.md] --> B[Landing]
    A --> C[Shop listing]
    A --> D[Product page]
    A --> E[Cart / Checkout]
```

| Zone UI | Données utilisées |
|---------|-------------------|
| Hero / Tagline | Tagline, sous-titre. |
| Prix | €129.00 (format cohérent). |
| USP / Features | 4 features (Anti-Transfer, Flash-Dry, Hygiene Shield, Eco-Impact). |
| Galerie | Image mapping (angle-front, side-view, sole-view, etc.). |

---

## The Flagship Model: "DRYVIA One"

[cite_start]**Tagline:** "Stay Dry. Train Hard." [cite: 72]

**Price:** €129.00

**Category:** Indoor Fitness / Cross-training

### Key Features (USP)

1. **Anti-Transfer Sole:** Sweat never reaches the floor. [cite_start]Keeps mats dry and hygienic[cite: 16].
2. [cite_start]**Flash-Dry Mesh:** Breathable fabric that evaporates moisture instantly[cite: 17].
3. [cite_start]**Hygiene Shield:** Antibacterial membrane to prevent odors[cite: 18].
4. [cite_start]**Eco-Impact:** Made from recycled materials[cite: 20].

### Description

[cite_start]The first indoor sneaker engineered to keep your feet, socks, and training mat perfectly dry[cite: 22]. Designed for HIIT, cross-training, and studio workouts where hygiene and grip are non-negotiable.

### Image Mapping (Located in /public/assets/)

- **Hero Banner:** `hero-banner.jpg` (Athlete jumping in dark gym)
- **Main Product:** `angle-front.png` (3/4 view, main shop image)
- **Side Profile:** `side-view.png` (Pure profile view)
- **Detail Tech:** `tech-mesh.jpg` (Close up on breathable fabric)
- **Sole:** `sole-view.png` (Bottom view showing green grip)
- **Lifestyle:** `gym-lifestyle.jpg` (Shoes on gym floor)