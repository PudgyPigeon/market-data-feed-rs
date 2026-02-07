{ pkgs, app, dependencies, automation, ... }:

pkgs.mkShell rec {
  # Tools that run ON the host
  nativeBuildInputs = dependencies.nativeBuild ++ [
    automation.dev-script
    automation.release-script
    pkgs.cargo-watch
    pkgs.secretscout 
    pkgs.perf
    pkgs.stress-ng
  ];

  # Libraries that get linked INTO the app
  buildInputs = dependencies.appBuild;
  
  shellHook = ''
    # --- System Paths ---
    export PKG_CONFIG_PATH="${pkgs.libpcap}/lib/pkgconfig"
    export LD_LIBRARY_PATH="${pkgs.libpcap}/lib:$LD_LIBRARY_PATH"

    # --- ANSI Color Codes ---
    NC='\033[0m'
    CYAN='\033[1;36m'
    WHITE='\033[1;37m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[1;34m'
    PURPLE='\033[1;35m'
    GRAY='\033[0;90m'

    # --- Combined Banner ---
    echo -e "''${CYAN}--------------------------------------------------''${NC}"
    echo -e "  ''${WHITE}🚀 HFT ENGINE ACTIVATED: MARKET-DATA-FEED''${NC}"
    echo -e "''${CYAN}--------------------------------------------------''${NC}"
    echo -e "  ✅ ''${GREEN}Environment Environment Loaded''${NC}"
    echo -e "  📦 ''${GREEN}Pcap & Deps:''${NC}  ''${YELLOW}MAX OPTIMIZED''${NC} 🏎️"
    echo -e "  🛠️  ''${BLUE}Dev Mode:''${NC}     Lvl 1 (Fast Compile) ⚡"
    echo -e "  💎 ''${PURPLE}Release:''${NC}      LTO Fat + Abort (Final) 🔥"
    echo -e ""
    echo -e "  ''${GRAY}Type''${NC} ''${WHITE}'dev' ''${NC}''${GRAY}to run Local CI + App''${NC}"
    echo -e "  ''${GRAY}Type''${NC} ''${WHITE}'release' ''${NC}''${GRAY}to run Production Build''${NC}"
    echo -e "''${CYAN}--------------------------------------------------''${NC}"
  '';
}

