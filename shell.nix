# nix requires using nix-shell since native-tls requires pkg-config to
# configure openssl
#
# See also:
#    https://nixos.wiki/wiki/C#pkg-config
#    https://github.com/NixOS/nixpkgs/issues/64530
with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "mastodon-async";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}