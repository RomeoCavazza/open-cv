{
  description = "TCO Core - Acquisition de Données Multi-Capteurs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ];
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          pythonEnv = pkgs.python3.withPackages (ps: with ps; [
            pyserial
            influxdb
            numpy
            pandas
            matplotlib
            scipy
          ]);
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              pythonEnv
              arduino-cli
            ];

            shellHook = ''
              echo "🔬 Environnement TCO Core activé"
              echo "Python: $(python3 --version)"
              echo "Arduino CLI: $(arduino-cli version 2>/dev/null || echo 'non disponible')"
              echo ""
              echo "Packages Python disponibles:"
              python3 -c "
                packages = ['serial', 'influxdb', 'numpy', 'pandas', 'matplotlib', 'scipy']
                for pkg in packages:
                  try:
                    __import__(pkg)
                    print(f'  ✓ {pkg}')
                  except ImportError:
                    print(f'  ✗ {pkg} (manquant)')
              " 2>/dev/null || echo "  ⚠ Vérification échouée"
            '';
          };
        });
    };
}
