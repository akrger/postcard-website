# let
# in { nixpkgs ? import <nixpkgs> { } }:
# with nixpkgs;
# mkShell { buildInputs = [ unstable.nodejs-15_x nixfmt rustup  ]; }

let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  unstable = import (builtins.fetchTarball
    "https://github.com/nixos/nixpkgs/tarball/87645e0250aec3b64cace2858fbebc6265e7aa04")
    { };
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust = (nixpkgs.rustChannelOf {
    date = "2020-12-31";
    channel = "stable";
  }).rust.override { extensions = [ "rust-src" ]; };
in with nixpkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    nixfmt
    unstable.nodejs-15_x
    rust
    # to use a specific nighly:
    # to use the project's rust-toolchain file:
  ];
}
