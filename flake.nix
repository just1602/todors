{
  description = "A todo CLI app that use the todo.txt format under the hood";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
