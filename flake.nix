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

          devShell =
            let
              inherit (pkgs) symlinkJoin mkShell stdenv rustPlatform;
              toolchain = symlinkJoin {
                name = "rust-toolchain";
                paths = with rustPlatform.rust; [ cargo rustc ];
              };
            in
            mkShell {
              buildInputs = with pkgs; [ toolchain keycloak pkg-config openssl ];

              # to make IDE happy
              NIX_LD = stdenv.cc.bintools.dynamicLinker;
              RUST_TOOLCHAIN = toolchain;
              RUST_STDLIB = rustPlatform.rustLibSrc;

              PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            };

          defaultPackage = let
            inherit (pkgs) rustPlatform;
          in rustPlatform.buildRustPackage {
            pname = "nss_keycloak";
            version = "0.1.0";
            src = builtins.path { path = ./.; name = "nss_keycloak"; };
            cargoSha256 = "sha256-d7/vpg471wGsw4kvNMSUMjzfBjgd1SkwQZiGZLzo8fw=";

            nativeBuildInputs = with pkgs; [ pkg-config openssl ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

            doCheck = false;  # tests require networking
          };
        }
      )
    // { inherit inputs; }
  ;
}
