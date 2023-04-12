{
  description = "Development environment for System Initiative";

  # Flake inputs
  inputs = {
    # rust-overlay is designed to work with nixos-unstable
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  # Flake outputs
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)

          (self: super: {
            rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      let
        sharedBuildInputs = [
          automake
          bash
          coreutils
          git
          gnumake
        ];
        sharedDepsTargetTarget = [ ];

        rustBuildInputs = [
          gcc
          libtool
          lld
          openssl
          pkg-config
          postgresql_14
          protobuf
          rustToolchain
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          libiconv
          darwin.apple_sdk.frameworks.Security
        ];

        # Council specific dependencies
        councilBuildInputs = [
        ] ++ rustBuildInputs;
        councilDepsTargetTarget = [ ];

        # Pinga specific dependencies
        pingaBuildInputs = [
        ] ++ rustBuildInputs;
        pingaDepsTargetTarget = [ ];

        # SDF specific dependencies
        sdfBuildInputs = [
        ] ++ rustBuildInputs;
        sdfDepsTargetTarget = [ ];

        # Veritech specific dependencies
        veritechBuildInputs = [
          nodejs-18_x
          nodePackages.pnpm
          nodePackages.typescript
        ] ++ rustBuildInputs;
        veritechDepsTargetTarget = [
          awscli
          butane
          kubeval
          skopeo
        ];

        # Web specific dependencies 
        webBuildInputs = [
          nodejs-18_x
          nodePackages.pnpm
          nodePackages.typescript
        ];
        webDepsTargetTarget = [ ];
      in
      with stdenv;
      {
        packages.council = mkDerivation {
          name = "council";
          buildInputs = councilBuildInputs;
          depsTargetTarget = councilDepsTargetTarget;
          src = self;
        };

        packages.pinga = mkDerivation {
          name = "pinga";
          buildInputs = pingaBuildInputs;
          depsTargetTarget = pingaDepsTargetTarget;
        };

        packages.sdf = mkDerivation {
          name = "sdf";
          buildInputs = sdfBuildInputs;
          depsTargetTarget = sdfDepsTargetTarget;
          src = self;
        };

        packages.veritech = mkDerivation {
          name = "veritech";
          buildInputs = veritechBuildInputs;
          depsTargetTarget = veritechDepsTargetTarget;
          src = self;
        };

        devShells.default = mkShell {
          buildInputs = [
            docker-compose
            jq
            pgcli
            nodePackages.typescript-language-server
            (rustToolchain.override {
              # This really should be augmenting the extensions, instead of
              # completely overriding them, but since we're not setting up
              # any extensions in our rust-toolchain file, it should be
              # fine for now.
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ] ++ sharedBuildInputs
          ++ councilBuildInputs
          ++ pingaBuildInputs
          ++ sdfBuildInputs
          ++ veritechBuildInputs
          ++ webBuildInputs;
          depsTargetTarget = sharedDepsTargetTarget
            ++ councilDepsTargetTarget
            ++ pingaDepsTargetTarget
            ++ sdfDepsTargetTarget
            ++ veritechDepsTargetTarget
            ++ webDepsTargetTarget;
          # This is awful, but necessary (until we find a better way) to
          # be able to `cargo run` anything that compiles against
          # openssl. Without this, ld is unable to find libssl.so.3 and
          # libcrypto.so.3.
          #
          # If we were packaging this up as a flake, instead of only
          # using nix for the development environment, we'd be using
          # wrapProgram with something like
          # `--prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [ openssl ]}`
          # to make sure the things we're compiling are always using the
          # version of openssl they were compiled against.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
        };
      }
    );
}
