{
  description = "alternance — builder de candidatures IA-native";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        # Postgres with pgvector pre-loaded
        pgWithVector = pkgs.postgresql_16.withPackages (p: [ p.pgvector ]);
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain

            # Database
            pgWithVector
            sqlx-cli

            # PDF generation
            typst

            # Headless browser support (for chromiumoxide later)
            chromium
            nodejs

            # Build deps for some crates
            pkg-config
            openssl

            # Quality of life
            cargo-watch
            cargo-edit
            cargo-nextest
            cargo-bloat
            cargo-deny
            cargo-udeps
            tokei
            jq
            just
          ];

          shellHook = ''
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.openssl.dev}/share/pkgconfig"
            export DATABASE_URL="''${DATABASE_URL:-postgres://alternance:alternance@localhost:5432/alternance}"
            export PGDATA="''${PGDATA:-$PWD/.pg}"
            export PGHOST="''${PGHOST:-$PWD/.pg}"
            export PGPORT="''${PGPORT:-5432}"
            export RUST_LOG="''${RUST_LOG:-info,sqlx=warn,hyper=warn}"

            echo ""
            echo "  alternance dev shell ready"
            echo ""
            echo "  Useful commands:"
            echo "    just db-init      # init local Postgres (first time)"
            echo "    just db-up        # start Postgres"
            echo "    just db-down      # stop Postgres"
            echo "    just migrate      # run migrations"
            echo "    just dev          # cargo watch -x 'run -p api'"
            echo ""
          '';
        };
      });
}
