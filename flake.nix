{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = ["aarch64-darwin" "aarch64-linux" "armv6l-linux" "armv7l-linux" "x86_64-darwin" "x86_64-linux"];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in {
    checks = forAllSystems (system: {
      default = self.packages.${system}.default;
    });

    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      mdbook-catppuccin = pkgs.rustPlatform.buildRustPackage {
        pname = "mdbook-catppuccin";
        version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
        buildInputs = with pkgs; ([] ++ lib.optionals stdenv.isDarwin [libiconv]);
        src = pkgs.nix-gitignore.gitignoreSource [] ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      default = mdbook-catppuccin;
    });

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      default = pkgs.mkShell {
        buildInputs = with pkgs; (
          [
            cargo
            mdbook
            mdbook-admonish
            node2nix
            nodejs
          ]
          ++ lib.optionals stdenv.isDarwin [libiconv]
        );
      };
      demo = pkgs.mkShell {
        buildInputs = default.buildInputs ++ [self.packages.${system}.mdbook-catppuccin];
      };
    });
  };
}
