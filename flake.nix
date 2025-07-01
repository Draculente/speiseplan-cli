{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs";
  };
  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
        };
        cargo-config = ./Cargo.toml |> builtins.readFile |> builtins.fromTOML;
      in
      {
        packages = rec {
          default = speiseplan-cli;
          speiseplan-cli = pkgs.rustPlatform.buildRustPackage {
            pname = cargo-config.package.name;
            inherit (cargo-config.package) version;
            src = ./.;
            cargoHash = "sha256-vzx1u7PJce5597dslJhJ9dGUACRKnhJC4EF2tfTbs7c=";

            meta = {
              description = "A CLI tool to fetch and display the menu of the mensas in Schleswig-Holstein";
              license = pkgs.lib.licenses.mit;
            };
          };
        };
      }
    );
}
