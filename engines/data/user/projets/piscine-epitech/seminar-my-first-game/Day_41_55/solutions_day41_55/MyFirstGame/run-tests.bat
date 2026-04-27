@echo off
echo ========================================
echo   LANCEMENT DE TOUS LES TESTS
echo ========================================
echo.

cd /d "%~dp0"

echo [1/3] Nettoyage complet du dossier reports...
if exist "core\build\reports" (
    rmdir /s /q "core\build\reports"
)
echo OK

echo.
echo [2/3] Compilation du projet...
call gradlew.bat :core:compileJava :core:compileTestJava
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERREUR: La compilation a echoue
    pause
    exit /b 1
)

echo.
echo [3/3] Execution des tests...
call gradlew.bat :core:test --console=plain

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo   TOUS LES TESTS SONT PASSES
    echo ========================================
    echo.
    echo Rapport HTML disponible dans:
    echo core\build\reports\tests\test\index.html
    echo.
) else (
    echo.
    echo ========================================
    echo   CERTAINS TESTS ONT ECHOUE
    echo ========================================
    echo.
)

pause

