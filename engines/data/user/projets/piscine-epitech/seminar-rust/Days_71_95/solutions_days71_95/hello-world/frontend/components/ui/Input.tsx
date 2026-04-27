"use client";

import { InputHTMLAttributes, forwardRef } from "react";
import { inputClasses } from "@/lib/theme";

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helperText?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, helperText, className = "", ...props }, ref) => {
    const finalClasses = `${inputClasses} ${error ? "border-red-500 focus:border-red-500 focus:shadow-[0_0_10px_rgba(239,68,68,0.3)]" : ""} ${className}`.trim();

    return (
      <div className="w-full">
        {label && (
          <label className="block text-[10px] font-bold text-white/60 uppercase tracking-wider mb-1">
            {label}
          </label>
        )}
        <input
          ref={ref}
          className={finalClasses}
          {...props}
        />
        {helperText && !error && (
          <p className="text-white/40 text-xs mt-1">{helperText}</p>
        )}
        {error && (
          <p className="text-red-400 text-xs mt-1">{error}</p>
        )}
      </div>
    );
  }
);

Input.displayName = "Input";

export default Input;

