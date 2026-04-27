"use client";

import { ButtonHTMLAttributes, ReactNode } from "react";
import { buttonVariants } from "@/lib/theme";

type ButtonVariant = "primary" | "secondary" | "danger" | "ghost" | "outline" | "link";
type ButtonSize = "sm" | "md" | "lg";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  children: ReactNode;
  isLoading?: boolean;
  fullWidth?: boolean;
}

const sizeClasses: Record<ButtonSize, string> = {
  sm: "px-3 py-1.5 text-xs",
  md: "px-4 py-2 text-sm",
  lg: "p-3 text-base",
};

export default function Button({
  variant = "primary",
  size = "md",
  children,
  isLoading = false,
  fullWidth = false,
  className = "",
  disabled,
  ...props
}: ButtonProps) {
  const baseClasses = "rounded-lg font-bold cursor-pointer transition-all flex items-center justify-center gap-2";
  const variantClass = buttonVariants[variant];
  const sizeClass = sizeClasses[size];
  const widthClass = fullWidth ? "w-full" : "";
  
  const finalClasses = `${baseClasses} ${variantClass} ${sizeClass} ${widthClass} ${className}`.trim();

  return (
    <button
      className={finalClasses}
      disabled={disabled || isLoading}
      {...props}
    >
      {isLoading ? (
        <span className="flex items-center justify-center gap-2">
          <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          Chargement...
        </span>
      ) : (
        children
      )}
    </button>
  );
}

