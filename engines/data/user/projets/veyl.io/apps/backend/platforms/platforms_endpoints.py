# platforms/platforms_endpoints.py
from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy.orm import Session
from typing import List, Optional
from db.base import get_db
from db.models import Platform, User
from auth_unified.auth_endpoints import get_current_user
from .schemas import PlatformCreate, PlatformResponse, PlatformUpdate

platforms_router = APIRouter(prefix="/api/v1/platforms", tags=["platforms"])

@platforms_router.get("/", response_model=List[PlatformResponse])
def get_platforms(
    skip: int = Query(0, ge=0),
    limit: int = Query(100, ge=1, le=1000),
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Récupérer les plateformes"""
    platforms = db.query(Platform).offset(skip).limit(limit).all()
    return platforms

@platforms_router.get("/{platform_id}", response_model=PlatformResponse)
def get_platform(
    platform_id: int,
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Récupérer une plateforme par ID"""
    platform = db.query(Platform).filter(Platform.id == platform_id).first()
    if not platform:
        raise HTTPException(status_code=404, detail="Plateforme non trouvée")
    return platform

@platforms_router.post("/", response_model=PlatformResponse)
def create_platform(
    platform_in: PlatformCreate,
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Créer une nouvelle plateforme"""
    # Vérifier que la plateforme n'existe pas déjà
    existing = db.query(Platform).filter(Platform.name == platform_in.name).first()
    if existing:
        raise HTTPException(status_code=400, detail="Plateforme déjà existante")
    
    platform = Platform(**platform_in.dict())
    db.add(platform)
    db.commit()
    db.refresh(platform)
    return platform

@platforms_router.put("/{platform_id}", response_model=PlatformResponse)
def update_platform(
    platform_id: int,
    platform_in: PlatformUpdate,
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Mettre à jour une plateforme"""
    platform = db.query(Platform).filter(Platform.id == platform_id).first()
    if not platform:
        raise HTTPException(status_code=404, detail="Plateforme non trouvée")
    
    update_data = platform_in.dict(exclude_unset=True)
    for field, value in update_data.items():
        setattr(platform, field, value)
    
    db.commit()
    db.refresh(platform)
    return platform

@platforms_router.delete("/{platform_id}")
def delete_platform(
    platform_id: int,
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Supprimer une plateforme"""
    platform = db.query(Platform).filter(Platform.id == platform_id).first()
    if not platform:
        raise HTTPException(status_code=404, detail="Plateforme non trouvée")
    
    db.delete(platform)
    db.commit()
    return {"message": "Plateforme supprimée"}
