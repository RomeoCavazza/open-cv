export type UserStatus = "online" | "offline" | "dnd" | "invisible";

export const STATUS_COLORS: Record<UserStatus, string> = {
  online: "bg-green-500",
  offline: "bg-gray-500",
  dnd: "bg-red-500",
  invisible: "bg-gray-400",
};

export const STATUS_LABELS: Record<UserStatus, string> = {
  online: "En ligne",
  offline: "Hors ligne",
  dnd: "Ne pas déranger",
  invisible: "Invisible",
};

/**
 * Normalise un statut (insensible à la casse, fallback sur offline)
 */
export function normalizeStatus(status: string | null | undefined): UserStatus {
  const s = status?.toLowerCase() || "offline";
  if (["online", "offline", "dnd", "invisible"].includes(s)) {
    return s as UserStatus;
  }
  return "offline";
}

/**
 * Retourne la couleur CSS pour un statut
 */
export function getStatusColor(status: string | null | undefined): string {
  return STATUS_COLORS[normalizeStatus(status)];
}

/**
 * Retourne le label traduit pour un statut
 */
export function getStatusLabel(status: string | null | undefined): string {
  return STATUS_LABELS[normalizeStatus(status)];
}
