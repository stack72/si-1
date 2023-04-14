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
          jq
          gnused
        ];
        sharedNativeBuildInputs = [ ];
        sharedDepsTargetTarget = [ ];
        sharedCheckInputs = [ ];

        rustNativeBuildInputs = [
          rustPlatform.cargoSetupHook
        ];
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
        ] ++ sharedBuildInputs ++ rustBuildInputs;
        councilNativeBuildInputs = [
        ] ++ sharedNativeBuildInputs ++ rustNativeBuildInputs;
        councilDepsTargetTarget = [ ];
        councilCheckInputs = [ ] ++ sharedCheckInputs;

        # Pinga specific dependencies
        pingaBuildInputs = [
        ] ++ sharedBuildInputs ++ rustBuildInputs;
        pingaNativeBuildInputs = [
        ] ++ sharedNativeBuildInputs ++ rustNativeBuildInputs;
        pingaDepsTargetTarget = [ ];
        pingaCheckInputs = [ ] ++ sharedCheckInputs;

        # SDF specific dependencies
        sdfBuildInputs = [
        ] ++ sharedBuildInputs ++ rustBuildInputs;
        sdfNativeBuildInputs = [
        ] ++ sharedNativeBuildInputs ++ rustNativeBuildInputs;
        sdfDepsTargetTarget = [ ];
        sdfCheckInputs = [ ] ++ sharedCheckInputs;

        # Veritech specific dependencies
        veritechBuildInputs = [
          nodejs-18_x
          nodePackages.pnpm
          nodePackages.typescript
        ] ++ sharedBuildInputs ++ rustBuildInputs;
        veritechNativeBuildInputs = [
        ] ++ sharedNativeBuildInputs ++ rustNativeBuildInputs;
        veritechDepsTargetTarget = [
          awscli
          butane
          kubeval
          skopeo
        ];
        veritechCheckInputs = [ ] ++ sharedCheckInputs ++ veritechDepsTargetTarget;

        # Web specific dependencies 
        webBuildInputs = [
          nodejs-18_x
          nodePackages.pnpm
          nodePackages.typescript
        ] ++ sharedBuildInputs;
        webNativeBuildInputs = [
        ] ++ sharedNativeBuildInputs;
        webDepsTargetTarget = [ ];
        webCheckInputs = [ ] ++ sharedCheckInputs;

        rootCargoLockInfo = {
          lockFile = ./Cargo.lock;
          # Have to specify hashes for git sources in the Cargo.toml.
          # Use `lib.fakeHash` to find out what the hash should be.
          outputHashes = {
            "hyperlocal-0.8.0" = "sha256-iEvEKJ/tkF+YaiCMpU3peC1dYZZHihUdAL5xaF3pIPo=";
          };
        };

        fixMakefilePaths = ''
          sed -i -e 's#/usr/bin/env#${coreutils}/bin/env#' Makefile
        '';
      in
      with stdenv;
      {
        packages.council = mkDerivation (finalAttrs: {
          name = "council";
          src = ./.;
          buildInputs = councilBuildInputs;
          nativeBuildInputs = councilNativeBuildInputs;
          depsTargetTarget = councilDepsTargetTarget;
          checkInputs = councilCheckInputs;
          cargoDeps = rustPlatform.importCargoLock rootCargoLockInfo;
          doCheck = false;
          patchPhase = ''
            ${fixMakefilePaths}
          '';
          buildPhase = ''
            make VERGEN_GIT_SHA="$(cat ./GIT_REVISION)" build//bin/council
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/debug/council $out/bin/.
          '';
          checkPhase = ''
            echo "Do something for the checkPhase?"
          '';
        });

        packages.pinga = mkDerivation (finalAttrs: {
          name = "pinga";
          src = ./.;
          buildInputs = pingaBuildInputs;
          nativeBuildInputs = pingaNativeBuildInputs;
          depsTargetTarget = pingaDepsTargetTarget;
          checkInputs = pingaCheckInputs;
          cargoDeps = rustPlatform.importCargoLock rootCargoLockInfo;
          doCheck = false;
          patchPhase = ''
            ${fixMakefilePaths}
          '';
          buildPhase = ''
            make VERGEN_GIT_SHA="$(cat ./GIT_REVISION)" build//bin/pinga
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/debug/pinga $out/bin/.
          '';
          checkPhase = ''
            echo "Do something for the checkPhase?"
          '';
        });

        packages.sdf = mkDerivation (finalAttrs: {
          name = "sdf";
          src = ./.;
          buildInputs = sdfBuildInputs;
          nativeBuildInputs = sdfNativeBuildInputs;
          depsTargetTarget = sdfDepsTargetTarget;
          checkInputs = sdfCheckInputs;
          cargoDeps = rustPlatform.importCargoLock rootCargoLockInfo;
          doCheck = false;
          patchPhase = ''
            ${fixMakefilePaths}
          '';
          buildPhase = ''
            make VERGEN_GIT_SHA="$(cat ./GIT_REVISION)" build//bin/sdf
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/debug/sdf $out/bin/.
          '';
          checkPhase = ''
            echo "Do something for the checkPhase?"
          '';
        });

        packages.veritech = mkDerivation (finalAttrs: {
          name = "veritech";
          src = ./.;
          buildInputs = veritechBuildInputs;
          nativeBuildInputs = veritechNativeBuildInputs;
          depsTargetTarget = veritechDepsTargetTarget;
          checkInputs = veritechCheckInputs;
          cargoDeps = rustPlatform.importCargoLock rootCargoLockInfo;
          doCheck = false;
          patchPhase = ''
            ${fixMakefilePaths}
          '';
          buildPhase = ''
            make VERGEN_GIT_SHA="$(cat ./GIT_REVISION)" build//bin/veritech
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/debug/veritech $out/bin/.
          '';
          checkPhase = ''
            echo "Do something for the checkPhase?"
          '';
        });

        packages.web = mkDerivation (finalAttrs: {
          name = "web";
          src = ./.;
          buildInputs = webBuildInputs;
          nativeBuildInputs = webNativeBuildInputs;
          depsTargetTarget = webDepsTargetTarget;
          checkInputs = webCheckInputs;
          dontInstall = true;
          doCheck = false;
          buildFlags = [ "release//app/web" ];
          checkTarget = "test//app/web";
        });

        devShells.default = mkShell {
          buildInputs = [
            docker-compose
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
