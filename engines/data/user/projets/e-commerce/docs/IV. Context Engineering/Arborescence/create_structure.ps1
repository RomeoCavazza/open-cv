# Script pour créer l'arborescence complète du projet DRYVIA e-commerce
# Exécuter avec: PowerShell -ExecutionPolicy Bypass -File create_dryvia_structure.ps1

$ErrorActionPreference = 'Stop'

Write-Host "Creation de l'arborescence du projet DRYVIA e-commerce..." -ForegroundColor Cyan

# Définir le dossier racine (le dossier courant)
$root = "."

# Créer la structure racine
New-Item -ItemType Directory -Force -Path "$root/backend", "$root/frontend", "$root/docs" | Out-Null
Write-Host "Structure racine creee" -ForegroundColor Green

# ==================== BACKEND ====================
Write-Host "Creation de la structure backend..." -ForegroundColor Yellow

# Créer les dossiers backend
$backendDirs = @(
    "config", "controllers", "middleware", "models", 
    "routes", "services", "utils"
)
foreach ($dir in $backendDirs) {
    New-Item -ItemType Directory -Force -Path "$root/backend/$dir" | Out-Null
}

# Créer les fichiers backend
$backendFiles = @(
    "app.ts",
    "config/db.config.ts", "config/env.config.ts",
    "controllers/product.controller.ts",
    "middleware/auth.middleware.ts", "middleware/error.middleware.ts",
    "models/product.model.ts", "models/user.model.ts",
    "package.json",
    "routes/auth.routes.ts", "routes/products.routes.ts",
    "server.ts",
    "services/product.service.ts",
    "tsconfig.json",
    "utils/logger.ts"
)
foreach ($file in $backendFiles) {
    New-Item -ItemType File -Force -Path "$root/backend/$file" | Out-Null
}

Write-Host "Backend structure creee" -ForegroundColor Green

# ==================== DOCS ====================
Write-Host "Creation de la structure docs (vide)..." -ForegroundColor Yellow
# Le dossier docs est laissé vide

Write-Host "Documentation (vide) creee" -ForegroundColor Green

# ==================== FRONTEND ====================
Write-Host "Creation de la structure frontend..." -ForegroundColor Yellow

# Créer les dossiers frontend
$frontendDirs = @(
    "public",
    "src/app", "src/components/layout", "src/components/ui",
    "src/features/products", "src/lib", "src/providers", "src/types",
    "src/app/cart", "src/app/shop/[slug]"
)
foreach ($dir in $frontendDirs) {
    New-Item -ItemType Directory -Force -Path "$root/frontend/$dir" | Out-Null
}

# Créer les fichiers de configuration frontend
$frontendConfigFiles = @(
    "next.config.ts", "next-env.d.ts", "package.json", "postcss.config.js",
    "tailwind.config.ts", "tsconfig.json"
)
foreach ($file in $frontendConfigFiles) {
    New-Item -ItemType File -Force -Path "$root/frontend/$file" | Out-Null
}

Write-Host "Creation du dossier public (vide)..." -ForegroundColor Yellow
# Le dossier public est laissé vide

Write-Host "Creation des fichiers app..." -ForegroundColor Yellow

# Créer les fichiers app
$appFiles = @(
    "src/app/globals.css", "src/app/layout.tsx", "src/app/page.tsx",
    "src/app/cart/page.tsx", "src/app/shop/page.tsx", 
    "src/app/shop/[slug]/page.tsx"
)
foreach ($file in $appFiles) {
    New-Item -ItemType File -Force -Path "$root/frontend/$file" | Out-Null
}

Write-Host "Creation des composants..." -ForegroundColor Yellow

# Créer les composants
$componentFiles = @(
    "src/components/layout/Footer.tsx", "src/components/layout/Header.tsx",
    "src/components/ProductCard.tsx", "src/components/ui/Button.tsx",
    "src/features/products/ProductList.tsx", "src/features/products/useProducts.ts"
)
foreach ($file in $componentFiles) {
    New-Item -ItemType File -Force -Path "$root/frontend/$file" | Out-Null
}

Write-Host "Creation des utilitaires..." -ForegroundColor Yellow

# Créer lib, providers, types
$utilFiles = @(
    "src/lib/utils.ts", "src/providers/CartProvider.tsx", "src/types/index.ts"
)
foreach ($file in $utilFiles) {
    New-Item -ItemType File -Force -Path "$root/frontend/$file" | Out-Null
}


# ==================== FINALISATION ====================
Write-Host "Structure complete creee avec succes !" -ForegroundColor Cyan
Write-Host ""
Write-Host "Resume de la structure creee :" -ForegroundColor Cyan

# Afficher l'arborescence
Write-Host ""
Write-Host "Arborescence complete :" -ForegroundColor Cyan
Write-Host ""

if (Get-Command tree -ErrorAction SilentlyContinue) {
    tree /F
} else {
    Get-ChildItem -Recurse | Select-Object FullName
}

Write-Host ""
Write-Host "Projet DRYVIA pret !" -ForegroundColor Green
Write-Host "Pour initialiser git : git init; git add .; git commit -m 'Initial commit'"
