"""
Extraction de sections spécialisées pour les briefs
Module spécialisé pour l'analyse de briefs
"""

import re
import logging
from typing import Dict, Any, Optional, List
from datetime import datetime

logger = logging.getLogger(__name__)

class PDFSectionExtractor:
    """Extracteur de sections spécialisé pour les briefs"""

    def __init__(self):
        # Patterns pour identifier les sections communes dans les briefs
        self.section_patterns = {
            'objectives': re.compile(r'(?:objectives?|goals?|buts?|cibles?)', re.IGNORECASE),
            'target_audience': re.compile(r'(?:target audience|public cible|cible|audience)', re.IGNORECASE),
            'budget': re.compile(r'(?:budget|financement|coûts?|prix)', re.IGNORECASE),
            'timeline': re.compile(r'(?:timeline|calendrier|planning|dates?|délais?)', re.IGNORECASE),
            'requirements': re.compile(r'(?:requirements?|exigences?|spécifications?)', re.IGNORECASE),
            'deliverables': re.compile(r'(?:deliverables?|livrables?|résultats?)', re.IGNORECASE),
            'brand_guidelines': re.compile(r'(?:brand guidelines?|charte graphique|identité visuelle)', re.IGNORECASE),
            'success_metrics': re.compile(r'(?:success metrics?|kpis?|indicateurs?|mesure)', re.IGNORECASE),
            'contact_info': re.compile(r'(?:contact|coordonnées?|responsable)', re.IGNORECASE),
            'company_info': re.compile(r'(?:company|entreprise|client)', re.IGNORECASE)
        }

        # Patterns pour nettoyer le texte
        self.cleanup_patterns = [
            (re.compile(r'\n{3,}'), '\n\n'),  # Remplacer plusieurs sauts de ligne par deux
            (re.compile(r'[ \t]+'), ' '),     # Normaliser les espaces
            (re.compile(r'^\s+|\s+$', re.MULTILINE), ''),  # Supprimer espaces en début/fin de ligne
        ]

    def extract_brief_sections(self, text: str) -> Dict[str, Any]:
        """
        Extraction spécialisée des sections d'un brief

        Args:
            text: Texte du PDF à analyser

        Returns:
            Dictionnaire avec les sections extraites
        """
        logger.info(" Extracting brief sections")

        result = {
            'sections': {},
            'extraction_metadata': {
                'total_text_length': len(text),
                'extraction_timestamp': datetime.now().isoformat(),
                'sections_found': 0
            }
        }

        try:
            # Nettoyer le texte
            cleaned_text = self._clean_text(text)

            # Extraire les sections
            sections = self._identify_sections(cleaned_text)

            result['sections'] = sections
            result['extraction_metadata']['sections_found'] = len(sections)

            # Analyser les sections
            analysis = self._analyze_sections(sections)
            result['analysis'] = analysis

            logger.info(f"✅ Extracted {len(sections)} sections from brief")

        except Exception as e:
            logger.error(f"Section extraction failed: {e}")
            result['error'] = str(e)

        return result

    def _clean_text(self, text: str) -> str:
        """Nettoie le texte extrait du PDF"""
        cleaned = text

        for pattern, replacement in self.cleanup_patterns:
            cleaned = pattern.sub(replacement, cleaned)

        return cleaned.strip()

    def _identify_sections(self, text: str) -> Dict[str, str]:
        """Identifie et extrait les sections du brief"""
        sections = {}
        lines = text.split('\n')

        current_section = None
        current_content = []

        for line in lines:
            line = line.strip()

            # Vérifier si c'est une nouvelle section
            for section_name, pattern in self.section_patterns.items():
                if pattern.search(line) and len(line) < 100:  # Ligne pas trop longue
                    # Sauvegarder la section précédente
                    if current_section and current_content:
                        sections[current_section] = '\n'.join(current_content).strip()

                    # Nouvelle section
                    current_section = section_name
                    current_content = [line]
                    break
            else:
                # Ligne de contenu pour la section actuelle
                if current_section:
                    current_content.append(line)

        # Sauvegarder la dernière section
        if current_section and current_content:
            sections[current_section] = '\n'.join(current_content).strip()

        return sections

    def _analyze_sections(self, sections: Dict[str, str]) -> Dict[str, Any]:
        """Analyse les sections extraites"""
        analysis = {
            'completeness_score': 0,
            'missing_sections': [],
            'quality_indicators': {}
        }

        # Liste des sections essentielles
        essential_sections = ['objectives', 'target_audience', 'budget', 'timeline']

        # Calculer le score de complétude
        found_essential = sum(1 for section in essential_sections if section in sections)
        analysis['completeness_score'] = (found_essential / len(essential_sections)) * 100

        # Identifier les sections manquantes
        analysis['missing_sections'] = [
            section for section in essential_sections
            if section not in sections
        ]

        # Analyse de qualité
        for section_name, content in sections.items():
            quality = self._assess_section_quality(section_name, content)
            analysis['quality_indicators'][section_name] = quality

        return analysis

    def _assess_section_quality(self, section_name: str, content: str) -> Dict[str, Any]:
        """Évalue la qualité d'une section"""
        quality = {
            'length_score': 0,
            'content_score': 0,
            'clarity_score': 0
        }

        # Score de longueur (basé sur la section)
        content_length = len(content)
        if section_name in ['objectives', 'requirements']:
            quality['length_score'] = min(100, content_length / 2)  # Attendre au moins 200 chars
        elif section_name == 'budget':
            quality['length_score'] = 100 if content_length > 10 else 50
        else:
            quality['length_score'] = min(100, content_length)

        # Score de contenu (présence de mots-clés)
        content_lower = content.lower()
        keywords = {
            'objectives': ['goal', 'objective', 'aim', 'target'],
            'budget': ['€', '$', 'euros', 'dollars', 'budget'],
            'timeline': ['week', 'month', 'deadline', 'date'],
            'target_audience': ['audience', 'demographic', 'target', 'public']
        }

        if section_name in keywords:
            found_keywords = sum(1 for keyword in keywords[section_name] if keyword in content_lower)
            quality['content_score'] = min(100, found_keywords * 25)

        # Score de clarté (structure)
        sentences = len([s for s in content.split('.') if s.strip()])
        quality['clarity_score'] = min(100, sentences * 10)

        return quality

    def extract_custom_sections(self, text: str, custom_patterns: Dict[str, str]) -> Dict[str, str]:
        """Extrait des sections personnalisées"""
        sections = {}

        for section_name, pattern_str in custom_patterns.items():
            try:
                pattern = re.compile(pattern_str, re.IGNORECASE | re.DOTALL)
                match = pattern.search(text)

                if match:
                    sections[section_name] = match.group(1).strip() if match.groups() else match.group(0).strip()

            except re.error as e:
                logger.error(f"Invalid regex pattern for {section_name}: {e}")

        return sections

# Fonction de compatibilité
def extract_brief_sections(text: str) -> Dict[str, Any]:
    """Fonction de compatibilité pour l'ancien code"""
    extractor = PDFSectionExtractor()
    return extractor.extract_brief_sections(text)
