{ inputs, lib, config, ... } @ args:
let
  pkgs = import inputs.nixpkgs { 
    system = args.pkgs.stdenv.system; 
  };
  unstable = import inputs.unstable {
    system = args.pkgs.stdenv.system; 
  };
in {
  devcontainer.enable = true;

  languages.nix.enable = true;
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ 
      "rustc" 
      "cargo" 
      "cargo-watch" 
      "clippy" 
      "rustfmt" 
      "rust-analyzer" 
    ];
    toolchain.cargo-watch = pkgs.cargo-watch;
  };

  packages = [
    unstable.lldb
  ];
}