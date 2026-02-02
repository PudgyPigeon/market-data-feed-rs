{ pkgs, ... }:

let 
  # Change "latest" or "1.80.0" for specific version etc
  rustToolChain = pkgs.rust-bin.stable."1.93.0".default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
    targets = [ "x86_64-unknown-linux-gnu" ];
  };

  appBuild = [];

  nativeBuild = [
    pkgs.pkg-config
    rustToolChain
  ];
in
{
  inherit appBuild nativeBuild rustToolChain;
}
