{
  description = "rust development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
  };
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell
        {
          nativeBuildInputs = [
            pkgs.rustup
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.rustfmt
          ];
          shellHook = ''
            export CARGO_HOME=$HOME/.cargo
            export RUSTUP_HOME=$HOME/.rustup
            echo "Rust development environment ready"
          '';
        };
    };
}
