{
  craneLib,
  inputs,
  lib,
  # Options (keep sorted)
  default-features ? true,
  all-features ? false,
  features ? [],
  profile ? "release",
}: let
  cargoManifest = lib.importTOML "${inputs.self}/Cargo.toml";

  commonArgs = {
    name = cargoManifest.package.name;
    version = cargoManifest.package.version;

    src = inputs.nix-filter.lib {
      include = [
        "Cargo.lock"
        "Cargo.toml"
        "src"
      ];
      root = inputs.self;
    };
  };

  # We perform default-feature unification in nix, because some of the dependencies
  # on the nix side depend on feature values.
  allDefaultFeatures = cargoManifest.features.default;
  allFeatures = lib.unique (
    lib.remove "default" (lib.attrNames cargoManifest.features)
    ++ lib.attrNames
    (lib.filterAttrs (_: dependency: dependency.optional or false)
      cargoManifest.dependencies)
  );

  features' =
    lib.unique
    (features
      ++ lib.optionals default-features allDefaultFeatures
      ++ lib.optionals all-features allFeatures);

  featureEnabled = feature: builtins.elem feature features';

  dontStrip = profile != "release";
in
  craneLib.buildPackage (commonArgs
    // {
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      cargoExtraArgs =
        "--locked --no-default-features "
        + lib.optionalString
        (features' != [])
        "--features "
        + (builtins.concatStringsSep "," features');

      passthru = {
        env = {};
      };

      # This is redundant with CI
      doCheck = false;

      meta.mainProgram = commonArgs.name;
    })
