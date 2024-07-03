{
  description = "tiny-basic";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
      devShells.${system}.default =
        pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
          ];
        };
    };
}
