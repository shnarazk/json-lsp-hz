{
  description = "WIP: JSON language server in Rust";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";
  outputs =
    { self, nixpkgs }:
    {
      packages = builtins.listToAttrs (
        map
          (
            system: with import nixpkgs { system = "${system}"; }; {
              name = system;
              value = {
                default = rustPlatform.buildRustPackage rec {
                  version = "0.1.0-20241121";
                  name = "json-lsp-hz-${version}";
                  pname = "json-lsp-hz";
                  src = self;
                  cargoHash = "sha256-uPB9HcIyj0Yy0i0riXLF7SVZ7DX7QoaIa1S3qPkX9ps=";
                  buildInputs = rustc.buildInputs ++ lib.optional stdenv.isDarwin [ libiconv ];
                  buildPhase = "cargo build --release";
                  installPhase = ''
                    mkdir -p $out/bin;
                    install -t $out/bin target/release/json-lsp-hz
                  '';
                };
              };
            }
          )
          [
            "x86_64-linux"
            "aarch64-linux"
            "x86_64-darwin"
            "aarch64-darwin"
          ]
      );
      devShells = builtins.listToAttrs (
        map
          (
            system:
            with import nixpkgs {
              system = "${system}";
              pkgs = nixpkgs.legacyPackages.${system};
            }; {
              name = system;
              value = {
                default = pkgs.mkShell {
                  packages = [
                    pkgs.bashInteractive
                    pkgs.cargo-watch
                    pkgs.lldb_19
                    pkgs.libiconv
                    pkgs.samply
                    pkgs.tokei
                  ];
                };
              };
            }
          )
          [
            "x86_64-linux"
            "aarch64-linux"
            "x86_64-darwin"
            "aarch64-darwin"
          ]
      );
    };
}
