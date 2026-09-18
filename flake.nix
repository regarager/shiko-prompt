{
  description = "Nix flake for shiko-prompt — an opinionated Rust-based zsh prompt builder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      shikoRev = "be0c878ccc56b8bd1c3fc9e623254e54c5210226";
    in
      flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          shikoSrc = pkgs.fetchFromGitHub {
            owner = "regarager";
            repo = "shiko-prompt";
            rev = shikoRev;
            hash = "sha256-1hyA1o1PfWn+kFh/apTEsNrGxmKJJkYOhQkdSQiCgUA=";
          };

          mkShiko = theme: pkgs.rustPlatform.buildRustPackage {
            pname = "shiko";
            version = "0.0.1";
            src = shikoSrc;

            cargoLock.lockFile = "${shikoSrc}/Cargo.lock";

            SHIKO_THEME = theme;
          };

          themes = {
            default   = mkShiko "themes/default.json";
            kanagawa  = mkShiko "themes/kanagawa.json";
            duskfox   = mkShiko "themes/duskfox.json";
            campfire  = mkShiko "themes/campfire.json";
          };
        in
          {
          packages = themes // {
            inherit (themes) default;
          };

          lib.mkShiko = mkShiko;
        }
      ) // {
      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.shiko-prompt;

          shikoSrc = pkgs.fetchFromGitHub {
            owner = "regarager";
            repo = "shiko-prompt";
            rev = "be0c878ccc56b8bd1c3fc9e623254e54c5210226";
            hash = "sha256-1hyA1o1PfWn+kFh/apTEsNrGxmKJJkYOhQkdSQiCgUA=";
          };

          mkShiko = theme: pkgs.rustPlatform.buildRustPackage {
            pname = "shiko";
            version = "0.0.1";
            src = shikoSrc;
            cargoLock.lockFile = "${shikoSrc}/Cargo.lock";
            SHIKO_THEME = theme;
          };
        in
          {
          options.programs.shiko-prompt = {
            enable = lib.mkEnableOption "shiko-prompt";

            theme = lib.mkOption {
              type = lib.types.str;
              default = "themes/default.json";
              description = ''
                Theme path relative to the shiko-prompt repository root.
                Available: themes/default.json, themes/kanagawa.json,
                themes/duskfox.json, themes/campfire.json.
              '';
            };

            rev = lib.mkOption {
              type = lib.types.str;
              default = "be0c878ccc56b8bd1c3fc9e623254e54c5210226";
              description = "Git revision of shiko-prompt to build.";
            };
          };

          config = lib.mkIf cfg.enable {
            home.packages = [ (mkShiko cfg.theme) ];

            programs.zsh.initContent = lib.mkAfter ''
              eval "$(shiko init)"
            '';
          };
        };
    };
}
