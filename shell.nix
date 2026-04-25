{ pkgs ? import <nixpkgs> {} }:

let
  python = pkgs.python3.withPackages (ps: [
    ps.beautifulsoup4
    ps.markdownify
    ps.pyyaml
    ps."curl-cffi"
  ]);
in
pkgs.mkShell {
  packages = [
    python
    pkgs.jq
  ];

  shellHook = ''
    export SCRAPER_CONFIG="$PWD/config/scrape-offres.yaml"

    echo "Environnement scraper prêt."
    echo "Config: $SCRAPER_CONFIG"
    echo "Lancer: python scripts/scrape_offres.py --config \"$SCRAPER_CONFIG\" --overwrite"
  '';
}
