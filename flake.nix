{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      craneLib = crane.lib.${system};
      mdBook-catppuccin = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        buildInputs = [] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
    in {
      checks = {
        inherit mdBook-catppuccin;
      };

      packages.default = mdBook-catppuccin;
      # TODO: nix build doesn't work atm since node/sass are not packaged too
      apps.default = flake-utils.lib.mkApp {
        drv = mdBook-catppuccin;
      };

      devShells.default = pkgs.mkShell {
        name = "rust-shell";
        inputsFrom = builtins.attrValues self.checks.${system};
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          nodejs
          sass
          mdbook
        ];
      };
    });
}
