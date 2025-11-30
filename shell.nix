{ ... }:
let
  sources = import ./npins;
  pkgs = import sources.nixpkgs {
    overlays = [ (import sources.rust-overlay) ];
  };
  toolchain = pkgs.rust-bin.stable."1.91.1".default.override {
    extensions = [ "rust-src" "clippy" "rustfmt" ];
  };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    toolchain
  ];

  buildInputs = with pkgs; [
    rust-analyzer
    jetbrains.rust-rover
    cargo
  ];

  shellHook = ''
    mkdir -p ./toolchain

    ln -sfn ${toolchain}/lib ./toolchain
    ln -sfn ${toolchain}/bin ./toolchain

    export RUST_SRC_PATH="$(pwd)/toolchain/lib/rustlib/src/rust/library"
  '';
}

