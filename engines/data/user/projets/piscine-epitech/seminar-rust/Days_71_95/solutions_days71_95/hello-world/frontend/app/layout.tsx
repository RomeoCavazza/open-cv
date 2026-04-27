import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "HELLO WORLD",
  description: "Real Time Messaging Platform",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="fr" className="w-full h-full overflow-hidden">
      <body className="w-full h-full font-medium text-white font-[system-ui,-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif] antialiased [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:bg-[#4fdfff]/50 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:hover:bg-[#4fdfff] [&::-webkit-scrollbar-track]:bg-black/30">
        <div className="relative w-screen h-screen">
          {/* Background avec overlay moderne */}
          <div 
            className="fixed inset-0 bg-[url('/background.png')] bg-cover bg-center bg-no-repeat brightness-[0.4] contrast-[1.1] saturate-[1.2] z-0" 
          />
          <div className="fixed inset-0 bg-gradient-to-b from-black/70 via-black/60 to-black/70 backdrop-blur-[2px] z-[1]" />
          <div className="relative z-10">
            {children}
          </div>
        </div>
      </body>
    </html>
  );
}
