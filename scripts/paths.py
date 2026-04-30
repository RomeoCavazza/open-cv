from __future__ import annotations

from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parents[1]
DATA_ROOT = PROJECT_ROOT / "data"
WEB_ROOT = PROJECT_ROOT / "web"
OFFERS_ROOT = DATA_ROOT / "offres"
INSTANCES_ROOT = DATA_ROOT / "instances"
TEMPLATES_ROOT = DATA_ROOT / "templates"

