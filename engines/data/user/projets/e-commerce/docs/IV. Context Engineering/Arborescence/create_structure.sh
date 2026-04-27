#!/bin/bash

# Script pour créer l'arborescence complète du projet DRYVIA e-commerce
# Exécuter avec: bash create_dryvia_structure.sh

set -e  # Arrêter en cas d'erreur

echo " Création de l'arborescence du projet DRYVIA e-commerce..."

# Créer la structure racine (backend, frontend, docs)
mkdir -p backend frontend docs

echo " Dossiers racines créés"

# ==================== BACKEND ====================
echo " Création de la structure backend..."

# Créer les dossiers backend
mkdir -p backend/{config,controllers,middleware,models,routes,services,utils}

# Créer les fichiers backend
touch backend/app.ts
touch backend/config/{db.config.ts,env.config.ts}
touch backend/controllers/product.controller.ts
touch backend/middleware/{auth.middleware.ts,error.middleware.ts}
touch backend/models/{product.model.ts,user.model.ts}
touch backend/package.json
touch backend/routes/{auth.routes.ts,products.routes.ts}
touch backend/server.ts
touch backend/services/product.service.ts
touch backend/tsconfig.json
touch backend/utils/logger.ts

echo "✅ Backend structure créée"

# ==================== DOCS ====================
echo " Création de la structure docs..."
# Le dossier docs est laissé vide pour l'instant
echo "✅ Documentation (vide) créée"

# ==================== FRONTEND ====================
echo " Création de la structure frontend..."

# Créer les dossiers frontend
mkdir -p frontend/public
mkdir -p frontend/src/{app,components/{layout,ui},features/products,lib,providers,types}
mkdir -p frontend/src/app/{cart,shop/[slug]}

# Créer les fichiers de configuration frontend
touch frontend/next.config.ts
touch frontend/next-env.d.ts
touch frontend/package.json
touch frontend/postcss.config.js

echo " Création du dossier public (vide)..."
# Le dossier public est laissé vide pour l'instant

echo "️  Création des fichiers app..."

# Créer les fichiers app
touch frontend/src/app/globals.css
touch frontend/src/app/layout.tsx
touch frontend/src/app/page.tsx
touch frontend/src/app/cart/page.tsx
touch frontend/src/app/shop/page.tsx
touch frontend/src/app/shop/[slug]/page.tsx

echo " Création des composants..."

# Créer les composants
touch frontend/src/components/layout/{Footer.tsx,Header.tsx}
touch frontend/src/components/ProductCard.tsx
touch frontend/src/components/ui/Button.tsx

# Créer les features
touch frontend/src/features/products/{ProductList.tsx,useProducts.ts}

echo " Création des utilitaires..."

# Créer lib, providers, types
touch frontend/src/lib/utils.ts
touch frontend/src/providers/CartProvider.tsx
touch frontend/src/types/index.ts

# Fichiers de configuration supplémentaires
touch frontend/tailwind.config.ts
touch frontend/tsconfig.json


# ==================== FINALISATION ====================
echo "✨ Structure complète créée avec succès !"
echo ""
echo " Résumé de la structure créée :"

# Afficher l'arborescence
echo ""
echo " Arborescence complète :"
echo ""
# Check if tree is installed, otherwise list
if command -v tree &> /dev/null; then
    tree -L 3 -I 'node_modules|.git'
else
    find . -maxdepth 3 -not -path '*/.*'
fi

echo ""
echo "✅ Projet DRYVIA prêt !"
echo " Pour initialiser git : git init && git add . && git commit -m 'Initial commit'"
