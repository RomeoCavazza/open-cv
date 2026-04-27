use rand::Rng;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{CreateInvitePayload, Invite, JoinServerWithCodePayload, MemberRole};
use crate::repositories::{InviteRepository, ServerRepository};

/// Génère un code d'invitation unique (8 caractères alphanumériques)
fn generate_invite_code() -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    (0..8)
        .map(|_| {
            let idx = rng.random_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
}

pub async fn create_invite(
    invite_repo: &InviteRepository,
    server_repo: &ServerRepository,
    server_id: Uuid,
    payload: CreateInvitePayload,
    created_by: Uuid,
) -> Result<Invite> {
    // Vérifier que le serveur existe
    server_repo
        .find_by_id(server_id)
        .await?
        .ok_or(Error::ServerNotFound)?;

    // Vérifier que l'utilisateur est Admin ou Owner
    let member = server_repo
        .find_member(server_id, created_by)
        .await?
        .ok_or(Error::ServerForbidden)?;

    if member.role == MemberRole::Member {
        return Err(Error::ServerForbidden);
    }

    // Générer un code unique
    let mut code = generate_invite_code();
    let mut attempts = 0;
    while invite_repo.find_by_code(&code).await?.is_some() {
        code = generate_invite_code();
        attempts += 1;
        if attempts > 10 {
            return Err(Error::InternalError {
                message: "Failed to generate unique invite code".to_string(),
            });
        }
    }

    let invite = invite_repo
        .create(
            server_id,
            created_by,
            &code,
            payload.max_uses,
            payload.expires_at,
        )
        .await?;

    Ok(invite)
}

pub async fn list_invites(
    invite_repo: &InviteRepository,
    server_repo: &ServerRepository,
    server_id: Uuid,
    requester_id: Uuid,
) -> Result<Vec<Invite>> {
    // Vérifier que le serveur existe
    server_repo
        .find_by_id(server_id)
        .await?
        .ok_or(Error::ServerNotFound)?;

    // Vérifier que l'utilisateur est Admin ou Owner
    let member = server_repo
        .find_member(server_id, requester_id)
        .await?
        .ok_or(Error::ServerForbidden)?;

    if member.role == MemberRole::Member {
        return Err(Error::ServerForbidden);
    }

    let invites = invite_repo.list_by_server(server_id).await?;
    Ok(invites)
}

pub async fn get_invite_by_code(invite_repo: &InviteRepository, code: &str) -> Result<Invite> {
    let invite = invite_repo
        .find_by_code(code)
        .await?
        .ok_or(Error::ServerNotFound)?;
    Ok(invite)
}

pub async fn join_server_with_code(
    invite_repo: &InviteRepository,
    server_repo: &ServerRepository,
    payload: JoinServerWithCodePayload,
    user_id: Uuid,
) -> Result<Uuid> {
    // Trouver l'invitation par code
    let invite = invite_repo
        .find_by_code(&payload.code)
        .await?
        .ok_or(Error::ServerNotFound)?; // Utiliser ServerNotFound pour ne pas révéler que le code est invalide

    // Vérifier l'expiration
    if let Some(expires_at) = invite.expires_at {
        if chrono::Utc::now() > expires_at {
            return Err(Error::ServerNotFound);
        }
    }

    // Vérifier max_uses
    if let Some(max_uses) = invite.max_uses {
        if invite.uses >= max_uses {
            return Err(Error::ServerNotFound);
        }
    }

    // Vérifier que l'utilisateur n'est pas déjà membre
    if server_repo
        .find_member(invite.server_id, user_id)
        .await?
        .is_some()
    {
        return Err(Error::ServerAlreadyMember);
    }

    // Ajouter l'utilisateur au serveur
    server_repo
        .add_member(invite.server_id, user_id, MemberRole::Member)
        .await?;

    // Incrémenter le compteur d'utilisations
    invite_repo.increment_uses(invite.id).await?;

    Ok(invite.server_id)
}
