let
unstable = import
    (builtins.fetchTarball https://github.com/nixos/nixpkgs/tarball/87645e0250aec3b64cace2858fbebc6265e7aa04)
    # reuse the current configuration
  {  };
in { nixpkgs ? import <nixpkgs> { } }:
with nixpkgs;
mkShell { buildInputs = [ unstable.nodejs-15_x nixfmt rustup  ]; }
