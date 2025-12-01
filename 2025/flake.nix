{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        let
          aocf = pkgs.stdenv.mkDerivation rec {
            pname = "aocf";
            version = "0.1.21";

            src = pkgs.fetchFromGitHub {
              owner = "nuxeh";
              repo = "aocf";
              rev = "e057187c5b9931ab97672f690ff102cad848c761";
              sha256 = "sha256-u+1dPSc3jVyJukWkOLhi4hiBRDbcemD8V8pCw3xFrko=";
            };

            nativeBuildInputs = [
              pkgs.rustToolchain
            ];

            CARGO_HTTP_DEBUG = "true";
            CARGO_HTTP_MULTIPLEXING = "false";
            CARGO_NET_GIT_FETCH_WITH_CLI = "true";
            CARGO_LOG = "network=trace";

            buildPhase = ''
              env | grep "CARGO"
              cargo build --release --locked
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp target/release/${pname} $out/bin/
            '';
          };
        in
        {
          default = pkgs.mkShellNoCC {
            packages = with pkgs; [
              rustToolchain
              openssl
              pkg-config
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer
              dasel
              aoc-cli
            ];

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );
    };
}
