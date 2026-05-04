{
  version,
  lib,
  installShellFiles,
  rustPlatform,
  buildFeatures ? [ ],
}:

rustPlatform.buildRustPackage {
  pname = "umls2kg";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.lock
      ../Cargo.toml
    ];
  };

  inherit buildFeatures;
  inherit version;

  # inject version from nix into the build
  env.NIX_RELEASE_VERSION = version;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    installShellFiles

    rustPlatform.bindgenHook
  ];

  buildInputs = [ ];

  meta = with lib; {
    description = "CLI tool that converts the UMLS dataset into a CSV-based Knowledge Graph representation (Neo4J)";
    mainProgram = "umls2kg";
    homepage = "https://github.com/c2fc2f/UMLS-to-KG";
    license = licenses.mit;
    maintainers = [ maintainers.c2fc2f ];
  };
}
