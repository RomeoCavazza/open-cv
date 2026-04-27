#  AI & LLM Strategy - Stratégie IA & LLM

##  Résumé

**Usage IA actuel** : OpenAI pour génération texte + vision
**Cible MVP** : Résumés + ideator avec guardrails anti-hallucinations
**Évolution** : RAG + modèles propriétaires + multi-provider
**Sécurité** : Rate limiting + monitoring coûts + content filtering

---

##  Preuves - État Actuel

### Recherche Intégrations IA
```bash
grep -r -n "openai\|OpenAI" src/ | wc -l  # 74 intégrations OpenAI !
grep -r -n "openai\|OpenAI" src/ | head -5
# Résultat :
# src/bot/ai/ai_directrice_artistique.py:1:from openai import AsyncOpenAI
# src/bot/ai/brief_summarizer.py:1:from openai import AsyncOpenAI
# src/bot/ai/vision_analyzer.py:1:from openai import AsyncOpenAI
# src/bot/ai/openai_client.py:1:from openai import AsyncOpenAI
# src/bot/ai/openai_client.py:8:class OpenAIClient:
```

**Analyse** :
- ✅ **74 appels OpenAI** : Intégration très complète
- ✅ **4 modules IA** : ai_directrice, brief_summarizer, vision_analyzer, openai_client
- ✅ **AsyncOpenAI** : Utilisation moderne et asynchrone
- ✅ **Client dédié** : OpenAIClient pour centralisation

### Recherche Modèles de Prompt
```bash
find . -name "*.md" -o -name "*.txt" | xargs grep -l "prompt\|system\|assistant\|user" || true
# Résultat : Quelques fichiers avec prompts
```

**Analyse** :
- ✅ **Prompts existants** : Dans resources/prompts/
- ⚠️ **Pas versionnés** : Gestion ad-hoc des prompts
-  **Amélioration** : Prompt contracts versionnés

---

##  Architecture IA - Multi-Couches

### Layer 1 : Interface LLM
```python
# src/core/ai/llm_client.py
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional
import asyncio
import logging
from dataclasses import dataclass

logger = logging.getLogger(__name__)

@dataclass
class LLMResponse:
    """Réponse standardisée LLM"""
    content: str
    model: str
    provider: str
    tokens_used: int
    cost: float
    confidence: Optional[float] = None
    metadata: Dict[str, Any] = None

class LLMProvider(ABC):
    """Interface abstraite pour providers LLM"""

    @abstractmethod
    async def generate(self, prompt: str, **kwargs) -> LLMResponse:
        """Génération de texte"""
        pass

    @abstractmethod
    async def get_embeddings(self, texts: list[str]) -> list[list[float]]:
        """Génération d'embeddings"""
        pass

    @property
    @abstractmethod
    def name(self) -> str:
        """Nom du provider"""
        pass

class OpenAIProvider(LLMProvider):
    """Provider OpenAI"""

    def __init__(self, api_key: str):
        from openai import AsyncOpenAI
        self.client = AsyncOpenAI(api_key=api_key)
        self.model = "gpt-4-turbo-preview"

    async def generate(self, prompt: str, **kwargs) -> LLMResponse:
        try:
            response = await self.client.chat.completions.create(
                model=self.model,
                messages=[{"role": "user", "content": prompt}],
                max_tokens=kwargs.get('max_tokens', 1000),
                temperature=kwargs.get('temperature', 0.7)
            )

            # Calcul coût approximatif
            tokens = response.usage.total_tokens
            cost = self._calculate_cost(tokens)

            return LLMResponse(
                content=response.choices[0].message.content,
                model=self.model,
                provider=self.name,
                tokens_used=tokens,
                cost=cost
            )
        except Exception as e:
            logger.error(f"OpenAI error: {e}")
            raise

    def _calculate_cost(self, tokens: int) -> float:
        """Calcul coût approximatif ($/1K tokens)"""
        rates = {
            "gpt-4-turbo-preview": 0.01,  # $0.01/1K input
            "gpt-3.5-turbo": 0.002       # $0.002/1K input
        }
        return (tokens / 1000) * rates.get(self.model, 0.01)

    @property
    def name(self) -> str:
        return "openai"

class LLMClient:
    """Client unifié pour tous providers"""

    def __init__(self):
        self.providers: Dict[str, LLMProvider] = {}
        self.current_provider = "openai"

    def register_provider(self, provider: LLMProvider):
        """Enregistre un nouveau provider"""
        self.providers[provider.name] = provider

    async def generate(self, prompt: str, provider: Optional[str] = None, **kwargs) -> LLMResponse:
        """Génération avec failover automatique"""
        provider_name = provider or self.current_provider

        # Essai provider principal
        if provider_name in self.providers:
            try:
                return await self.providers[provider_name].generate(prompt, **kwargs)
            except Exception as e:
                logger.warning(f"Provider {provider_name} failed: {e}")

        # Failover vers autres providers
        for name, p in self.providers.items():
            if name != provider_name:
                try:
                    logger.info(f"Trying failover to {name}")
                    return await p.generate(prompt, **kwargs)
                except Exception as e:
                    logger.warning(f"Failover {name} also failed: {e}")

        raise Exception("All LLM providers failed")

# Instance globale
llm_client = LLMClient()
```

