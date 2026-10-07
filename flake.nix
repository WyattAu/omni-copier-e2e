{
  # OmniRust dev environment — nix owns system tools, rustup owns the Rust
  # toolchain (rust-toolchain.toml stays the pin everyone shares).
  description = "OmniRust-template development environment";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
            rust-analyzer

            # estate gate toolchain
            cargo-nextest
            cargo-llvm-cov
            cargo-deny
            cargo-vet
            cargo-mutants
            cargo-semver-checks
            cargo-audit
            cargo-fuzz

            pkg-config
            openssl
            git
          ];

          env = {
            RUSTUP_TOOLCHAIN = "stable";
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          };
        };
      });
    };
}
