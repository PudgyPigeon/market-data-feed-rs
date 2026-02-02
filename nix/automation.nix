{ pkgs, app, dependencies, cargoPath, ... }:

let 
  rustToolChain = dependencies.rustToolChain;

  setup-env = ''
    set -e 
    # --- ANSI Color Codes (Enhanced) ---
    # Regular
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'

    # Bold (Better for Headers)
    BOLD='\033[1m'
    B_WHITE='\033[1;37m'
    B_CYAN='\033[1;36m'
    B_GREEN='\033[1;32m'

    # Functional Colors
    GRAY='\033[0;90m'
    NC='\033[0m' # No Color
    
    START_TIME=$SECONDS
    trap 'echo -e "\n💥 ''${RED}Pipeline failed after $(( SECONDS - START_TIME ))s''${NC}"; exit 1' ERR
  '';

  common-ci = ''    
    echo -e "''${YELLOW}▶ Starting Local CI Pipeline...''${NC}"

    echo -e "🎨 ''${B_CYAN}[1/5] Running rustfmt...''${NC}"
    ${rustToolChain}/bin/cargo-fmt fmt --manifest-path ${toString cargoPath}/Cargo.toml

    echo -e "🏗️ ''${B_CYAN}[2/5] Running Clippy...''${NC}"
    ${rustToolChain}/bin/cargo-clippy clippy --manifest-path ${toString cargoPath}/Cargo.toml -- -D warnings

    if [ "$ENV" = "dev" ]; then
        echo -e "🧪 ''${B_CYAN}[3/5] Skipping tests in dev environment...''${NC}"
      else
        echo -e "🧪 ''${B_CYAN}[3/5] Running Unit Tests...''${NC}"
        ${rustToolChain}/bin/cargo test --manifest-path ${toString cargoPath}/Cargo.toml
      fi

    echo -e "🔍 ''${B_CYAN}[4/5] Running SecretScout...''${NC}"
    secretscout protect
    secretscout detect

    ELAPSED=$(( SECONDS - START_TIME ))
    echo -e "✅ ''${GREEN}CI Passed in ''${ELAPSED}s''${NC}"
  '';

  dev-script = pkgs.writeShellScriptBin "dev" ''
    echo "--------------------------------------------------"
    echo "🎮 MARKET DATA FEED: LOCAL CI & DEV RUNNER"
    echo "--------------------------------------------------"
    ENV="dev"
    ${setup-env}
    ${common-ci}
    echo -e "🚀 ''${B_GREEN}[5/5] Launching Bevy App...''${NC}"
    ${rustToolChain}/bin/cargo run \
        --manifest-path Cargo.toml \
        -- "$@"
  '';

in
{
  inherit dev-script;
}
