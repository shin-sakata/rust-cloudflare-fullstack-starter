{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import ./nix/overlays/worker-build.nix) ];
          };
          rust-toolchain = inputs.fenix.packages.${system}.combine [
            (inputs.fenix.packages.${system}.stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            inputs.fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            buildInputs = [
              rust-toolchain
              pkgs.wrangler
              pkgs.worker-build
              pkgs.trunk
              pkgs.process-compose
            ];
          };
        };
    };
}
