[
  # External Rust overlay
  (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  # Forked SecretScout overlay
  (import ./secretscout.nix)
]