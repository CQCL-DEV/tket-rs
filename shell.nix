{ pkgs ? import ./nix {} }:

pkgs.mkShell {
  name = "tket-rs-dev-shell";

  buildInputs = with pkgs; [
    clang
    cmake
    ninja
    ccache
    python39
    llvmPackages.libclang
    cargo
  ];

  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
}