### Layer 2 : Guardrails Anti-Hallucinations
```python
# src/core/ai/guardrails.py
import re
import json
from typing import Dict, Any, List, Optional
from .llm_client import LLMResponse

class HallucinationGuard:
    """Protection contre hallucinations IA"""

    def __init__(self):
        # Allowlist de librairies/packages autorisés
        self.allowed_packages = {
            'fastapi', 'uvicorn', 'pydantic', 'sqlalchemy',
            'playwright', 'openai', 'requests', 'pandas'
        }

        # Patterns dangereux à détecter
        self.dangerous_patterns = [
            r'(?:rm|del|drop|delete)\s+.*',
            r'(?:exec|eval)\s*\(',
            r'(?:import|from)\s+os\s+',
            r'(?:subprocess|system)\s*\('
        ]

    def validate_response(self, response: LLMResponse) -> Dict[str, Any]:
        """Valide réponse IA pour détecter hallucinations"""
        content = response.content
        issues = []

        # 1. Détection librairies non autorisées
        mentioned_packages = self._extract_packages(content)
        unauthorized = mentioned_packages - self.allowed_packages

        if unauthorized:
            issues.append({
                "type": "unauthorized_package",
                "packages": list(unauthorized),
                "severity": "high"
            })

        # 2. Détection code dangereux
        for pattern in self.dangerous_patterns:
            if re.search(pattern, content, re.IGNORECASE):
                issues.append({
                    "type": "dangerous_code",
                    "pattern": pattern,
                    "severity": "critical"
                })

        # 3. Validation structure si JSON attendu
        if "json" in content.lower() or content.strip().startswith('{'):
            try:
                json.loads(content)
            except json.JSONDecodeError:
                issues.append({
                    "type": "invalid_json",
                    "severity": "medium"
                })

        return {
            "is_safe": len(issues) == 0,
            "issues": issues,
            "confidence": self._calculate_confidence(response, issues)
        }

    def _extract_packages(self, content: str) -> set:
        """Extrait noms de packages mentionnés"""
        # Patterns pour détecter imports et installations
        patterns = [
            r'(?:import|from)\s+([a-zA-Z_][a-zA-Z0-9_]*)',
            r'pip\s+install\s+([a-zA-Z_][a-zA-Z0-9_-]*)',
            r'npm\s+install\s+([a-zA-Z_][a-zA-Z0-9_-]*)'
        ]

        packages = set()
        for pattern in patterns:
            matches = re.findall(pattern, content, re.IGNORECASE)
            packages.update(matches)

        return packages

    def _calculate_confidence(self, response: LLMResponse, issues: List) -> float:
        """Calcule score de confiance"""
        base_confidence = 0.8  # Confiance de base en GPT-4

        # Pénalités selon sévérité
        penalties = {
            "critical": 0.5,
            "high": 0.2,
            "medium": 0.1,
            "low": 0.05
        }

        for issue in issues:
            severity = issue.get("severity", "low")
            base_confidence -= penalties.get(severity, 0.05)

        return max(0.0, base_confidence)

class ContentFilter:
    """Filtrage contenu sensible"""

    def __init__(self):
        self.sensitive_keywords = {
            'password', 'secret', 'key', 'token', 'api_key',
            'credit_card', 'ssn', 'social_security'
        }

    def filter_response(self, content: str) -> str:
        """Filtre contenu sensible"""
        for keyword in self.sensitive_keywords:
            content = re.sub(
                rf'\b{re.escape(keyword)}\b.*',
                f'{keyword}[FILTERED]',
                content,
                flags=re.IGNORECASE
            )
        return content
```

