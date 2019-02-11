{pkgs ? import ./nix/pkgs.nix {}}:
pkgs.stdenv.mkDerivation {
    name = "mana";
    buildInputs = [
        pkgs.cargo
        pkgs.python3Packages.sphinx
    ];
    phases = ["installPhase"];
    installPhase = ''
        {
            echo 'This derivation is not for building, just for use with '
            echo 'nix-shell.'
        } 2>&1
        false
    '';
}
