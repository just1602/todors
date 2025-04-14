{
  description = "A todo CLI app that use the todo.txt format under the hood";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = pkgs.callPackage nix/package.nix { };
        };

        legacyPackages = {
          default = pkgs.callPackage nix/package.nix { };
        };

        devShells = {
          default = pkgs.mkShell {
            packages = [
              pkgs.rustc
              pkgs.cargo
              pkgs.rust-analyzer
            ];
          };
        };
      }
    )
    // {
      overlays.default = final: prev: {
        todors = prev.pkgs.callPackage nix/package.nix { };
      };
    };
}