### Layer 3 : Prompt Engineering
```python
# src/core/ai/prompts.py
from typing import Dict, Any, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class PromptTemplate:
    """Template de prompt versionné"""
    id: str
    version: str
    name: str
    system_message: str
    user_template: str
    created_at: datetime
    metadata: Dict[str, Any]

class PromptManager:
    """Gestionnaire de prompts versionnés"""

    def __init__(self):
        self.templates: Dict[str, PromptTemplate] = {}
        self._load_default_templates()

    def _load_default_templates(self):
        """Charge templates par défaut"""
        self.templates["summary"] = PromptTemplate(
            id="summary_v1",
            version="1.0.0",
            name="Content Summary",
            system_message="You are a content summarization expert. Provide concise, accurate summaries.",
            user_template="""
            Summarize the following social media posts in 3-5 key insights:

            Posts: {posts}

            Focus on:
            - Main themes and topics
            - Engagement patterns
            - Brand sentiment
            - Key performance indicators

            Keep summary under 200 words.
            """,
            created_at=datetime.utcnow(),
            metadata={"max_tokens": 500, "temperature": 0.3}
        )

        self.templates["ideator"] = PromptTemplate(
            id="ideator_v1",
            version="1.0.0",
            name="Content Ideation",
            system_message="You are a creative content strategist. Generate engaging, brand-appropriate content ideas.",
            user_template="""
            Generate 3 creative content ideas for {brand} based on these insights:

            Insights: {insights}

            Target audience: {audience}
            Brand voice: {brand_voice}
            Content type: {content_type}

            For each idea, provide:
            - Concept description
            - Key message
            - Visual suggestions
            - Expected engagement
            """,
            created_at=datetime.utcnow(),
            metadata={"max_tokens": 1000, "temperature": 0.7}
        )

    def get_template(self, name: str, version: Optional[str] = None) -> PromptTemplate:
        """Récupère template par nom et version"""
        if name not in self.templates:
            raise ValueError(f"Template {name} not found")

        template = self.templates[name]

        if version and template.version != version:
            # TODO: Version management
            pass

        return template

    def render_prompt(self, template_name: str, **kwargs) -> str:
        """Rend prompt avec variables"""
        template = self.get_template(template_name)

        # Format user message
        user_message = template.user_template.format(**kwargs)

        # Combine system + user
        full_prompt = f"{template.system_message}\n\n{user_message}"

        return full_prompt

# Instance globale
prompt_manager = PromptManager()
```

---

##  Usage IA par Fonctionnalité

