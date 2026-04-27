#!/bin/bash

echo "========================================"
echo "  LANCEMENT DE TOUS LES TESTS"
echo "========================================"
echo ""

cd "$(dirname "$0")"

echo "[1/3] Nettoyage complet du dossier reports..."
rm -rf core/build/reports
echo "OK"

echo ""
echo "[2/3] Compilation du projet..."
./gradlew :core:compileJava :core:compileTestJava
if [ $? -ne 0 ]; then
    echo ""
    echo "ERREUR: La compilation a echoue"
    exit 1
fi

echo ""
echo "[3/3] Execution des tests..."
./gradlew :core:test --console=plain

if [ $? -eq 0 ]; then
    echo ""
    echo "========================================"
    echo "  TOUS LES TESTS SONT PASSES"
    echo "========================================"
    echo ""
    echo "Rapport HTML disponible dans:"
    echo "core/build/reports/tests/test/index.html"
    echo ""
else
    echo ""
    echo "========================================"
    echo "  CERTAINS TESTS ONT ECHOUE"
    echo "========================================"
    echo ""
    exit 1
fi

