{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, ... } @ inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };
    rust-build = pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
      targets = [];
    };
    naersk-lib = naersk.lib.${system}.override {
      rustc = rust-build;
      cargo = rust-build;
    };
    cargo-aoc = (pkgs.makeRustPlatform { rustc = rust-build; cargo = rust-build; }).buildRustPackage rec {
      pname = "cargo-aoc";
      version = "0.3.7";
      src = pkgs.fetchCrate {
        inherit pname;
        inherit version;
        hash = "sha256-XZWV7Nzc+E+58B38yZkIBVjMYhiaOlkuWpszhTuQ55g=";
      };
      cargoSha256 = "sha256-nH/ON+cmm+EJGym3xV+AgNtVIxNhD/zobeUbBWXUGyk=";
    };
    aoc = naersk-lib.buildPackage {
      pname = "aoc";
      root = ./.;
      buildInputs = with pkgs; [
      ];
      nativeBuildInputs = with pkgs; [
        cargo-aoc
        rust-build
      ];
    };
  in
  {
    devShell.${system} = pkgs.mkShell {
      packages = with pkgs; [
        git
        cargo-edit
        cargo-watch
        rust-analyzer-unwrapped
      ];
      inputsFrom = with pkgs; [
        aoc
      ];
      RUST_SRC_PATH = "${rust-build}/lib/rustlib/src/rust/library";
    };
    packages.${system}.default = aoc;
  };
}
