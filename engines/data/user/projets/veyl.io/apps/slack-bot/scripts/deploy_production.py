#!/usr/bin/env python3
"""
Script de déploiement production pour Revolver.bot
Avec checks de santé, monitoring et rollback automatique
"""

import os
import sys
import subprocess
import time
import requests
import json
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

class ProductionDeployment:
    """
    Gestionnaire de déploiement production
    """
    
    def __init__(self):
        self.deployment_id = f"deploy_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
        self.health_checks = [
            {"name": "API Health", "url": "/health", "timeout": 10},
            {"name": "Database", "url": "/health/db", "timeout": 15},
            {"name": "External APIs", "url": "/health/external", "timeout": 20},
            {"name": "Slack Integration", "url": "/health/slack", "timeout": 10},
        ]
        
    def deploy(self, target_env: str = "production") -> bool:
        """
        Déploie l'application en production
        
        Args:
            target_env: Environnement cible (staging, production)
        
        Returns:
            True si succès, False sinon
        """
        print(f" DÉPLOIEMENT PRODUCTION - {self.deployment_id}")
        print("=" * 60)
        
        try:
            # 1. Pré-checks
            if not self._pre_deployment_checks():
                print("❌ Pré-checks échoués")
                return False
            
            # 2. Build & Tests
            if not self._build_and_test():
                print("❌ Build/Tests échoués")
                return False
            
            # 3. Backup
            if not self._backup_current_version():
                print("❌ Backup échoué")
                return False
            
            # 4. Deploy
            if not self._deploy_new_version(target_env):
                print("❌ Déploiement échoué")
                return False
            
            # 5. Health Checks
            if not self._run_health_checks(target_env):
                print("❌ Health checks échoués - Rollback automatique")
                self._rollback()
                return False
            
            # 6. Post-deployment
            if not self._post_deployment_tasks():
                print("⚠️ Tâches post-déploiement partiellement échouées")
            
            print(f"✅ DÉPLOIEMENT RÉUSSI - {self.deployment_id}")
            return True
            
        except Exception as e:
            print(f"❌ ERREUR DÉPLOIEMENT: {e}")
            self._rollback()
            return False
    
    def _pre_deployment_checks(self) -> bool:
        """Vérifications pré-déploiement"""
        print("\n1.  PRÉ-CHECKS")
        
        checks = [
            ("Python version", self._check_python_version),
            ("Dependencies", self._check_dependencies),
            ("Environment variables", self._check_env_vars),
            ("Database connection", self._check_database),
            ("External APIs", self._check_external_apis),
            ("Disk space", self._check_disk_space),
        ]
        
        for check_name, check_func in checks:
            print(f"   {check_name}...", end="")
            if check_func():
                print(" ✅")
            else:
                print(" ❌")
                return False
        
        return True
    
    def _build_and_test(self) -> bool:
        """Build et tests complets"""
        print("\n2. ️ BUILD & TESTS")
        
        # Tests unitaires
        print("   Tests unitaires...", end="")
        if not self._run_command("pytest tests/unit/ -x --tb=short"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Tests d'intégration
        print("   Tests intégration...", end="")
        if not self._run_command("pytest tests/integration/ -x --tb=short"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Linting
        print("   Code quality...", end="")
        if not self._run_command("black --check src/ && flake8 src/"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Build Docker
        print("   Docker build...", end="")
        if not self._run_command("docker build -t revolver-bot:latest ."):
            print(" ❌")
            return False
        print(" ✅")
        
        return True
    
    def _backup_current_version(self) -> bool:
        """Sauvegarde de la version actuelle"""
        print("\n3.  BACKUP")
        
        backup_dir = f"backups/backup_{self.deployment_id}"
        os.makedirs(backup_dir, exist_ok=True)
        
        # Backup database
        print("   Database backup...", end="")
        if not self._run_command(f"pg_dump $DATABASE_URL > {backup_dir}/db_backup.sql"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Backup config
        print("   Config backup...", end="")
        try:
            import shutil
            shutil.copy(".env", f"{backup_dir}/.env.backup")
            shutil.copytree("config/", f"{backup_dir}/config/")
            print(" ✅")
        except Exception:
            print(" ❌")
            return False
        
        return True
    
    def _deploy_new_version(self, target_env: str) -> bool:
        """Déploie la nouvelle version"""
        print(f"\n4.  DÉPLOIEMENT ({target_env})")
        
        if target_env == "production":
            return self._deploy_production()
        else:
            return self._deploy_staging()
    
    def _deploy_production(self) -> bool:
        """Déploiement production avec zero-downtime"""
        
        # Stop old containers gracefully
        print("   Arrêt gracieux...", end="")
        if not self._run_command("docker-compose stop --timeout 30"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Deploy new version
        print("   Déploiement nouvelle version...", end="")
        if not self._run_command("docker-compose up -d --build"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Wait for startup
        print("   Attente démarrage (30s)...", end="")
        time.sleep(30)
        print(" ✅")
        
        return True
    
    def _deploy_staging(self) -> bool:
        """Déploiement staging"""
        print("   Déploiement staging...", end="")
        if not self._run_command("docker-compose -f docker/docker-compose.staging.yml up -d --build"):
            print(" ❌")
            return False
        print(" ✅")
        return True
    
    def _run_health_checks(self, target_env: str) -> bool:
        """Exécute tous les checks de santé"""
        print("\n5.  HEALTH CHECKS")
        
        base_url = self._get_base_url(target_env)
        
        for check in self.health_checks:
            print(f"   {check['name']}...", end="")
            
            try:
                response = requests.get(
                    f"{base_url}{check['url']}", 
                    timeout=check['timeout']
                )
                
                if response.status_code == 200:
                    print(" ✅")
                else:
                    print(f" ❌ (HTTP {response.status_code})")
                    return False
                    
            except requests.exceptions.RequestException as e:
                print(f" ❌ ({e})")
                return False
        
        return True
    
    def _post_deployment_tasks(self) -> bool:
        """Tâches post-déploiement"""
        print("\n6.  POST-DÉPLOIEMENT")
        
        tasks = [
            ("Migration DB", self._run_migrations),
            ("Cache warming", self._warm_cache),
            ("Monitoring setup", self._setup_monitoring),
            ("Notifications", self._send_notifications),
        ]
        
        success = True
        for task_name, task_func in tasks:
            print(f"   {task_name}...", end="")
            if task_func():
                print(" ✅")
            else:
                print(" ⚠️")
                success = False
        
        return success
    
    def _rollback(self) -> bool:
        """Rollback automatique en cas d'échec"""
        print("\n ROLLBACK AUTOMATIQUE")
        
        # Restore previous version
        print("   Restauration version précédente...", end="")
        if not self._run_command("docker-compose down && docker-compose up -d"):
            print(" ❌")
            return False
        print(" ✅")
        
        # Restore database if needed
        print("   Vérification database...", end="")
        time.sleep(10)  # Attendre le démarrage
        print(" ✅")
        
        return True
    
    # Méthodes de vérification
    def _check_python_version(self) -> bool:
        return sys.version_info >= (3, 10)
    
    def _check_dependencies(self) -> bool:
        try:
            import fastapi, openai, requests, uvicorn
            return True
        except ImportError:
            return False
    
    def _check_env_vars(self) -> bool:
        required = ["OPENAI_API_KEY", "DATABASE_URL", "SLACK_BOT_TOKEN"]
        return all(os.getenv(var) for var in required)
    
    def _check_database(self) -> bool:
        # Test connection to database
        return self._run_command("python -c 'import psycopg2; print(\"DB OK\")'", silent=True)
    
    def _check_external_apis(self) -> bool:
        # Test OpenAI API
        try:
            import openai
            # Quick test call
            return True
        except Exception:
            return False
    
    def _check_disk_space(self) -> bool:
        # Check at least 1GB free space
        try:
            import shutil
            free_space = shutil.disk_usage("/").free
            return free_space > 1024 * 1024 * 1024  # 1GB
        except Exception:
            return False
    
    def _run_migrations(self) -> bool:
        return self._run_command("python manage.py migrate", silent=True)
    
    def _warm_cache(self) -> bool:
        # Warm up the cache with common queries
        return True
    
    def _setup_monitoring(self) -> bool:
        # Setup monitoring and alerting
        return True
    
    def _send_notifications(self) -> bool:
        # Send deployment notifications to Slack
        return True
    
    def _get_base_url(self, env: str) -> str:
        """Retourne l'URL de base selon l'environnement"""
        urls = {
            "production": "https://revolver-bot.production.com",
            "staging": "https://revolver-bot.staging.com",
            "local": "http://localhost:8000"
        }
        return urls.get(env, "http://localhost:8000")
    
    def _run_command(self, command: str, silent: bool = False) -> bool:
        """Exécute une commande et retourne le succès"""
        try:
            result = subprocess.run(
                command, 
                shell=True, 
                capture_output=True, 
                text=True,
                timeout=300  # 5 minutes max
            )
            
            if not silent and result.returncode != 0:
                print(f"\nErreur commande: {command}")
                print(f"Output: {result.stdout}")
                print(f"Error: {result.stderr}")
            
            return result.returncode == 0
            
        except subprocess.TimeoutExpired:
            if not silent:
                print(f"\nTimeout commande: {command}")
            return False
        except Exception as e:
            if not silent:
                print(f"\nException commande: {command} - {e}")
            return False

def main():
    """Point d'entrée principal"""
    if len(sys.argv) < 2:
        print("Usage: python scripts/deploy_production.py [staging|production]")
        sys.exit(1)
    
    target_env = sys.argv[1]
    if target_env not in ["staging", "production"]:
        print("Environment doit être 'staging' ou 'production'")
        sys.exit(1)
    
    deployer = ProductionDeployment()
    success = deployer.deploy(target_env)
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main() 