{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = ["aarch64-darwin" "aarch64-linux" "armv6l-linux" "armv7l-linux" "x86_64-darwin" "x86_64-linux"];
    version = builtins.substring 0 8 self.lastModifiedDate;
    inherit (nixpkgs) lib;
    forAllSystems = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
  in {
    checks = forAllSystems (pkgs: {
      default = self.packages.${pkgs.system}.default;
    });

    packages = forAllSystems (pkgs: rec {
      mdbook-catppuccin = pkgs.rustPlatform.buildRustPackage {
        pname = "mdbook-catppuccin";
        inherit version;
        src = pkgs.nix-gitignore.gitignoreSource [] ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      default = mdbook-catppuccin;
    });

    devShells = forAllSystems (pkgs: rec {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          gcc
          rustfmt
          clippy
          rust-analyzer
          # mdbook
          mdbook-admonish
          node2nix
          nodejs
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
      demo = pkgs.mkShell {
        buildInputs = default.buildInputs ++ [self.packages.${pkgs.system}.mdbook-catppuccin];
      };
    });
  };

  nixConfig = {
    extra-substituters = ["https://catppuccin.cachix.org"];
    extra-trusted-public-keys = ["catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU="];
  };
}
