# To update sha256 later: nix-prefetch-url --unpack https://github.com/NixOS/nixpkgs/archive/be5afa0fcb31f0a96bf9ecba05a516c66fcd8114.tar.gz
# - change hash in URL when running command 
builtins.fetchTarball {
  url = "https://github.com/NixOS/nixpkgs/archive/be5afa0fcb31f0a96bf9ecba05a516c66fcd8114.tar.gz";
  sha256 = "0jm942f32ih264hmna7rhjn8964sid0sn53jwrpc73s2vyvqs7kc";
}