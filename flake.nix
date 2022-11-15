{
  description = "BTreeList";

  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {
        overlays = [rust-overlay.overlays.default];
        inherit system;
      };
      lib = pkgs.lib;
      rust = pkgs.rust-bin.stable.latest.default;
      craneLib = crane.lib.${system};
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          pname = "btreelist-deps";
        });
      clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
        });
      crate = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
      coverage = craneLib.cargoTarpaulin (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      packages = {
        default = crate;
        btreelist = crate;
        clippy = clippy;
        coverage = coverage;
      };

      checks = {
        btreelist = self.packages.${system}.btreelist;
        clippy = clippy;
        coverage = coverage;
      };

      formatter = pkgs.alejandra;

      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          (rust.override {
            extensions = ["rust-src"];
          })
          cargo-edit
          cargo-watch
          cargo-criterion
          cargo-fuzz
          cargo-flamegraph

          rnix-lsp
          nixpkgs-fmt
        ];
      };
    });
}
