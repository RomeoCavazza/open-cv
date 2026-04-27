"use client";

import { useEffect, useMemo, useState } from "react";
import { createInvite } from "@/lib/api-server";

type Props = {
  serverId: string;
  serverName: string;
  onClose: () => void;
};

export default function InviteModal({ serverId, serverName, onClose }: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [code, setCode] = useState<string | null>(null);

  const inviteLink = useMemo(() => {
    if (!code) return null;
    return `${window.location.origin}/invite/${code}`;
  }, [code]);

  useEffect(() => {
    (async () => {
      try {
        setLoading(true);
        setError(null);
        const res = await createInvite(serverId, { max_uses: 10, expires_at: null });
        setCode(res.code);
      } catch (e: any) {
        setError(e?.message ?? "Failed to create invite");
      } finally {
        setLoading(false);
      }
    })();
  }, [serverId]);

  const handleCopy = async () => {
    if (!inviteLink) return;
    await navigator.clipboard.writeText(inviteLink);
  };

  const handleRegenerate = async () => {
    try {
      setLoading(true);
      setError(null);
      const res = await createInvite(serverId, { max_uses: 10, expires_at: null });
      setCode(res.code);
    } catch (e: any) {
      setError(e?.message ?? "Failed to create invite");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/80 backdrop-blur-sm" onClick={onClose} />

      <div className="relative w-full max-w-md rounded-xl border-2 border-[#4fdfff] bg-[rgba(20,20,20,0.98)] p-6 shadow-[0_0_40px_rgba(79,223,255,0.3)]">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-white font-bold text-lg">
            Invite to <span className="text-[#4fdfff]">{serverName}</span>
          </h2>
          <button onClick={onClose} className="text-white/60 hover:text-white text-xl">
            ×
          </button>
        </div>

        {error && (
          <div className="mb-4 rounded-lg border border-red-500 bg-red-500/20 px-4 py-3 text-sm text-red-200">
            {error}
          </div>
        )}

        <div className="space-y-3">
          <label className="block text-[10px] font-bold text-[#4fdfff] tracking-widest uppercase">
            Invite link
          </label>

          <div className="flex gap-2">
            <input
              readOnly
              value={inviteLink ?? (loading ? "Generating..." : "")}
              className="flex-1 px-3 py-2 bg-black/50 border border-[#4fdfff]/50 rounded-lg text-white text-sm outline-none"
            />
            <button
              onClick={handleCopy}
              disabled={!inviteLink}
              className="px-3 py-2 rounded-lg border-2 border-[#4fdfff] text-white font-bold hover:shadow-[0_0_12px_rgba(79,223,255,0.6)] disabled:opacity-50"
            >
              Copy
            </button>
          </div>

          <div className="flex gap-2 pt-2">
            <button
              onClick={handleRegenerate}
              disabled={loading}
              className="flex-1 py-2 rounded-lg bg-[#a00000] border-2 border-[#4fdfff] text-white font-bold hover:bg-[#c00000] disabled:opacity-50"
            >
              Regenerate
            </button>
            <button
              onClick={onClose}
              className="flex-1 py-2 rounded-lg text-white/70 hover:text-white hover:underline"
            >
              Close
            </button>
          </div>

          <p className="text-xs text-white/40">
            Max uses: 10 • Expires: never
          </p>
        </div>
      </div>
    </div>
  );
}