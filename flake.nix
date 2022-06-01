{
  description = "sequence-tree";

  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            overlays = [ rust-overlay.overlay ];
            inherit system;
          };
          lib = pkgs.lib;
          rust = pkgs.rust-bin.stable.latest.default;
          cargoNix = import ./Cargo.nix {
            inherit pkgs;
            release = true;
          };
          debugCargoNix = import ./Cargo.nix {
            inherit pkgs;
            release = false;
          };
        in
        {
          packages = lib.attrsets.mapAttrs
            (name: value: value.build)
            cargoNix.workspaceMembers;

          defaultPackage = self.packages.${system}.sequence-tree;

          checks = lib.attrsets.mapAttrs
            (name: value: value.build.override {
              runTests = true;
            })
            debugCargoNix.workspaceMembers;

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              (rust.override {
                extensions = [ "rust-src" ];
              })
              cargo-edit
              cargo-watch
              cargo-criterion
              cargo-fuzz
              cargo-flamegraph
              crate2nix

              rnix-lsp
              nixpkgs-fmt
            ];
          };
        });
}
