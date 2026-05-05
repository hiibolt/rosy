{
    # Tremendous thanks to @oati for her help
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };
    outputs = { self, nixpkgs, rust-overlay, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [ rust-overlay.overlays.default ];
                };
                rustVersion = pkgs.rust-bin.nightly.latest.default;
                    rustPlatform = pkgs.makeRustPlatform {
                    cargo = rustVersion;
                    rustc = rustVersion;
                };
                deps = [ (rustVersion.override { extensions = ["rust-src"]; }) ] ++ (with pkgs; [
                    pkg-config
                    cargo
                    gcc
                    rustfmt
                    clippy
                    openssl.dev
                    pest-ide-tools
                    clang
                    cmake
                    mpi
                    libclang
                    llvmPackages.libclang
                    bun
                    nodejs
                ]);
                localRustBuild = rustPlatform.buildRustPackage rec {
                    pname = "app";
                    version = "0.0.1";
                    src = ./.;
                    cargoBuildFlags = "";
                    cargoLock = {
                        lockFile = ./Cargo.lock;
                    };
                    nativeBuildInputs = deps;
                    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
                    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps;
                    OPENSSL_LIB_DIR = pkgs.openssl.out + "/lib";
                    LIBCLANG_PATH = pkgs.libclang.lib + "/lib";
                };
            in
            {
                defaultPackage = localRustBuild;
                packages.cargo_setup = pkgs.mkShell {
                    buildInputs = with pkgs; [ cargo rustc ];
                };
                devShells.default = pkgs.mkShell {
                    inputsFrom = [ localRustBuild ];
                    shellHook = ''
                        export PATH="$HOME/.bun/bin:$PATH"
                    '';
                    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
                    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps;
                    OPENSSL_LIB_DIR = pkgs.openssl.out + "/lib";
                    LIBCLANG_PATH = pkgs.libclang.lib + "/lib";
                };
            }
        );
}
