#!/usr/bin/env python3
"""
AUDIT COMPLET DE QUALITÉ DU CODE - Revolver AI Bot
====================================================

Ce script audite systématiquement le code pour identifier :
- Duplications de fonctions
- Imports redondants
- Code mort
- Patterns répétitifs
- Optimisations possibles

Usage: python audit_code_quality.py
"""

import os
import re
import ast
import hashlib
from typing import Dict, List, Set, Tuple
from pathlib import Path
from collections import defaultdict, Counter

class CodeAuditor:
    """Auditeur complet de qualité du code"""

    def __init__(self, src_path: str = "src"):
        self.src_path = Path(src_path)
        self.issues = []
        self.stats = defaultdict(int)

    def audit_all(self) -> Dict:
        """Audit complet de tous les aspects"""
        print(" AUDIT COMPLET DE QUALITÉ DU CODE")
        print("=" * 50)

        results = {
            'duplications': self.audit_duplications(),
            'imports': self.audit_imports(),
            'dead_code': self.audit_dead_code(),
            'complexity': self.audit_complexity(),
            'patterns': self.audit_patterns(),
            'efficiency': self.audit_efficiency()
        }

        self.print_report(results)
        return results

    def audit_duplications(self) -> Dict:
        """Audit des duplications de code"""
        print(" Audit des duplications...")

        functions = {}
        duplications = []

        # Collecter toutes les fonctions
        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                tree = ast.parse(content)
                for node in ast.walk(tree):
                    if isinstance(node, ast.FunctionDef):
                        func_code = ast.get_source_segment(content, node)
                        if func_code:
                            func_hash = hashlib.md5(func_code.encode()).hexdigest()
                            func_info = {
                                'name': node.name,
                                'file': str(py_file),
                                'code': func_code,
                                'hash': func_hash
                            }

                            if func_hash in functions:
                                duplications.append({
                                    'original': functions[func_hash],
                                    'duplicate': func_info
                                })
                            else:
                                functions[func_hash] = func_info

            except Exception as e:
                print(f"⚠️ Erreur dans {py_file}: {e}")

        return {
            'total_functions': len(functions),
            'duplications': duplications,
            'duplication_rate': len(duplications) / len(functions) * 100 if functions else 0
        }

    def audit_imports(self) -> Dict:
        """Audit des imports"""
        print(" Audit des imports...")

        all_imports = defaultdict(list)
        unused_imports = []

        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                # Extraire les imports
                tree = ast.parse(content)
                imported_names = set()

                for node in ast.walk(tree):
                    if isinstance(node, ast.Import):
                        for alias in node.names:
                            imported_names.add(alias.asname or alias.name.split('.')[0])
                    elif isinstance(node, ast.ImportFrom):
                        for alias in node.names:
                            imported_names.add(alias.asname or alias.name)

                # Vérifier utilisation
                for name in imported_names:
                    if name not in content:
                        unused_imports.append({
                            'file': str(py_file),
                            'import': name
                        })

                # Collecter stats
                for line in content.split('\n'):
                    if line.strip().startswith(('import ', 'from ')):
                        module = line.split()[1].split('.')[0]
                        all_imports[module].append(str(py_file))

            except Exception as e:
                print(f"⚠️ Erreur imports {py_file}: {e}")

        return {
            'unused_imports': unused_imports,
            'import_frequency': dict(Counter([m for files in all_imports.values() for m in files])),
            'total_imports': sum(len(files) for files in all_imports.values())
        }

    def audit_dead_code(self) -> Dict:
        """Audit du code mort"""
        print(" Audit du code mort...")

        dead_functions = []
        unused_variables = []

        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                tree = ast.parse(content)

                # Fonctions définies mais non utilisées
                defined_functions = set()
                called_functions = set()

                for node in ast.walk(tree):
                    if isinstance(node, ast.FunctionDef):
                        defined_functions.add(node.name)
                    elif isinstance(node, ast.Call):
                        if isinstance(node.func, ast.Name):
                            called_functions.add(node.func.id)

                unused_functions = defined_functions - called_functions
                for func in unused_functions:
                    if not func.startswith('_'):  # Ignorer fonctions privées
                        dead_functions.append({
                            'file': str(py_file),
                            'function': func
                        })

            except Exception as e:
                print(f"⚠️ Erreur dead code {py_file}: {e}")

        return {
            'dead_functions': dead_functions,
            'unused_variables': unused_variables
        }

    def audit_complexity(self) -> Dict:
        """Audit de la complexité"""
        print(" Audit de complexité...")

        complexity_issues = []

        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                lines = len(content.split('\n'))
                functions = len(re.findall(r'^def ', content, re.MULTILINE))

                # Complexité par fonction
                tree = ast.parse(content)
                for node in ast.walk(tree):
                    if isinstance(node, ast.FunctionDef):
                        func_lines = len(ast.get_source_segment(content, node).split('\n'))
                        if func_lines > 50:  # Fonction trop longue
                            complexity_issues.append({
                                'file': str(py_file),
                                'function': node.name,
                                'lines': func_lines,
                                'issue': 'Fonction trop longue'
                            })

                # Fichier trop gros
                if lines > 1000:
                    complexity_issues.append({
                        'file': str(py_file),
                        'lines': lines,
                        'issue': 'Fichier trop gros'
                    })

            except Exception as e:
                print(f"⚠️ Erreur complexité {py_file}: {e}")

        return {
            'complexity_issues': complexity_issues,
            'total_files': len(list(self.src_path.rglob("*.py")))
        }

    def audit_patterns(self) -> Dict:
        """Audit des patterns répétitifs"""
        print(" Audit des patterns répétitifs...")

        patterns = defaultdict(list)

        # Patterns courants à détecter
        pattern_checks = {
            'try_except': r'try:\s*.*?\s*except',
            'if_none': r'if .* is None',
            'async_def': r'async def',
            'list_comprehension': r'\[.* for .* in .*\]',
            'dict_comprehension': r'\{.* for .* in .*\}'
        }

        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                for pattern_name, pattern in pattern_checks.items():
                    matches = len(re.findall(pattern, content, re.DOTALL))
                    if matches > 0:
                        patterns[pattern_name].append({
                            'file': str(py_file),
                            'count': matches
                        })

            except Exception as e:
                print(f"⚠️ Erreur patterns {py_file}: {e}")

        return dict(patterns)

    def audit_efficiency(self) -> Dict:
        """Audit d'efficacité"""
        print("⚡ Audit d'efficacité...")

        efficiency_issues = []

        for py_file in self.src_path.rglob("*.py"):
            try:
                with open(py_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                # Détecter ineffciencies
                issues = []

                # Boucles imbriquées
                nested_loops = len(re.findall(r'for .*:\s*.*for .*:', content, re.DOTALL))
                if nested_loops > 2:
                    issues.append(f"{nested_loops} boucles imbriquées")

                # Listes créées puis modifiées
                list_creations = len(re.findall(r'\[\]', content))
                if list_creations > 10:
                    issues.append(f"{list_creations} listes vides créées")

                # Strings concaténées
                string_concat = len(re.findall(r'.*\+.*\+.*\+', content))
                if string_concat > 5:
                    issues.append(f"{string_concat} concaténations de strings")

                if issues:
                    efficiency_issues.append({
                        'file': str(py_file),
                        'issues': issues
                    })

            except Exception as e:
                print(f"⚠️ Erreur efficacité {py_file}: {e}")

        return {
            'efficiency_issues': efficiency_issues,
            'total_issues': len(efficiency_issues)
        }

    def print_report(self, results: Dict):
        """Afficher le rapport complet"""
        print("\n" + "="*60)
        print(" RAPPORT D'AUDIT COMPLET")
        print("="*60)

        # Duplications
        dup = results['duplications']
        print(f"\n DUPLICATIONS:")
        print(f"   • Total fonctions: {dup['total_functions']}")
        print(f"   • Duplications trouvées: {len(dup['duplications'])}")
        print(f"   • Taux de duplication: {dup['duplication_rate']:.1f}%")

        if dup['duplications']:
            print("    Principales duplications:")
            for i, dup_pair in enumerate(dup['duplications'][:5]):
                print(f"     {i+1}. {dup_pair['original']['name']} dans {dup_pair['original']['file']}")
                print(f"        ↳ Dupliqué dans {dup_pair['duplicate']['file']}")

        # Imports
        imp = results['imports']
        print(f"\n IMPORTS:")
        print(f"   • Total imports: {imp['total_imports']}")
        print(f"   • Imports inutilisés: {len(imp['unused_imports'])}")

        if imp['unused_imports']:
            print("    Imports inutilisés:")
            for unused in imp['unused_imports'][:5]:
                print(f"     • {unused['import']} dans {unused['file']}")

        # Code mort
        dead = results['dead_code']
        print(f"\n CODE MORT:")
        print(f"   • Fonctions inutilisées: {len(dead['dead_functions'])}")

        if dead['dead_functions']:
            print("    Fonctions inutilisées:")
            for func in dead['dead_functions'][:5]:
                print(f"     • {func['function']} dans {func['file']}")

        # Complexité
        comp = results['complexity']
        print(f"\n COMPLEXITÉ:")
        print(f"   • Fichiers analysés: {comp['total_files']}")
        print(f"   • Problèmes de complexité: {len(comp['complexity_issues'])}")

        if comp['complexity_issues']:
            print("    Problèmes détectés:")
            for issue in comp['complexity_issues'][:5]:
                print(f"     • {issue['file']}: {issue['issue']}")

        # Efficacité
        eff = results['efficiency']
        print(f"\n⚡ EFFICACITÉ:")
        print(f"   • Problèmes d'efficacité: {eff['total_issues']}")

        # Recommandations
        print(f"\n RECOMMANDATIONS:")
        recommendations = self.generate_recommendations(results)
        for rec in recommendations[:10]:
            print(f"   • {rec}")

        print(f"\n✅ Audit terminé - {len(self.issues)} problèmes identifiés")

    def generate_recommendations(self, results: Dict) -> List[str]:
        """Générer des recommandations d'optimisation"""
        recommendations = []

        # Duplications
        if results['duplications']['duplications']:
            recommendations.append(f"Éliminer {len(results['duplications']['duplications'])} duplications de fonctions")

        # Imports inutilisés
        if results['imports']['unused_imports']:
            recommendations.append(f"Supprimer {len(results['imports']['unused_imports'])} imports inutilisés")

        # Code mort
        if results['dead_code']['dead_functions']:
            recommendations.append(f"Supprimer {len(results['dead_code']['dead_functions'])} fonctions inutilisées")

        # Complexité
        if results['complexity']['complexity_issues']:
            recommendations.append(f"Refactoriser {len(results['complexity']['complexity_issues'])} éléments complexes")

        # Efficacité
        if results['efficiency']['total_issues']:
            recommendations.append(f"Optimiser {results['efficiency']['total_issues']} problèmes d'efficacité")

        return recommendations


def main():
    """Fonction principale"""
    auditor = CodeAuditor()
    results = auditor.audit_all()

    # Sauvegarder les résultats
    import json
    with open('audit_results.json', 'w') as f:
        json.dump(results, f, indent=2, default=str)

    print("\n Résultats sauvegardés dans audit_results.json")


if __name__ == "__main__":
    main()