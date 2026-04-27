#!/bin/bash

echo " Nettoyage de Revolver AI Bot..."

# Suppression des fichiers de cache Python
echo " Suppression des fichiers de cache Python..."
find . -name "*.pyc" -delete
find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true

# Suppression des fichiers temporaires
echo "️ Suppression des fichiers temporaires..."
rm -f test_*.pptx test_*.csv test_results_*.json test_output.csv collected_tests.txt

# Suppression des rapports de couverture
echo " Suppression des rapports de couverture..."
rm -rf coverage_reports htmlcov .coverage

# Suppression des dossiers vides
echo " Suppression des dossiers vides..."
find . -type d -empty -delete

# Nettoyage des logs
echo " Nettoyage des logs..."
find . -name "*.log" -delete

echo "✅ Nettoyage terminé !"
echo " Espace utilisé :"
du -sh * | sort -hr 