#!/bin/bash

# Script de setup pour le système de scraping professionnel Revolver AI Bot
# Usage: ./scripts/setup_scraping.sh

set -e

echo " Setup du système de scraping professionnel Revolver AI Bot"
echo "================================================================"

# Vérifier Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 n'est pas installé"
    exit 1
fi

echo "✅ Python 3 détecté: $(python3 --version)"

# Installer les dépendances Python
echo " Installation des dépendances Python..."

pip install aiofiles
pip install aiohttp
pip install beautifulsoup4
pip install feedparser
pip install selenium
pip install webdriver-manager
pip install instaloader
pip install click
pip install google-cloud-aiplatform
pip install google-cloud-speech
pip install google-cloud-vision

echo "✅ Dépendances Python installées"

# Installer Chrome/Chromium pour Selenium
echo " Installation de Chrome/Chromium..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if ! command -v brew &> /dev/null; then
        echo " Installation de Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    
    if ! command -v google-chrome &> /dev/null; then
        echo " Installation de Google Chrome..."
        brew install --cask google-chrome
    fi
    
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if command -v apt-get &> /dev/null; then
        # Ubuntu/Debian
        sudo apt-get update
        sudo apt-get install -y chromium-browser
    elif command -v yum &> /dev/null; then
        # CentOS/RHEL
        sudo yum install -y chromium
    fi
    
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    # Windows
    echo "⚠️ Sur Windows, installez Chrome manuellement depuis https://www.google.com/chrome/"
fi

echo "✅ Chrome/Chromium installé"

# Installer Tor (optionnel)
echo " Installation de Tor (optionnel)..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if ! command -v tor &> /dev/null; then
        brew install tor
    fi
    
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if command -v apt-get &> /dev/null; then
        sudo apt-get install -y tor
    elif command -v yum &> /dev/null; then
        sudo yum install -y tor
    fi
fi

echo "✅ Tor installé (si disponible)"

# Installer FFmpeg pour l'analyse vidéo
echo " Installation de FFmpeg..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if ! command -v ffmpeg &> /dev/null; then
        brew install ffmpeg
    fi
    
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if command -v apt-get &> /dev/null; then
        sudo apt-get install -y ffmpeg
    elif command -v yum &> /dev/null; then
        sudo yum install -y ffmpeg
    fi
fi

echo "✅ FFmpeg installé"

# Créer les dossiers nécessaires
echo " Création des dossiers..."

mkdir -p downloads/media
mkdir -p data/veille
mkdir -p logs
mkdir -p test_output
mkdir -p config/secrets

echo "✅ Dossiers créés"

# Copier le fichier de configuration d'exemple
if [ ! -f "config/secrets/.env" ]; then
    echo " Copie du fichier de configuration d'exemple..."
    cp config/secrets.example.env config/secrets/.env
    echo "⚠️ IMPORTANT: Configurez vos API keys dans config/secrets/.env"
else
    echo "✅ Fichier de configuration existe déjà"
fi

# Test d'installation
echo " Test d'installation..."

python3 -c "
from src.scout.intelligence.veille import UltraVeilleEngine, EnhancedScraper
print('✅ Import des modules réussi')
"

echo "✅ Test d'installation réussi"

# Instructions finales
echo ""
echo " Installation terminée avec succès!"
echo ""
echo " Prochaines étapes:"
echo "1. Configurez vos API keys dans config/secrets/.env"
echo "2. Testez le système: python test_enhanced_scraper.py"
echo "3. Utilisez le CLI: python -m src.scout.cli.main --help"
echo ""
echo " Commandes utiles:"
echo "  - Veille complète: python -m src.scout.cli.main veille-complete --competitors nike adidas"
echo "  - Instagram: python -m src.scout.cli.main instagram nike"
echo "  - TikTok: python -m src.scout.cli.main tiktok nike"
echo "  - Web: python -m src.scout.cli.main web https://example.com"
echo "  - OSINT: python -m src.scout.cli.main osint example.com"
echo ""
echo " Documentation: src/scout/intelligence/veille/README.md" 