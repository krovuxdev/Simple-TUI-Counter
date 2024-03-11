{
  description = "A TUI (ratatui) for terminal";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }: 
    let
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rust = pkgs.buildPackages.rust-bin.stable.latest.minimal;
      nativeBuildInputs = with pkgs; [
        rust
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        inherit nativeBuildInputs;
      };
    };

}

