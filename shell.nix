let
    pkgs = import <nixpkgs-19.03> {};
    rust = import ./nixpkgs/rust-1.33.nix {
      inherit (pkgs.stdenv) mkDerivation;
      inherit (pkgs) fetchurl stdenv patchelf;
    };
in pkgs.stdenv.mkDerivation {
    name = "emseries";

    buildInputs = [ pkgs.carnix
                    rust
                  ];
}
