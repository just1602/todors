{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rust-analyzer
    pkgs.rustup
  ];
}
