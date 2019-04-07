with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "git-ignore";
  buildInputs = with pkgs; [
    pkgconfig
    openssl
  ];
}