### 1. Résumés Automatiques
```python
# src/services/summary_service.py
from ..core.ai.llm_client import llm_client
from ..core.ai.prompts import prompt_manager
from ..core.ai.guardrails import HallucinationGuard

class SummaryService:
    def __init__(self):
        self.guard = HallucinationGuard()

    async def generate_summary(self, competitor_id: int, posts: list) -> str:
        """Génère résumé IA avec protection"""

        # Format posts pour prompt
        posts_text = "\n".join([
            f"Post: {p.get('content', '')} (Likes: {p.get('likes_count', 0)})"
            for p in posts[:20]  # Limite à 20 posts
        ])

        # Génère prompt
        prompt = prompt_manager.render_prompt(
            "summary",
            posts=posts_text
        )

        # Appel IA
        response = await llm_client.generate(
            prompt,
            max_tokens=500,
            temperature=0.3
        )

        # Validation anti-hallucinations
        validation = self.guard.validate_response(response)

        if not validation["is_safe"]:
            logger.warning(f"Unsafe response detected: {validation['issues']}")
            # Fallback vers résumé basique
            return self._fallback_summary(posts)

        return response.content

    def _fallback_summary(self, posts: list) -> str:
        """Résumé basique en cas d'échec IA"""
        total_posts = len(posts)
        avg_likes = sum(p.get('likes_count', 0) for p in posts) / max(total_posts, 1)

        return f"""Summary (fallback mode):
- Total posts analyzed: {total_posts}
- Average engagement: {avg_likes:.1f} likes per post
- Most active period: Last 7 days
- Note: AI summary unavailable, using basic metrics"""
```

### 2. Idéation Créative
```python
# src/services/ideator_service.py
from ..core.ai.llm_client import llm_client
from ..core.ai.prompts import prompt_manager

class IdeatorService:
    async def generate_ideas(
        self,
        brand: str,
        insights: str,
        audience: str,
        brand_voice: str,
        content_type: str = "social_media"
    ) -> list:
        """Génère idées créatives"""

        prompt = prompt_manager.render_prompt(
            "ideator",
            brand=brand,
            insights=insights,
            audience=audience,
            brand_voice=brand_voice,
            content_type=content_type
        )

        response = await llm_client.generate(
            prompt,
            max_tokens=1000,
            temperature=0.8  # Plus créatif
        )

        # Parse réponse en idées structurées
        return self._parse_ideas(response.content)

    def _parse_ideas(self, content: str) -> list:
        """Parse idées depuis réponse IA"""
        # Logique de parsing (sections, bullets, etc.)
        ideas = []
        sections = content.split("\n\n")

        for section in sections:
            if "idea" in section.lower() or "concept" in section.lower():
                ideas.append({
                    "title": section.split("\n")[0],
                    "description": "\n".join(section.split("\n")[1:]),
                    "type": "creative_idea"
                })

        return ideas
```

### 3. Génération Images (DALL-E)
```python
# src/services/image_service.py
import openai
from typing import Optional

class ImageService:
    def __init__(self, api_key: str):
        self.client = openai.OpenAI(api_key=api_key)

    async def generate_image(
        self,
        prompt: str,
        size: str = "1024x1024",
        quality: str = "standard"
    ) -> Optional[str]:
        """Génère image avec DALL-E"""

        try:
            response = await self.client.images.generate(
                model="dall-e-3",
                prompt=prompt,
                size=size,
                quality=quality,
                n=1
            )

            return response.data[0].url

        except Exception as e:
            logger.error(f"Image generation failed: {e}")
            return None

    async def create_slide_visual(self, slide_content: str) -> Optional[str]:
        """Crée visuel pour slide"""

        prompt = f"""
        Create a professional, modern slide visual for:

        {slide_content}

        Style: Clean, corporate, data-driven visualization
        Colors: Professional blue and white
        Layout: Chart or infographic style
        """

        return await self.generate_image(prompt)
```

---

##  Gestion Coûts & Quotas

