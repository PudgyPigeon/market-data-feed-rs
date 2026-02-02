let
  # 1. Import NixPkgs and Overlay custom packages we want to inject into shells as well
  nixpkgs = import ./nix/nixpkgs.nix;
  overlays = import ./nix/overlays/default.nix;
  pkgs = import nixpkgs { overlays = overlays; };
     
  # 2. Dependencies (System libs, Bevy deps)
  dependencies = import ./nix/packages.nix { inherit pkgs; };

  # 3. The App Derivation
  cargoPath = ./app;
  app = pkgs.callPackage ./nix/app.nix { inherit dependencies; cargoPath = cargoPath; };

  # 4. Automation (Pass pkgs and app here)
  # This imports the file and immediately calls the function with the args
  automation = import ./nix/automation.nix { inherit pkgs app dependencies; cargoPath = cargoPath; };
  
  # 5. The Shell
  # We pass dev-script explicitly from the automation set
  shell = pkgs.callPackage ./nix/shell.nix { inherit app dependencies; dev-script = automation.dev-script; };
in
{
  inherit app shell;
}