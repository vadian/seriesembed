let
    pkgs = import <nixpkgs-18.09> {};
    unstable = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation {
    name = "emseries";

    buildInputs = [ pkgs.rustc
                    pkgs.cargo
                    pkgs.rustfmt
                    unstable.carnix
                  ];
}
