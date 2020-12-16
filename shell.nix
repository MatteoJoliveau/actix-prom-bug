with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "actix-prom-shell";
    buildInputs = with pkgs; [
      pkgconfig
      openssl.dev
    ];
}
