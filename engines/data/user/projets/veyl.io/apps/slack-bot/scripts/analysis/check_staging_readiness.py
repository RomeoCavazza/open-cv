#!/usr/bin/env python3
"""
Script de vérification de préparation pour le staging
"""

import os
import sys
from pathlib import Path
import importlib

def check_imports():
    """Vérifier les imports critiques"""
    print(" Vérification des imports...")
    
    critical_modules = [
        ("src.scout.core.scout_config", "ScoutConfig"),
        ("src.api.main", "app"),
        ("src.bot.orchestrator", "main"),
        ("src.scout.data.weekly_models", "WeeklyReportGenerator"),
        ("src.scout.data.recommendation_models", "Recommendation"),
        ("src.scout.intelligence.ai.specialized_prompts", "SpecializedPrompts"),
        ("src.scout.livrables.templates.slide_templates", "template_manager")
    ]
    
    failed_imports = []
    
    for module_name, class_name in critical_modules:
        try:
            module = importlib.import_module(module_name)
            if hasattr(module, class_name):
                print(f"✅ {module_name}.{class_name}")
            else:
                print(f"⚠️  {module_name} importé mais {class_name} manquant")
                failed_imports.append(f"{module_name}.{class_name}")
        except ImportError as e:
            print(f"❌ {module_name}: {e}")
            failed_imports.append(module_name)
    
    return len(failed_imports) == 0

def check_config_files():
    """Vérifier les fichiers de configuration"""
    print("\n Vérification des fichiers de configuration...")
    
    config_files = [
        ".env.staging",
        "config/config.py",
        "src/scout/core/scout_config.py"
    ]
    
    missing_files = []
    
    for file_path in config_files:
        if Path(file_path).exists():
            print(f"✅ {file_path}")
        else:
            print(f"❌ {file_path} manquant")
            missing_files.append(file_path)
    
    return len(missing_files) == 0

def check_docker_files():
    """Vérifier les fichiers Docker"""
    print("\n Vérification des fichiers Docker...")
    
    docker_files = [
        "Dockerfile",
        "docker-compose.yml",
        "requirements.txt"
    ]
    
    missing_files = []
    
    for file_path in docker_files:
        if Path(file_path).exists():
            print(f"✅ {file_path}")
        else:
            print(f"❌ {file_path} manquant")
            missing_files.append(file_path)
    
    return len(missing_files) == 0

def check_dependencies():
    """Vérifier les dépendances critiques"""
    print("\n Vérification des dépendances...")
    
    critical_deps = [
        "fastapi",
        "uvicorn", 
        "openai",
        "slack_sdk",
        "redis",
        "psycopg2"
    ]
    
    missing_deps = []
    
    for dep in critical_deps:
        try:
            importlib.import_module(dep.replace("-", "_"))
            print(f"✅ {dep}")
        except ImportError:
            print(f"❌ {dep} manquant")
            missing_deps.append(dep)
    
    return len(missing_deps) == 0

def check_api_endpoints():
    """Vérifier les endpoints API"""
    print("\n Vérification des endpoints API...")
    
    try:
        from src.api.main import app
        
        # Vérifier les routes critiques
        critical_routes = [
            "/health",
            "/slack/events",
            "/upload-brief"
        ]
        
        available_routes = []
        for route in app.routes:
            if hasattr(route, 'path'):
                available_routes.append(route.path)
        
        print(f"✅ {len(available_routes)} routes disponibles")
        
        # Vérifier les routes critiques
        for route in critical_routes:
            if route in available_routes:
                print(f"✅ {route}")
            else:
                print(f"⚠️  {route} non trouvé")
        
        return True
        
    except Exception as e:
        print(f"❌ Erreur API: {e}")
        return False

def check_bot_functionality():
    """Vérifier la fonctionnalité du bot"""
    print("\n Vérification de la fonctionnalité du bot...")
    
    try:
        from src.bot.orchestrator import main
        
        # Test simple de la fonction main (sans fichier réel)
        try:
            # Test avec mode invalide pour vérifier la gestion d'erreur
            main(mode="invalid_mode")
        except RuntimeError as e:
            if "Mode non reconnu" in str(e):
                print("✅ Bot gère correctement les modes invalides")
                return True
            else:
                raise e
        
        return True
        
    except Exception as e:
        print(f"❌ Erreur bot: {e}")
        return False

def main():
    """Fonction principale"""
    print(" VÉRIFICATION DE PRÉPARATION POUR LE STAGING")
    print("=" * 50)
    
    checks = [
        ("Imports critiques", check_imports),
        ("Fichiers de configuration", check_config_files),
        ("Fichiers Docker", check_docker_files),
        ("Dépendances", check_dependencies),
        ("Endpoints API", check_api_endpoints),
        ("Fonctionnalité du bot", check_bot_functionality)
    ]
    
    results = []
    
    for check_name, check_func in checks:
        try:
            result = check_func()
            results.append((check_name, result))
        except Exception as e:
            print(f"❌ Erreur dans {check_name}: {e}")
            results.append((check_name, False))
    
    # Résumé
    print("\n" + "=" * 50)
    print(" RÉSUMÉ DE PRÉPARATION")
    print("=" * 50)
    
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    for check_name, result in results:
        status = "✅" if result else "❌"
        print(f"{status} {check_name}")
    
    print(f"\n Score: {passed}/{total} ({passed/total*100:.1f}%)")
    
    if passed == total:
        print(" PRÊT POUR LE STAGING !")
        print("\n Prochaines étapes:")
        print("1. Compléter le fichier .env.staging avec tes secrets")
        print("2. Modifier docker-compose.yml pour utiliser .env.staging")
        print("3. Lancer: docker-compose up --build")
    else:
        print("⚠️  CORRECTIONS NÉCESSAIRES AVANT LE STAGING")
        print("\n Actions requises:")
        for check_name, result in results:
            if not result:
                print(f"- Corriger: {check_name}")
    
    return passed == total

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1) 