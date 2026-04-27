/**
 * Design System - Hello World RTC
 * Palette: Noir / Bleu cyan (#4fdfff) / Rouge (#ff3333)
 */

export const theme = {
  colors: {
    // Couleurs principales
    accent: {
      cyan: "#4fdfff",
      red: "#ff3333",
      redDark: "#a00000",
      redDarkHover: "#c00000",
    },
    
    // Backgrounds
    bg: {
      dark: "#0a0a0a",
      panel: "rgba(20, 20, 20, 0.85)",
      panelSolid: "#1f1f1f",
      card: "#232428",
      cardDark: "#111214",
      input: "#1e1f22",
      hover: "rgba(255, 255, 255, 0.1)",
      hoverCyan: "rgba(79, 223, 255, 0.1)",
      hoverCyan20: "rgba(79, 223, 255, 0.2)",
    },
    
    // Borders
    border: {
      cyan: "#4fdfff",
      cyanDim: "rgba(79, 223, 255, 0.3)",
      cyanDim50: "rgba(79, 223, 255, 0.5)",
      red: "#ff3333",
      gray: "#3f4147",
      grayDark: "#4e5058",
    },
    
    // Text
    text: {
      primary: "#ffffff",
      secondary: "rgba(255, 255, 255, 0.8)",
      dim: "rgba(255, 255, 255, 0.6)",
      muted: "rgba(255, 255, 255, 0.4)",
      cyan: "#4fdfff",
      red: "#ff3333",
    },
    
    // Status colors
    status: {
      online: "#10b981", // green-500
      offline: "#6b7280", // gray-500
      dnd: "#ef4444", // red-500
      invisible: "#9ca3af", // gray-400
    },
    
    // Discord-like colors (pour ProfileCard uniquement)
    discord: {
      blurple: "#5865f2",
      blurpleHover: "#4752c4",
      gray: "#4e5058",
      grayHover: "#6d6f78",
    },
  },
  
  // Shadows
  shadows: {
    glow: {
      cyan: "0 0 12px rgba(79, 223, 255, 0.5)",
      cyanStrong: "0 0 15px rgba(79, 223, 255, 0.6)",
      cyanStronger: "0 0 12px rgba(79, 223, 255, 0.8)",
      red: "0 0 8px rgba(255, 59, 59, 0.6)",
    },
    card: "0 20px 60px rgba(0, 0, 0, 0.5)",
  },
  
  // Spacing
  spacing: {
    xs: "0.25rem", // 4px
    sm: "0.5rem", // 8px
    md: "1rem", // 16px
    lg: "1.5rem", // 24px
    xl: "2rem", // 32px
  },
  
  // Border radius
  radius: {
    sm: "0.375rem", // 6px
    md: "0.5rem", // 8px
    lg: "0.75rem", // 12px
    xl: "1rem", // 16px
    full: "9999px",
  },
  
  // Transitions
  transition: {
    default: "all 0.2s ease",
    colors: "colors 0.2s ease",
    all: "all 0.3s ease",
  },
} as const;

/**
 * Classes Tailwind pré-configurées pour les boutons
 */
export const buttonVariants = {
  primary: "bg-[#a00000] border-2 border-[#4fdfff] text-white font-bold hover:bg-[#c00000] hover:shadow-[0_0_12px_rgba(79,223,255,0.8)] transition-all disabled:opacity-50 disabled:cursor-not-allowed",
  secondary: "bg-[#5865f2] text-white font-medium hover:bg-[#4752c4] transition-colors disabled:opacity-50",
  danger: "text-[#ff3333] hover:bg-[#ff3333]/20 transition-colors",
  ghost: "bg-[#4e5058] text-white font-medium hover:bg-[#6d6f78] transition-colors",
  outline: "border-2 border-[#4fdfff]/50 text-[#4fdfff] hover:bg-[#4fdfff]/10 hover:border-[#4fdfff] transition-all",
  link: "text-[#4fdfff] hover:underline",
};

/**
 * Classes Tailwind pré-configurées pour les inputs
 */
export const inputClasses = "w-full p-3 rounded-lg border-2 border-[#4fdfff]/50 bg-[#1f1f1f] text-white placeholder-white/50 outline-none focus:border-[#4fdfff] focus:shadow-[0_0_10px_rgba(79,223,255,0.3)] transition-all";

