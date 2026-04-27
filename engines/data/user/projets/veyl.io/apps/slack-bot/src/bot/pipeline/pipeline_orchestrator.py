"""
Orchestrateur de Pipeline
Coordonne tous les exécuteurs de pipeline
"""

import logging
from typing import Dict, List, Optional
from datetime import datetime
import asyncio

from .pipeline_config import PipelineConfig, PipelineResult
from .pipeline_executors import (
    VeilleExecutor,
    StyleGuideExecutor,
    PresentationExecutor,
    DeliveryExecutor
)

logger = logging.getLogger(__name__)

class AutomatedPipeline:
    """Pipeline automatisé principal refactorisé"""

    def __init__(self, config: PipelineConfig):
        self.config = config
        self.veille_executor = VeilleExecutor(config)
        self.style_guide_executor = StyleGuideExecutor(config)
        self.presentation_executor = PresentationExecutor(config)
        self.delivery_executor = DeliveryExecutor(config)

    async def execute_full_pipeline(self) -> PipelineResult:
        """Exécute le pipeline complet"""
        start_time = datetime.now()
        result = PipelineResult(success=False)

        logger.info(f" Starting automated pipeline for {self.config.brand_name}")

        try:
            # Étape 1: Collecte de veille
            if self.config.enable_veille:
                result.veille_data = await self.veille_executor.execute()
                result.logs.append("Veille collection completed")

            # Étape 2: Génération du style guide
            if self.config.enable_ai_da and result.veille_data:
                style_guide, image_prompts = await self.style_guide_executor.execute(result.veille_data)
                result.style_guide = style_guide
                result.image_prompts = image_prompts
                result.logs.append("Style guide generation completed")

            # Étape 3: Génération de présentation
            if result.veille_data and result.style_guide:
                presentation_path = await self.presentation_executor.execute(
                    result.veille_data,
                    result.style_guide,
                    result.image_prompts
                )
                result.presentation_path = presentation_path
                result.logs.append("Presentation generation completed")

            # Étape 4: Livraison
            if result.presentation_path:
                delivery_result = await self.delivery_executor.execute(
                    result.presentation_path,
                    result.veille_data,
                    result.style_guide
                )
                result.logs.append("Delivery completed")

            result.success = True
            result.execution_time = (datetime.now() - start_time).total_seconds()

            logger.info(f"✅ Pipeline completed successfully in {result.execution_time:.2f}s")

        except Exception as e:
            result.errors.append(str(e))
            result.execution_time = (datetime.now() - start_time).total_seconds()
            logger.error(f"❌ Pipeline failed: {e}")

        return result

class PipelineOrchestrator:
    """Orchestrateur de multiples pipelines"""

    def __init__(self):
        self.pipelines = []

    def add_pipeline(self, config: PipelineConfig) -> 'PipelineOrchestrator':
        """Ajoute un pipeline à l'orchestrateur"""
        pipeline = AutomatedPipeline(config)
        self.pipelines.append(pipeline)
        logger.info(f"Added pipeline for {config.brand_name}")
        return self

    async def execute_all_pipelines(self) -> List[PipelineResult]:
        """Exécute tous les pipelines"""
        logger.info(f" Executing {len(self.pipelines)} pipelines")

        tasks = [pipeline.execute_full_pipeline() for pipeline in self.pipelines]
        results = await asyncio.gather(*tasks, return_exceptions=True)

        # Traiter les exceptions
        processed_results = []
        for i, result in enumerate(results):
            if isinstance(result, Exception):
                logger.error(f"Pipeline {i} failed with exception: {result}")
                error_result = PipelineResult(success=False)
                error_result.errors.append(str(result))
                processed_results.append(error_result)
            else:
                processed_results.append(result)

        logger.info(f"✅ All pipelines executed: {len(processed_results)} results")
        return processed_results

    def generate_summary_report(self) -> Dict:
        """Génère un rapport de synthèse"""
        return {
            'total_pipelines': len(self.pipelines),
            'generated_at': datetime.now().isoformat(),
            'pipeline_configs': [p.config.to_dict() for p in self.pipelines]
        }

# Fonctions de test et compatibilité
async def test_automated_pipeline():
    """Test du pipeline automatisé"""
    config = PipelineConfig(
        brand_name="Test Brand",
        sector="Technology",
        project_type="Website",
        target_audience="Tech professionals",
        brand_story="A story about innovation",
        core_values=["Innovation", "Quality"],
        positioning="Market leader",
        competitors=["Competitor A", "Competitor B"]
    )

    pipeline = AutomatedPipeline(config)
    result = await pipeline.execute_full_pipeline()

    print(f"Pipeline test result: {result.success}")
    return result

async def test_pipeline_orchestrator():
    """Test de l'orchestrateur"""
    orchestrator = PipelineOrchestrator()

    # Ajouter quelques pipelines de test
    configs = [
        PipelineConfig(
            brand_name=f"Test Brand {i}",
            sector="Technology",
            project_type="Website",
            target_audience="Tech professionals",
            brand_story=f"Story {i}",
            core_values=["Innovation"],
            positioning="Leader",
            competitors=["Comp A"]
        )
        for i in range(3)
    ]

    for config in configs:
        orchestrator.add_pipeline(config)

    results = await orchestrator.execute_all_pipelines()

    print(f"Orchestrator test: {len(results)} pipelines executed")
    return results
