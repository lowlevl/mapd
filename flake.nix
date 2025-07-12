{
  description = "A devshell for the `mapd` project";

  nixConfig = {
    extra-substituters = ["https://nix-community.cachix.org"];
    extra-trusted-public-keys = ["nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    systems,
    treefmt-nix,
    fenix,
  }: let
    # Small tools to iterate over each systems and nixpkgs
    eachSystem = f: nixpkgs.lib.genAttrs (import systems) f;
    eachSystemPkgs = f: eachSystem (system: f nixpkgs.legacyPackages.${system});

    # Eval the treefmt modules from ./treefmt.nix
    treefmt = pkgs: (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper;
    treefmt-check = pkgs: (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.check;
  in {
    formatter = eachSystemPkgs treefmt;
    checks = eachSystemPkgs (pkgs: {
      formatting = treefmt-check pkgs self;
    });

    devShells = eachSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };
    in {
      default = pkgs.mkShell {
        buildInputs = let
          channel = pkgs.fenix.stable.withComponents [
            "cargo"
            "rustc"
            "clippy"
            "rustfmt"
            "rust-src"
            "rust-std"
            "rust-analyzer"
          ];

          toolchain = pkgs.fenix.combine [
            channel
            pkgs.fenix.targets.wasm32-unknown-unknown.stable.rust-std
          ];
        in [
          toolchain

          pkgs.cargo-leptos
          pkgs.binaryen
          pkgs.wasm-pack
        ];
      };
    });
  };
}
