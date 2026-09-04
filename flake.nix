{
  description = "A haskell.nix project";
  inputs.haskellNix.url = "github:input-output-hk/haskell.nix";
  inputs.nixpkgs.follows = "haskellNix/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils, haskellNix }:
    let
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    in
      flake-utils.lib.eachSystem supportedSystems (system:
      let
        overlays = [ haskellNix.overlay
          (final: _prev: {
            # This overlay adds our project to pkgs
            hixProject =
              final.haskell-nix.hix.project {
                src = ./.;
                evalSystem = "aarch64-darwin";
                shell = {
                  tools = {
                    haskell-language-server = {};
                  };
                  buildInputs = with final.elmPackages; [
                    elm
                    elm-format
                    elm-language-server
                  ];
                };
              };
          })
                   ];

        pkgs = import nixpkgs { inherit system overlays; inherit (haskellNix) config; };
        flake = pkgs.hixProject.flake {};

        frontend = pkgs.buildNpmPackage {
          pname = "frontend";
          version = "0.1.0";
          src = ./frontend;
          npmDepsHash = "sha256-ISmmvj/aBSrMEJ6fbHP4cGN24JjEHEdqfXlh272HHXk=";
          nativeBuildInputs = [ pkgs.elmPackages.elm ];
          installPhase = ''
            mkdir -p $out/dist
            cp -r dist/* $out/dist/
          '';
        };

      in flake // {
        legacyPackages = pkgs;

        # `nix build .` builds the crewd executable; the per-component
        # attributes (e.g. `nix build .#crewd:exe:crewd`) come from `flake`.
        packages = flake.packages // {
          inherit frontend;
          default = flake.packages."crewd:exe:crewd";
        };
      });

  # --- Flake Local Nix Configuration ----------------------------
  nixConfig = {
    # This sets the flake to use the IOG nix cache.
    # Nix should ask for permission before using it,
    # but remove it here if you do not want it to.
    #extra-substituters = ["https://cache.iog.io"];
    #extra-trusted-public-keys = ["hydra.iohk.io:f/Ea+s+dFdN+3Y/G+FDgSq+a5NEWhJGzdjvKNGv0/EQ="];
    allow-import-from-derivation = "true";
  };
}
