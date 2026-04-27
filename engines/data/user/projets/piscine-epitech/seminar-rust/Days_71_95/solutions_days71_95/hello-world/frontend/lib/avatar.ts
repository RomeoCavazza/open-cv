import { User } from "@/lib/api-server";

/**
 * Normalise le chemin d'avatar (ancien format → nouveau format)
 */
export function normalizeAvatarUrl(url: string | null | undefined): string | null {
  if (!url) return null;
  if (url.includes("/space_invaders_avatars/space_invader_")) {
    return url
      .replace("/space_invaders_avatars/", "/avatars/")
      .replace("space_invader_", "avatar_");
  }
  return url;
}

/**
 * Génère une URL d'avatar déterministe basée sur un UUID
 */
export function getAvatarFromId(id: string): string {
  const hex = id.replace(/-/g, "").slice(0, 2);
  const num = parseInt(hex, 16);
  const avatarNum = (num % 100) + 1;
  return `/avatars/avatar_${String(avatarNum).padStart(3, "0")}.png`;
}

/**
 * Retourne l'avatar approprié : si c'est l'utilisateur connecté, utilise son avatar réel
 */
export function getAvatar(userId: string, currentUser: User | null): string {
  if (currentUser && userId === currentUser.id) {
    return normalizeAvatarUrl(currentUser.avatar_url) || getAvatarFromId(userId);
  }
  return getAvatarFromId(userId);
}