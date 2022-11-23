{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.nightly."2022-11-17".default.override {
          targets = [ "wasm32-unknown-unknown" "aarch64-unknown-linux-gnu" ];
        };

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;
        deps = {
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs; [ clang git pkg-config ];
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
            (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        };
        commonArgs = deps // { src = craneLib.cleanCargoSource ./.; };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "cargoArtifacts";
          cargoExtraArgs =
            "--workspace --exclude kusama-runtime --exclude kusama-runtime-wasm --exclude polkadot-runtime --exclude polkadot-runtime-wasm --exclude pallet-xcm --exclude wasm-project";
        });

      in {
        packages.default = craneLib.buildPackage (deps // {
          src = craneLib.cleanCargoSource ./.;
          inherit cargoArtifacts;
          doCheck = false;
          cargoCheckCommand = "true";
          cargoExtraArgs =
            "--workspace --exclude=parachain-node,parachain-runtime";
        });
      });
}
