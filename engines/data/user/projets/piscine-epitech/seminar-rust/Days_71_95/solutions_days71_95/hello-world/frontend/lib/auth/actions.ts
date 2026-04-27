"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { API_URL } from "../config";

async function safeJson<T>(res: Response): Promise<T | null> {
  const contentType = res.headers.get("content-type") || "";
  if (!contentType.includes("application/json")) {
    return null;
  }
  try {
    return await res.json();
  } catch {
    return null;
  }
}

export async function login(email: string, password: string) {
  let res: Response;
  try {
    res = await fetch(`${API_URL}/auth/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, password }),
    });
  } catch {
    return { error: "Erreur de connexion. Vérifiez que le backend tourne (port 3001)." };
  }

  const data = await safeJson<{ error?: string; token?: string; user?: unknown }>(res);
  if (!res.ok) {
    return { error: data?.error || "Identifiants invalides" };
  }
  if (!data?.token) {
    return { error: "Erreur de connexion. Le serveur API n'a pas renvoyé de token (backend sur port 3001 ?)." };
  }

  const cookieStore = await cookies();
  cookieStore.set("token", data.token, {
    httpOnly: false,
    secure: process.env.NODE_ENV === "production",
    sameSite: "lax",
    maxAge: 60 * 60 * 24,
    path: "/",
  });

  return { user: data.user, token: data.token };
}

export async function signup(username: string, email: string, password: string) {
  let res: Response;
  try {
    res = await fetch(`${API_URL}/auth/signup`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, email, password }),
    });
  } catch {
    return { error: "Erreur de connexion. Vérifiez que le backend tourne (port 3001)." };
  }

  const data = await safeJson<{ error?: string; token?: string; user?: unknown }>(res);
  if (!res.ok) {
    return { error: data?.error || "Échec de l'inscription" };
  }
  if (!data?.token) {
    return { error: "Erreur de connexion. Le serveur API n'a pas renvoyé de token (backend sur port 3001 ?)." };
  }

  const cookieStore = await cookies();
  cookieStore.set("token", data.token, {
    httpOnly: false,
    secure: process.env.NODE_ENV === "production",
    sameSite: "lax",
    maxAge: 60 * 60 * 24,
    path: "/",
  });

  return { user: data.user };
}

export async function logout() {
  const cookieStore = await cookies();
  const token = cookieStore.get("token")?.value;

  if (token) {
    await fetch(`${API_URL}/auth/logout`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    }).catch(() => {});
  }

  cookieStore.delete("token");
  redirect("/login");
}

export async function getUser() {
  const cookieStore = await cookies();
  const token = cookieStore.get("token")?.value;

  if (!token) return null;

  const res = await fetch(`${API_URL}/me`, {
    headers: { Authorization: `Bearer ${token}` },
  });

  if (!res.ok) return null;

  return res.json();
}

export async function clearToken() {
  const cookieStore = await cookies();
  cookieStore.delete("token");
}
