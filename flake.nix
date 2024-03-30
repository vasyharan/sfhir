{
  inputs = {
    unstable.url= "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, unstable, fenix, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # rust-toolchain = (pkgs.fenix.stable.withComponents [
        #               "rustc" 
        #               "cargo" 
        #               # "cargo-watch" 
        #               "clippy" 
        #               "rustfmt" 
        #               "rust-analyzer" 
        # ]);
        rust-toolchain = pkgs.fenix.stable.toolchain;
      in {
      devShells.default = pkgs.mkShell {
        buildInputs = [ rust-toolchain ];
      };
    });
}
