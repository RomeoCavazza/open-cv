import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

// Routes publiques (pas besoin d'auth)
const publicRoutes = ["/login", "/register"];

export default function proxy(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Verifier si c'est une route publique
  const isPublicRoute = publicRoutes.some((route) => pathname.startsWith(route));

  // Recuperer le token depuis les cookies
  const token = request.cookies.get("token")?.value;

  // Si pas de token et route protegee -> redirect login
  if (!token && !isPublicRoute) {
    const loginUrl = new URL("/login", request.url);
    loginUrl.searchParams.set("redirect", pathname);
    return NextResponse.redirect(loginUrl);
  }

  // Si token et route auth -> redirect dashboard
  if (token && isPublicRoute) {
    return NextResponse.redirect(new URL("/", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    // Matcher toutes les routes sauf les assets statiques
    "/((?!api|_next/static|_next/image|favicon.ico|.*\\.png$|.*\\.jpg$).*)",
  ],
};