### Monitoring Coûts
```python
# src/core/ai/cost_monitor.py
from collections import defaultdict
import time
from typing import Dict, Any

class CostMonitor:
    """Monitoring coûts IA"""

    def __init__(self):
        self.usage: Dict[str, Dict[str, Any]] = defaultdict(dict)
        self.monthly_budget = 1000.0  # $1000/mois

    def track_usage(self, provider: str, model: str, tokens: int, cost: float):
        """Track utilisation IA"""
        month_key = time.strftime("%Y-%m")

        if month_key not in self.usage:
            self.usage[month_key] = {
                "providers": defaultdict(lambda: {"tokens": 0, "cost": 0.0, "requests": 0})
            }

        provider_data = self.usage[month_key]["providers"][f"{provider}:{model}"]
        provider_data["tokens"] += tokens
        provider_data["cost"] += cost
        provider_data["requests"] += 1

    def get_monthly_cost(self) -> float:
        """Coût total du mois en cours"""
        month_key = time.strftime("%Y-%m")
        if month_key not in self.usage:
            return 0.0

        return sum(
            provider_data["cost"]
            for provider_data in self.usage[month_key]["providers"].values()
        )

    def check_budget(self) -> bool:
        """Vérifie si budget dépassé"""
        return self.get_monthly_cost() >= self.monthly_budget

    def get_usage_report(self) -> Dict[str, Any]:
        """Rapport d'utilisation détaillé"""
        month_key = time.strftime("%Y-%m")
        if month_key not in self.usage:
            return {"total_cost": 0, "providers": {}}

        data = self.usage[month_key]
        return {
            "total_cost": sum(p["cost"] for p in data["providers"].values()),
            "total_tokens": sum(p["tokens"] for p in data["providers"].values()),
            "total_requests": sum(p["requests"] for p in data["providers"].values()),
            "providers": dict(data["providers"])
        }
```

### Rate Limiting IA
```python
# src/core/ai/rate_limiter.py
import asyncio
from collections import defaultdict
import time
from typing import Dict

class AIRateLimiter:
    """Rate limiting pour APIs IA"""

    def __init__(self):
        # Limites par provider (requests/minute)
        self.limits = {
            "openai": 60,      # 60 req/min
            "anthropic": 50,   # 50 req/min
        }

        self.requests: Dict[str, list] = defaultdict(list)

    async def wait_if_needed(self, provider: str):
        """Attend si limite dépassée"""
        if provider not in self.limits:
            return

        now = time.time()
        window_start = now - 60  # 1 minute window

        # Nettoie anciennes requêtes
        self.requests[provider] = [
            req_time for req_time in self.requests[provider]
            if req_time > window_start
        ]

        # Vérifie limite
        if len(self.requests[provider]) >= self.limits[provider]:
            # Calcule temps d'attente
            oldest_request = min(self.requests[provider])
            wait_time = 60 - (now - oldest_request)

            if wait_time > 0:
                await asyncio.sleep(wait_time)

        # Enregistre requête
        self.requests[provider].append(now)
```

---

##  Évolution - RAG & Vector DB

### Architecture RAG Future
```python
# src/core/ai/rag_engine.py (v2)
from typing import List, Dict, Any
import numpy as np
from .llm_client import llm_client

class RAGEngine:
    """Retrieval-Augmented Generation"""

    def __init__(self, vector_db):
        self.vector_db = vector_db  # pgvector ou Weaviate
        self.chunk_size = 1000
        self.overlap = 200

    async def retrieve_context(self, query: str, top_k: int = 5) -> List[str]:
        """Récupère contexte pertinent depuis vector DB"""
        # Embedding de la requête
        query_embedding = await llm_client.get_embeddings([query])

        # Recherche vectorielle
        results = await self.vector_db.search(
            query_embedding[0],
            limit=top_k
        )

        return [result["content"] for result in results]

    async def generate_with_context(
        self,
        query: str,
        context_docs: List[str],
        **kwargs
    ) -> str:
        """Génération augmentée par contexte"""

        # Format contexte
        context = "\n\n".join(context_docs[:3])  # Top 3 résultats

        # Prompt RAG
        rag_prompt = f"""
        Based on the following context, answer the question:

        Context:
        {context}

        Question: {query}

        Answer concisely and accurately.
        """

        response = await llm_client.generate(rag_prompt, **kwargs)
        return response.content

    async def store_document(self, content: str, metadata: Dict[str, Any]):
        """Stocke document dans vector DB"""
        # Chunking
        chunks = self._chunk_text(content)

        # Embeddings
        embeddings = await llm_client.get_embeddings(chunks)

        # Stockage
        for chunk, embedding in zip(chunks, embeddings):
            await self.vector_db.store({
                "content": chunk,
                "embedding": embedding,
                "metadata": metadata
            })

    def _chunk_text(self, text: str) -> List[str]:
        """Découpe texte en chunks"""
        chunks = []
        start = 0

        while start < len(text):
            end = start + self.chunk_size

            # Ajuste pour ne pas couper les mots
            if end < len(text):
                while end > start and text[end] not in [' ', '\n', '.']:
                    end -= 1

            chunk = text[start:end].strip()
            if chunk:
                chunks.append(chunk)

            start = end - self.overlap

        return chunks
```

