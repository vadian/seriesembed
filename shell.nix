let
    pkgs = import <nixpkgs-18.09> {};
    unstable = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation {
    name = "emseries";

    buildInputs = [ pkgs.rustc
                    pkgs.cargo
                    unstable.carnix
                  ];

    shellHook = ''
        export PS1="[$name] \[$txtgrn\]\u@\h\[$txtwht\]:\[$bldpur\]\w \[$txtcyn\]\$git_branch\[$txtred\]\$git_dirty \[$bldylw\]\$aws_env\[$txtrst\]\$ "
    '';
}
