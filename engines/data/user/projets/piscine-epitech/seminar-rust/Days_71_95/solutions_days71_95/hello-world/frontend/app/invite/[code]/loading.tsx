export default function LoadingInvite() {
  return (
    <main className="min-h-screen flex items-center justify-center p-6">
      <div className="w-full max-w-lg bg-[rgba(20,20,20,0.92)] border-2 border-[#4fdfff] rounded-xl p-6 shadow-[0_0_40px_rgba(79,223,255,0.25)]">
        <div className="w-14 h-14 border-2 border-[#4fdfff] border-t-transparent rounded-full animate-spin mx-auto mb-4" />
        <p className="text-[#4fdfff] font-mono text-sm tracking-widest text-center animate-pulse">
          LOADING INVITE...
        </p>
      </div>
    </main>
  );
}