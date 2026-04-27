"""
Préparation des données pour génération de slides
Extrait et structure les données du brief et de la veille
"""

import logging
from typing import Dict, List, Any
from datetime import datetime

logger = logging.getLogger(__name__)

class PresentationDataPreparator:
    """Prépare et structure les données pour génération de slides"""

    def __init__(self):
        pass

    def prepare_presentation_data(self, data: Dict, style: Any) -> Dict:
        """Prépare les données complètes pour la présentation"""
        logger.info(" Preparing presentation data")

        prepared_data = {
            'brand_name': data.get('brand_name', 'Unknown Brand'),
            'sector': data.get('sector', 'General'),
            'project_type': data.get('project_type', 'Marketing Campaign'),
            'timestamp': datetime.now().isoformat(),
            'style': style,
        }

        # Extraction des insights du brief
        brief_insights = self._extract_brief_insights(data)
        prepared_data.update(brief_insights)

        # Extraction des données de veille
        if 'veille_data' in data:
            veille_insights = self._extract_veille_insights(data['veille_data'])
            prepared_data.update(veille_insights)

        # Génération des idées et priorités
        prepared_data['strategic_priorities'] = self._generate_strategic_priorities(data)
        prepared_data['ideas'] = self._generate_ideas(data)

        return prepared_data

    def _extract_brief_insights(self, data: Dict) -> Dict[str, Any]:
        """Extrait les insights du brief"""
        return {
            'brand_story': data.get('brand_story', ''),
            'target_audience': data.get('target_audience', ''),
            'objectives': data.get('objectives', []),
            'challenges': data.get('challenges', []),
            'budget': data.get('budget', 0),
            'timeline': data.get('timeline', ''),
        }

    def _extract_veille_insights(self, veille_data: Dict) -> Dict[str, Any]:
        """Extrait les insights de la veille"""
        return {
            'market_insights': self._extract_market_insights(veille_data),
            'competitive_landscape': self._extract_competitive_landscape(veille_data),
            'consumer_trends': self._extract_consumer_trends(veille_data),
            'opportunities': self._extract_opportunities(veille_data),
        }

    def _extract_market_insights(self, veille_data: Dict) -> str:
        """Extrait les insights marché"""
        insights = veille_data.get('insights', [])
        market_data = veille_data.get('market_data', {})

        if insights:
            return f"Marché en croissance avec {len(insights)} insights clés identifiés"

        return "Analyse de marché en cours"

    def _extract_competitive_landscape(self, veille_data: Dict) -> str:
        """Extrait le paysage concurrentiel"""
        competitors = veille_data.get('competitors', [])

        if competitors:
            return f"{len(competitors)} concurrents identifiés dans le secteur"

        return "Analyse concurrentielle en cours"

    def _extract_consumer_trends(self, veille_data: Dict) -> str:
        """Extrait les tendances consommateurs"""
        trends = veille_data.get('trends', [])

        if trends:
            return f"{len(trends)} tendances consommateurs identifiées"

        return "Analyse des tendances en cours"

    def _extract_opportunities(self, veille_data: Dict) -> str:
        """Extrait les opportunités"""
        return "Opportunités identifiées dans l'analyse de veille"

    def _generate_strategic_priorities(self, data: Dict) -> List[str]:
        """Génère les priorités stratégiques"""
        return [
            "Développer la présence digitale",
            "Optimiser l'engagement client",
            "Innover dans les services",
            "Renforcer la notoriété de marque"
        ]

    def _generate_ideas(self, data: Dict) -> List[Dict]:
        """Génère des idées créatives"""
        return [
            {
                'title': 'Campagne Digital Innovation',
                'description': 'Campagne multi-canal mettant en avant l\'innovation technologique',
                'impact': 'High',
                'effort': 'Medium'
            },
            {
                'title': 'Contenu Authentique',
                'description': 'Création de contenu authentique racontant des histoires vraies',
                'impact': 'High',
                'effort': 'Low'
            }
        ]
