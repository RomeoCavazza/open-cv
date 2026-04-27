import { FlatCompat } from "@eslint/eslintrc";

const compat = new FlatCompat();

export default [
  {
    ignores: ["node_modules/**", ".next/**", "out/**", "public/**"],
  },
  ...compat.config({
    extends: ["next"],
    rules: {
      "@next/next/no-html-link-for-pages": "off",
    },
  }),
];