### Prompt Contracts Versionnés
```python
# src/core/ai/prompt_contracts.py
from typing import Dict, Any
from dataclasses import dataclass
from datetime import datetime

@dataclass
class PromptContract:
    """Contrat de prompt versionné"""
    id: str
    version: str
    name: str
    description: str
    input_schema: Dict[str, Any]
    output_schema: Dict[str, Any]
    system_prompt: str
    user_prompt_template: str
    created_at: datetime
    deprecated_at: Optional[datetime] = None

class PromptRegistry:
    """Registre de contrats de prompt"""

    def __init__(self):
        self.contracts: Dict[str, PromptContract] = {}

    def register(self, contract: PromptContract):
        """Enregistre nouveau contrat"""
        self.contracts[contract.id] = contract

    def get_contract(self, contract_id: str, version: Optional[str] = None) -> PromptContract:
        """Récupère contrat par ID et version"""
        if contract_id not in self.contracts:
            raise ValueError(f"Contract {contract_id} not found")

        contract = self.contracts[contract_id]

        if version and contract.version != version:
            # Version pinning
            pass

        return contract

    def validate_input(self, contract_id: str, input_data: Dict[str, Any]) -> bool:
        """Valide données d'entrée contre schéma"""
        # TODO: JSON Schema validation
        return True

    def validate_output(self, contract_id: str, output_data: Any) -> bool:
        """Valide données de sortie contre schéma"""
        # TODO: Output validation
        return True
```

---

## ⚡ Actions - Développement Prioritaire

### Semaine 1 : Infrastructure IA
1. **LLM Client** : Interface multi-provider (OpenAI + fallback)
2. **Guardrails** : Anti-hallucinations + content filtering
3. **Prompt Manager** : Templates versionnés de base

### Semaine 2 : Services IA
4. **Summary Service** : Résumés avec validation
5. **Ideator Service** : Génération idées créatives
6. **Cost Monitor** : Tracking coûts + rate limiting

### Semaine 3 : Production Ready
7. **Error Handling** : Retry + circuit breakers
8. **Caching** : Résumés fréquents en Redis
9. **Logging** : Métriques IA + alerting

### Semaine 4 : Optimisations
10. **RAG Prep** : Structure pour vector DB
11. **Batch Processing** : Traitement groupé pour coût
12. **A/B Testing** : Comparaison modèles/providers

---

##  Definition of Done

### IA MVP
- ✅ **LLM Client** : Multi-provider avec failover
- ✅ **Guardrails** : Anti-hallucinations + validation
- ✅ **Prompts** : Templates versionnés pour use cases
- ✅ **Services** : Résumé + idéation fonctionnels

### Production
- ✅ **Monitoring** : Coûts + performance trackés
- ✅ **Rate Limiting** : Protection APIs + budgets
- ✅ **Error Handling** : Graceful degradation
- ✅ **Caching** : Optimisation coût/réponse

### Évolutif
- ✅ **RAG Ready** : Structure pour vector DB
- ✅ **Multi-Modal** : Texte + image + future vidéo
- ✅ **Contracts** : Prompts versionnés + schémas
- ✅ **Testing** : A/B testing modèles/providers

---

**État actuel** : OpenAI intégré mais usage limité
**Cible** : IA robuste avec guardrails + monitoring
**Timeline** : 4 semaines pour IA MVP sécurisée