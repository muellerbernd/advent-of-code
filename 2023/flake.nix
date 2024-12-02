{
  description = "AOC Rust devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        # Rust package
        devShells.default = mkShell {
          name = "AOC rust devShell";

          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer
            rustfmt
            cargo
            clippy
          ];

          shellHook = "";
        };
      });

}
