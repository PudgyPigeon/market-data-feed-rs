self: super:

{
  secretscout = super.rustPlatform.buildRustPackage {
    pname = "SecretScout";
    version = "v0.0.1";

    src = super.fetchFromGitHub {
      owner = "PudgyPigeon";
      repo = "SecretScout";
      rev = "v0.0.1";
      sha256 = "sha256-M6Gqp8kIUUEt0EaQcHC4SXivdqA4U6xUjUpphAtizF0=";

    };
    # Add these to satisfy openssl-sys
    nativeBuildInputs = [ super.pkg-config ];
    buildInputs = [ super.openssl ];

    cargoHash = "sha256-rSpGS24WIXjZh7hfLvVjwhNF5/aEF9IgOoR/bQvc8GU=";

    cargoTestFlags = [ "--lib" ];

    meta = with super.lib; {
      description = "Secret scanning tool written in Rust";
      license = licenses.mit;
      maintainers = [ "globalbusinessadvisors" ];
    };
  };
} 