# let
# in { nixpkgs ? import <nixpkgs> { } }:
# with nixpkgs;
# mkShell { buildInputs = [ unstable.nodejs-15_x nixfmt rustup  ]; }

let
  tarballPath =
    "https://github.com/cpcloud/nixpkgs-mozilla/archive/install-docs-optional.tar.gz";
  moz_overlay = import (builtins.fetchTarball tarballPath);
  # moz_overlay = import (builtins.fetchTarball
  #   "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  # unstable = import (builtins.fetchTarball
  #   "https://github.com/nixos/nixpkgs/tarball/87645e0250aec3b64cace2858fbebc6265e7aa04")
  #   { };
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust = (pkgs.rustChannelOf {
    date = "2021-03-25";
    channel = "stable";
    installDoc = false;
  }).rust.override { extensions = [ "rust-src" "rustfmt-preview" ]; };
in with pkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    nixfmt

    pkgs.nodejs-15_x
#    rust
    pkgconfig
    openssl
    # to use a specific nighly:
    # to use the project's rust-toolchain file:
  ];
}
