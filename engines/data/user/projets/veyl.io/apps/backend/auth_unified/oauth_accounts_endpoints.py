# oauth_accounts_endpoints.py
# Endpoints pour gérer les comptes OAuth connectés

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session
from typing import List
from db.base import get_db
from db.models import User, OAuthAccount
from auth_unified.auth_endpoints import get_current_user

oauth_accounts_router = APIRouter(prefix="/api/v1/auth/accounts", tags=["oauth-accounts"])

@oauth_accounts_router.get("/connected")
def get_connected_accounts(
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Récupérer tous les comptes OAuth connectés pour l'utilisateur actuel"""
    accounts = db.query(OAuthAccount).filter(OAuthAccount.user_id == current_user.id).all()
    
    return {
        "accounts": [
            {
                "id": account.id,
                "provider": account.provider,
                "provider_user_id": account.provider_user_id,
                "connected_at": account.created_at.isoformat() if account.created_at else None,
                "has_token": bool(account.access_token)
            }
            for account in accounts
        ]
    }

@oauth_accounts_router.delete("/{account_id}")
def disconnect_account(
    account_id: int,
    db: Session = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Déconnecter un compte OAuth"""
    account = db.query(OAuthAccount).filter(
        OAuthAccount.id == account_id,
        OAuthAccount.user_id == current_user.id
    ).first()
    
    if not account:
        raise HTTPException(status_code=404, detail="Compte non trouvé")
    
    provider = account.provider
    db.delete(account)
    db.commit()
    
    return {
        "message": f"Compte {provider} déconnecté avec succès",
        "provider": provider
    }

