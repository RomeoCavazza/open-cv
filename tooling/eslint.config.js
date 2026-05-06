export default [
  {
    files: ["web/assets/js/**/*.js"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        alert: "readonly",
        console: "readonly",
        CustomEvent: "readonly",
        document: "readonly",
        EventTarget: "readonly",
        fetch: "readonly",
        FileReader: "readonly",
        localStorage: "readonly",
        Node: "readonly",
        Promise: "readonly",
        SVGElement: "readonly",
        URLSearchParams: "readonly",
        window: "readonly"
      }
    },
    rules: {}
  }
];
