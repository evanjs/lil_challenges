let
  nixpkgs = import ./nix/sources.nix {};
  oxalica = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz") {};
  latestNightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    extensions = [ "rust-src" "rust-std" "rustfmt-preview" "clippy-preview" ];
  });
  latestStable = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-std" "rustfmt-preview" "clippy-preview" ];
  };
  pkgs = import <nixpkgs> { overlays = [
    (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ]; };
in
  with pkgs;

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
      latestNightly
      bashInteractive
    ];
  }
