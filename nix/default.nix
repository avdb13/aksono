{
  craneLib,
  inputs,
  lib,
}: let
  cargoManifest = lib.importTOML "${inputs.self}/Cargo.toml";

  buildPackageEnv = {};

  commonArgs = {
    name = "Aksono";
    version = "0.1.0";

    src = inputs.nix-filter.lib {
      include = [
        "Cargo.lock"
        "Cargo.toml"
        "src"
      ];
      root = inputs.self;
    };
  };
in
  craneLib.buildPackage (commonArgs
    // {
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      cargoExtraArgs =
        "--locked --no-default-features ";
        # + lib.optionalString
        # (cargoManifest.features.default != [])
        # "--features "
        # + (builtins.concatStringsSep "," cargoManifest.features.default);

      env = buildPackageEnv;

      passthru = {
        env = buildPackageEnv;
      };

      meta.mainProgram = commonArgs.name;
    })
