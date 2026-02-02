{ pkgs, app, dependencies, dev-script, ... }:

pkgs.mkShell rec {
  # Tools that run ON the host
  nativeBuildInputs = dependencies.nativeBuild ++ [
    dev-script
    pkgs.cargo-watch
    pkgs.secretscout 
  ];

  # Libraries that get linked INTO the app
  buildInputs = dependencies.appBuild;
  
  shellHook = ''
      echo "--------------------------------------------------"
      echo "✅ Development Environment Loaded"
      echo "💡 Type 'dev' to run your Local CI + App"
      echo "--------------------------------------------------"
    '';
}

