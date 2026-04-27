"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { User, updateMe, UpdateProfilePayload } from "@/lib/api-server";
import { logout } from "@/lib/auth/actions";
import { normalizeAvatarUrl } from "@/lib/avatar";
import { UserStatus, STATUS_LABELS, getStatusColor, getStatusLabel, normalizeStatus } from "@/lib/presence";
import Button from "@/components/ui/Button";
import Input from "@/components/ui/Input";

// Générer la liste des 100 avatars
const AVATARS = Array.from({ length: 100 }, (_, i) =>
  `/avatars/avatar_${String(i + 1).padStart(3, '0')}.png`
);

interface ProfileCardProps {
  user: User;
  onClose: () => void;
  onUpdate?: (user: User) => void;
}

export default function ProfileCard({ user, onClose, onUpdate }: ProfileCardProps) {
  const router = useRouter();
  const [isEditing, setIsEditing] = useState(false);
  const [showAvatarPicker, setShowAvatarPicker] = useState(false);
  const [editData, setEditData] = useState({
    username: user.username,
    avatar_url: normalizeAvatarUrl(user.avatar_url) || "",
    status: normalizeStatus(user.status),
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleLogout = async () => {
    await logout();
    router.push("/login");
    router.refresh();
  };

  const currentStatus = normalizeStatus(user.status);
  const memberSince = new Date(user.created_at).toLocaleDateString("fr-FR", {
    day: "numeric",
    month: "long",
    year: "numeric",
  });

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      const payload: UpdateProfilePayload = {};
      if (editData.username !== user.username) payload.username = editData.username;
      if (editData.avatar_url !== (normalizeAvatarUrl(user.avatar_url) || "")) payload.avatar_url = editData.avatar_url || undefined;
      if (editData.status !== currentStatus) payload.status = editData.status;

      if (Object.keys(payload).length > 0) {
        const updatedUser = await updateMe(payload);
        onUpdate?.(updatedUser);
      }
      setIsEditing(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors de la mise à jour");
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/70 backdrop-blur-md"
        onClick={onClose}
      />

      {/* Card */}
      <div className="relative w-full max-w-[420px] bg-[rgba(5,10,15,0.98)] border border-[#4fdfff]/30 rounded-xl shadow-[0_20px_60px_rgba(0,0,0,0.8)] overflow-hidden animate-[fadeIn_0.2s_ease]">

        {/* Header avec avatar */}
        <div className="relative bg-gradient-to-br from-[#4fdfff]/10 to-[#ff3333]/5 p-6 pb-16">
          {/* Close button */}
          <button
            onClick={onClose}
            className="absolute top-4 right-4 w-8 h-8 rounded-full bg-black/30 hover:bg-black/50 flex items-center justify-center text-white/60 hover:text-white transition-colors"
            aria-label="Fermer"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>

          {/* Avatar */}
          <div className="flex justify-center">
            <div className="relative">
              <div className="w-24 h-24 rounded-full bg-[rgba(5,10,15,0.98)] p-1 border-2 border-[#4fdfff]/50">
                {user.avatar_url ? (
                  <img
                    src={normalizeAvatarUrl(user.avatar_url) || ''}
                    alt={user.username}
                    className="w-full h-full rounded-full object-cover"
                  />
                ) : (
                  <div className="w-full h-full rounded-full bg-gradient-to-br from-[#4fdfff] to-[#ff3333] flex items-center justify-center">
                    <span className="text-white text-3xl font-bold">
                      {user.username.charAt(0).toUpperCase()}
                    </span>
                  </div>
                )}
              </div>
              {/* Status badge */}
              <div className={`absolute bottom-0 right-0 w-6 h-6 rounded-full border-[3px] border-[rgba(5,10,15,0.98)] ${getStatusColor(currentStatus)}`} />
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="px-6 pb-6 -mt-8">
          {/* Username section */}
          <div className="bg-[rgba(0,0,0,0.4)] rounded-lg p-4 mb-4 border border-[#4fdfff]/20">
            {isEditing ? (
              <div className="space-y-4">
                {/* Username edit */}
                <Input
                  id="edit-username"
                  type="text"
                  label="Nom d'utilisateur"
                  value={editData.username}
                  onChange={(e) => setEditData({ ...editData, username: e.target.value })}
                />

                {/* Avatar selector */}
                <div>
                  <label className="text-[10px] font-bold text-white/60 uppercase tracking-wider block mb-2">
                    Avatar
                  </label>
                  <button
                    type="button"
                    onClick={() => setShowAvatarPicker(!showAvatarPicker)}
                    className="w-full px-3 py-2 bg-[rgba(20,20,20,0.8)] border border-[#4fdfff]/30 rounded-lg text-white text-sm focus:outline-none focus:border-[#4fdfff] focus:shadow-[0_0_8px_rgba(79,223,255,0.3)] flex items-center gap-2 hover:bg-[rgba(20,20,20,0.95)] transition-all"
                  >
                    {editData.avatar_url && (
                      <img src={editData.avatar_url} alt="Avatar" className="w-6 h-6 rounded" />
                    )}
                    <span>{showAvatarPicker ? "Fermer" : "Choisir un avatar"}</span>
                  </button>

                  {showAvatarPicker && (
                    <div className="mt-2 max-h-[240px] overflow-y-auto bg-[rgba(0,0,0,0.6)] border border-[#4fdfff]/20 rounded-lg p-3 grid grid-cols-8 gap-2">
                      {AVATARS.map((avatarUrl, index) => (
                        <button
                          key={index}
                          type="button"
                          onClick={() => {
                            setEditData({ ...editData, avatar_url: avatarUrl });
                            setShowAvatarPicker(false);
                          }}
                          className={`w-10 h-10 rounded-lg hover:ring-2 hover:ring-[#4fdfff] transition-all ${editData.avatar_url === avatarUrl ? "ring-2 ring-[#4fdfff]" : ""
                            }`}
                        >
                          <img
                            src={avatarUrl}
                            alt={`Avatar ${index + 1}`}
                            className="w-full h-full rounded-lg object-cover"
                          />
                        </button>
                      ))}
                    </div>
                  )}
                </div>

                {/* Status edit */}
                <div>
                  <label htmlFor="edit-status" className="text-[10px] font-bold text-white/60 uppercase tracking-wider block mb-2">
                    Statut
                  </label>
                  <select
                    id="edit-status"
                    value={editData.status}
                    onChange={(e) => setEditData({ ...editData, status: e.target.value as UserStatus })}
                    className="w-full px-3 py-2 bg-[rgba(20,20,20,0.8)] border border-[#4fdfff]/30 rounded-lg text-white text-sm focus:outline-none focus:border-[#4fdfff] focus:shadow-[0_0_8px_rgba(79,223,255,0.3)] transition-all"
                  >
                    {Object.entries(STATUS_LABELS).map(([value, label]) => (
                      <option key={value} value={value}>{label}</option>
                    ))}
                  </select>
                </div>

                {error && (
                  <div className="bg-red-500/20 border border-red-500/50 rounded-lg p-2">
                    <p className="text-red-400 text-xs">{error}</p>
                  </div>
                )}
              </div>
            ) : (
              <div className="text-center">
                <h2 className="text-2xl font-bold text-white mb-1">{user.username}</h2>
                <p className="text-sm text-white/50">@{user.username.toLowerCase().replace(/\s+/g, "")}</p>
              </div>
            )}
          </div>

          {/* Info section */}
          <div className="bg-[rgba(0,0,0,0.4)] rounded-lg p-4 mb-4 border border-[#4fdfff]/20 space-y-4">
            {/* Status display */}
            <div>
              <h4 className="text-[10px] font-bold text-white/50 uppercase tracking-wider mb-2">
                Statut
              </h4>
              <div className="flex items-center gap-2">
                <div className={`w-3 h-3 rounded-full ${getStatusColor(currentStatus)}`} />
                <span className="text-sm text-white">{getStatusLabel(currentStatus)}</span>
              </div>
            </div>

            {/* Member since */}
            <div>
              <h4 className="text-[10px] font-bold text-white/50 uppercase tracking-wider mb-2">
                Membre depuis
              </h4>
              <p className="text-sm text-white">{memberSince}</p>
            </div>

            {/* Email */}
            <div>
              <h4 className="text-[10px] font-bold text-white/50 uppercase tracking-wider mb-2">
                Email
              </h4>
              <p className="text-sm text-white/80 break-all">{user.email}</p>
            </div>
          </div>

          {/* Actions */}
          <div className="space-y-2">
            {isEditing ? (
              <div className="flex gap-2">
                <Button
                  variant="ghost"
                  size="md"
                  onClick={() => {
                    setIsEditing(false);
                    setShowAvatarPicker(false);
                    setEditData({
                      username: user.username,
                      avatar_url: normalizeAvatarUrl(user.avatar_url) || "",
                      status: currentStatus,
                    });
                    setError(null);
                  }}
                  className="flex-1"
                >
                  Annuler
                </Button>
                <Button
                  variant="primary"
                  size="md"
                  onClick={handleSave}
                  isLoading={saving}
                  className="flex-1"
                >
                  Sauvegarder
                </Button>
              </div>
            ) : (
              <>
                <Button
                  variant="primary"
                  size="md"
                  onClick={() => setIsEditing(true)}
                  fullWidth
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                  </svg>
                  Modifier le profil
                </Button>
                <Button
                  variant="danger"
                  size="md"
                  onClick={handleLogout}
                  fullWidth
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                  </svg>
                  Déconnexion
                </Button>
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

