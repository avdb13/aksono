{
  default,
  pkgs,
  mkShell,
  toolchain,
  taplo,
}:
mkShell {
  env =
    default.env
    // {
      RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    };
  nativeBuildInputs =
    [
      pkgs.rust-bin.nightly.latest.rustfmt
      taplo
      toolchain
    ]
    ++ (with default;
      nativeBuildInputs
      ++ propagatedBuildInputs
      ++ buildInputs);
}
