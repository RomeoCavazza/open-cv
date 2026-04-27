import { getInvite } from "@/lib/api-server";
import { redirect } from "next/navigation";

export default async function InvitePage({
  params,
}: {
  params: Promise<{ code: string }>;
}) {
  const { code } = await params;

  let invite: Awaited<ReturnType<typeof getInvite>>;
  try {
    invite = await getInvite(code);
  } catch (e: any) {
    const msg = String(e?.message || "");
    if (msg.toLowerCase().includes("authentication required") || msg.includes("401")) {
      redirect(`/login?next=/invite/${code}`);
    }
    return (
      <main className="min-h-screen flex items-center justify-center p-6">
        <div className="w-full max-w-lg bg-[rgba(20,20,20,0.92)] border-2 border-[#4fdfff] rounded-xl p-6 shadow-[0_0_40px_rgba(79,223,255,0.25)]">
          <h1 className="text-white font-bold text-xl mb-2">Invalid invite</h1>
          <p className="text-white/60 text-sm">
            This invite link is invalid or expired.
          </p>
        </div>
      </main>
    );
  }

  async function onJoin() {
    "use server";
    const { acceptInvite } = await import("@/lib/api-server");
    try {
      await acceptInvite(code);
    } catch (e: any) {
      // Si déjà membre, rediriger quand même vers le serveur
      if (e?.message?.includes("Already a member")) {
        redirect(`/`);
      }
      throw e;
    }
    redirect(`/`);
  }

  const expiresText = invite.expires_at ? new Date(invite.expires_at).toLocaleString("fr-FR") : "never";
  const maxUsesText = invite.max_uses ?? "∞";

  return (
    <main className="min-h-screen flex items-center justify-center p-6">
      <div className="w-full max-w-lg bg-[rgba(20,20,20,0.92)] border-2 border-[#4fdfff] rounded-xl p-6 shadow-[0_0_40px_rgba(79,223,255,0.25)]">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-white font-bold text-xl">
              You’ve been invited
            </h1>
            <p className="text-white/60 text-sm mt-1">
              Invite code: <span className="text-[#4fdfff] font-mono">{invite.code}</span>
            </p>
          </div>
        </div>

        <div className="mt-5 p-4 rounded-lg border border-[#4fdfff]/40 bg-black/30">
          <p className="text-white/70 text-sm">
            Server ID: <span className="text-white font-mono">{invite.server_id}</span>
          </p>
          <p className="text-white/70 text-sm mt-2">
            Max uses: <span className="text-white font-mono">{maxUsesText}</span> • Uses:{" "}
            <span className="text-white font-mono">{invite.uses}</span>
          </p>
          <p className="text-white/70 text-sm mt-2">
            Expires: <span className="text-white font-mono">{expiresText}</span>
          </p>
          <p className="text-white/40 text-xs mt-3">
            Anyone with this link can join this server.
          </p>
        </div>

        <form action={onJoin} className="mt-6 flex gap-3">
          <button
            type="submit"
            className="flex-1 p-[10px] bg-[#a00000] border-2 border-[#4fdfff] text-white font-bold cursor-pointer rounded-lg text-[13px] uppercase transition-all duration-300 hover:bg-[#c00000] hover:shadow-[0_0_12px_rgba(79,223,255,0.8)] hover:scale-[1.02] active:scale-[0.98]"
          >
            Join server
          </button>

          <a
            href="/"
            className="px-4 py-2.5 text-white/60 hover:text-white hover:underline transition-colors text-sm flex items-center"
          >
            Cancel
          </a>
        </form>
      </div>
    </main>
  );
}