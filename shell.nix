with import <nixpkgs> {};

mkShell {
  buildInputs = [
    cargo
    clippy
    rustfmt
    rustc
    rust-analyzer
  ];
}
