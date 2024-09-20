{
  description = "Radior flake.";

  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url  = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {

        packages.default = pkgs.rustPlatform.buildRustPackage rec {

          name = "radior";
          version = "0.4.1";
          src = ./.;

          nativeBuildInputs = with pkgs ; [
            pkg-config
            mpv-unwrapped
            rust-bin.nightly.latest.default
          ];

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          PKG_CONFIG_PATH = "${pkgs.mpv-unwrapped.dev}/lib/pkgconfig";

        };

      }
    );

}
