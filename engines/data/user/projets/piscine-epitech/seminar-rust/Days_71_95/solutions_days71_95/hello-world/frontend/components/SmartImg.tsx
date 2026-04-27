"use client";

import React from "react";

type SmartImgProps = React.ImgHTMLAttributes<HTMLImageElement> & {
  src: string;
  fallbackSrc?: string;
};

/**
 * Img robuste: tente `src`, puis bascule sur `fallbackSrc` si le chargement échoue.
 * Utile pendant une transition (ex: WebP → PNG fallback).
 */
export default function SmartImg({ src, fallbackSrc, onError, ...props }: SmartImgProps) {
  const [currentSrc, setCurrentSrc] = React.useState(src);

  // Si le src change (re-render), on réinitialise la source courante.
  React.useEffect(() => {
    setCurrentSrc(src);
  }, [src]);

  return (
    <img
      {...props}
      src={currentSrc}
      onError={(e) => {
        if (fallbackSrc && currentSrc !== fallbackSrc) {
          setCurrentSrc(fallbackSrc);
        }
        onError?.(e);
      }}
    />
  );
}


