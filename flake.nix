{
  description = "shiko-prompt: An opinionated Rust-based zsh prompt builder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          
          mkShiko = theme: pkgs.rustPlatform.buildRustPackage {
            pname = "shiko";
            version = "0.0.1";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            SHIKO_THEME = theme;
          };
        in
        {
          default   = mkShiko "themes/default.json";
          gruvbox   = mkShiko "themes/gruvbox.json";
          kanagawa  = mkShiko "themes/kanagawa.json";
          duskfox   = mkShiko "themes/duskfox.json";
          campfire  = mkShiko "themes/campfire.json";
        }
      );

      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.shiko-prompt;
        in
        {
          options.programs.shiko-prompt = {
            enable = lib.mkEnableOption "shiko-prompt";
            theme = lib.mkOption {
              type = lib.types.str;
              default = "themes/default.json";
              description = "Theme path relative to the repository root.";
            };
          };

          config = lib.mkIf cfg.enable {
            home.packages = [
              (pkgs.rustPlatform.buildRustPackage {
                pname = "shiko";
                version = "0.0.1";
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
                SHIKO_THEME = cfg.theme;
              })
            ];

            programs.zsh.initContent = lib.mkAfter ''
              eval "$(shiko init)"
            '';
          };
        };
    };
}
