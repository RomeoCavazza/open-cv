"use client";

import { useState } from "react";
import Link from "next/link";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { signup } from "@/lib/auth/actions";
import Button from "@/components/ui/Button";
import Input from "@/components/ui/Input";

export default function RegisterPage() {
  const router = useRouter();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const [formData, setFormData] = useState({
    username: "",
    email: "",
    password: "",
    confirmPassword: "",
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError("");

    if (formData.password !== formData.confirmPassword) {
      setError("Les mots de passe ne correspondent pas");
      setIsLoading(false);
      return;
    }

    if (formData.password.length < 8) {
      setError("Le mot de passe doit contenir au moins 8 caracteres");
      setIsLoading(false);
      return;
    }

    try {
      const result = await signup(formData.username, formData.email, formData.password);

      if (result.error) {
        setError(result.error);
        return;
      }

      router.push("/");
      router.refresh();
    } catch {
      setError("Erreur lors de l'inscription");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main className="relative w-screen h-screen overflow-hidden">
      {/* BACKGROUND */}
      <div className="fixed inset-0 bg-[url('/background.png')] bg-cover bg-center bg-no-repeat brightness-[0.7] contrast-[1.1] z-0" />

      {/* MAIN LAYOUT */}
      <div className="relative z-10 flex w-full h-full items-center justify-center">
        
        {/* CENTER CONTENT */}
        <div className="flex flex-col items-center justify-center p-6">
          
          {/* Logo */}
          <Image
            src="/logo.png"
            alt="Hello World logo"
            width={120}
            height={120}
            className="mb-6"
          />

          {/* Header Message */}
          <header className="mb-8 text-center">
            <h1 className="text-white text-xl font-bold">
              Welcome to <span className="text-[#ff3333]">HELLO WORLD</span> messaging platform
            </h1>
          </header>

          {/* AUTH FORM */}
          <section className="w-[380px] p-10 bg-[rgba(20,20,20,0.85)] backdrop-blur-xl rounded-2xl shadow-[0_20px_60px_rgba(0,0,0,0.5)] border-2 border-[#4fdfff] animate-[fadeIn_0.5s_ease]">
            <div className="text-center mb-8">
              <h3 className="text-white font-bold tracking-widest text-lg">INSCRIPTION</h3>
            </div>

            {error && (
              <div className="bg-red-500/20 border border-red-500 text-red-300 px-4 py-3 rounded-lg mb-6 text-sm">
                {error}
              </div>
            )}

            <form onSubmit={handleSubmit} className="flex flex-col gap-4">
              <Input
                type="text"
                placeholder="Pseudo"
                required
                minLength={3}
                maxLength={32}
                value={formData.username}
                onChange={(e) => setFormData({ ...formData, username: e.target.value })}
              />
              <Input
                type="email"
                placeholder="Email"
                required
                value={formData.email}
                onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              />
              <Input
                type="password"
                placeholder="Mot de passe"
                required
                minLength={8}
                value={formData.password}
                onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                helperText="Minimum 8 caracteres"
              />
              <Input
                type="password"
                placeholder="Confirmer le mot de passe"
                required
                value={formData.confirmPassword}
                onChange={(e) => setFormData({ ...formData, confirmPassword: e.target.value })}
              />
              <Button
                type="submit"
                variant="primary"
                size="lg"
                isLoading={isLoading}
                fullWidth
                className="mt-4"
              >
                S&apos;INSCRIRE
              </Button>
            </form>

            <div className="mt-6 text-center text-sm">
              <Link href="/login" className="text-[#4fdfff] hover:underline transition-colors">
                Deja un compte ? Se connecter
              </Link>
            </div>
          </section>

        </div>
      </div>
    </main>
  );
}
