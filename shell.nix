let
    pkgs = import <nixpkgs-19.03> {};
    ld = import <luminescent-dreams> {};
in pkgs.mkShell {
    name = "emseries";

    buildInputs = [ ld.rust_1_39 ];

    RUST_BACKTRACE = "full";

    shellHook = ''if [ -e ~/.nixpkgs/shellhook.sh ]; then . ~/.nixpkgs/shellhook.sh; fi'';
}
