"use client";

import { useState, useEffect } from "react";
import { getMe, User } from "@/lib/api-server";

export function useAuth() {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadUser() {
      try {
        setLoading(true);
        const userData = await getMe();
        setUser(userData);
      } catch (err) {
        console.error("Erreur auth:", err);
        setError("Session invalide");
      } finally {
        setLoading(false);
      }
    }
    loadUser();
  }, []);

  return { user, loading, error };
}