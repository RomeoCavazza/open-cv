#!/usr/bin/env python3
"""
Script de setup automatique pour Revolver.bot
Usage: python scripts/setup_dev.py
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

def run_command(cmd, description=""):
    """Exécute une commande et gère les erreurs"""
    print(f" {description or cmd}")
    try:
        result = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
        if result.stdout:
            print(f"✅ {result.stdout.strip()}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Erreur: {e.stderr.strip()}")
        return False

def setup_environment():
    """Configure l'environnement de développement"""
    print(" SETUP REVOLVER.BOT - ENVIRONNEMENT DEV")
    print("=" * 50)
    
    # 1. Vérifier Python 3.10+
    print("\n1. Vérification Python...")
    python_version = sys.version_info
    if python_version < (3, 10):
        print(f"❌ Python 3.10+ requis. Version actuelle: {python_version.major}.{python_version.minor}")
        return False
    print(f"✅ Python {python_version.major}.{python_version.minor}.{python_version.micro}")
    
    # 2. Vérifier/créer .env
    print("\n2. Configuration .env...")
    if not os.path.exists(".env"):
        if os.path.exists(".env.template"):
            shutil.copy(".env.template", ".env")
            print("✅ .env créé depuis template")
            print("⚠️  IMPORTANT: Éditer .env avec vos vraies API keys!")
        else:
            print("❌ .env.template manquant")
            return False
    else:
        print("✅ .env existe déjà")
    
    # 3. Installer dépendances
    print("\n3. Installation dépendances...")
    if not run_command("pip install -r requirements.txt", "Installation requirements.txt"):
        return False
    
    # 4. Créer répertoires nécessaires
    print("\n4. Création répertoires...")
    dirs = ["logs", "output", "data/temp", "cache"]
    for dir_path in dirs:
        Path(dir_path).mkdir(parents=True, exist_ok=True)
        print(f"✅ {dir_path}/")
    
    # 5. Tests de base
    print("\n5. Tests configuration...")
    if not run_command("python -c 'import fastapi, openai, requests; print(\"Dépendances OK\")'", "Test imports"):
        return False
    
    # 6. Test API
    print("\n6. Test démarrage API...")
    print(" Tentative démarrage API (5s)...")
    api_test = subprocess.Popen(
        ["python", "-m", "uvicorn", "src.api.main:app", "--host", "0.0.0.0", "--port", "8002"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    
    import time
    time.sleep(3)  # Laisser le temps de démarrer
    
    # Test health endpoint
    test_health = run_command("curl -s http://localhost:8002/health || echo 'API non accessible'", "Test health endpoint")
    
    # Arrêter l'API
    api_test.terminate()
    api_test.wait()
    
    print("\n" + "=" * 50)
    print(" SETUP TERMINÉ!")
    print("\nPROCHAINES ÉTAPES:")
    print("1. Éditer .env avec vos vraies API keys")
    print("2. python -m uvicorn src.api.main:app --reload")
    print("3. Ouvrir http://localhost:8000/docs")
    
    return True

if __name__ == "__main__":
    try:
        success = setup_environment()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        print("\n❌ Setup interrompu")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Erreur: {e}")
        sys.exit(1) 