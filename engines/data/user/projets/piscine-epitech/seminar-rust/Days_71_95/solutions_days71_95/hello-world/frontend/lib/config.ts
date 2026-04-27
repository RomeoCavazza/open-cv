/**
 * Configuration centralisée de l'application
 */

// Utilisation du préfixe NEXT_PUBLIC pour que Vercel puisse transmettre la variable au navigateur
export const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";
