{
  description = "A minimalistic template for developing generic web applications in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
          in
          f pkgs
        );
    in
    {
      devShells = forAllSystems (pkgs:
        let
          # Use the exact toolchain from rust-toolchain.toml so the shell, the
          # Makefile, and CI all agree on the compiler version.
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            name = "template-rust-web-app-dev";

            packages = [
              rustToolchain
              pkgs.sqlx-cli
              pkgs.cargo-audit
              pkgs.nodejs_24
              pkgs.postgresql_18
              pkgs.docker-compose
              pkgs.gnumake
              pkgs.pre-commit
              pkgs.python3
              pkgs.uv
              pkgs.pkg-config
              pkgs.openssl
            ];

            # Matches the `db` service in docker-compose.yaml, so sqlx-cli and the
            # integration tests work in this shell without further setup.
            DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable";

            shellHook = ''
              echo "=========================================================="
              echo "  template-rust-web-app development environment"
              echo "  rust: $(rustc --version)"
              echo "  node: $(node --version)"
              echo "  psql: $(psql --version | cut -d' ' -f3)"
              echo ""
              echo "  make help       show the available targets"
              echo "  make docker-up  start PostgreSQL"
              echo "=========================================================="
            '';
          };
        });

      # `nixfmt` is the formatter that nixpkgs itself uses.
      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
