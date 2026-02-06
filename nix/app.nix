{ pkgs, dependencies, cargoPath, ... }:

let
    rustPlatform = pkgs.makeRustPlatform {
      cargo = dependencies.rustToolChain;
      rustc = dependencies.rustToolChain;
    };
in
  pkgs.rustPlatform.buildRustPackage {
    pname = "market-data-feed-rs";
    version = "0.0.1";

    nativeBuildInputs = dependencies.nativeBuild;
    buildInputs = dependencies.appBuild;

    src = cargoPath;
    cargoLock.lockFile = "${cargoPath}/Cargo.lock";
  }   