{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }@inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          legacyPackages = pkgs;

          devShell = let
            inherit (pkgs) symlinkJoin mkShell stdenv rustPlatform;

            # to let IDE happy
            toolchain = symlinkJoin {
              name = "rust-toolchain";
              paths = with rustPlatform.rust; [ cargo rustc ];
            };
          in mkShell {
            buildInputs = with pkgs; [ toolchain keycloak pkg-config openssl ];
            NIX_LD = stdenv.cc.bintools.dynamicLinker;
            RUST_TOOLCHAIN = toolchain;
            RUST_STDLIB = rustPlatform.rustLibSrc;
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        }
      )
    // { inherit inputs; }
  ;
}
