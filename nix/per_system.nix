# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    inputs,
    ...
}:

{
    perSystem =
        { pkgs, ... }:
        let
            cargoTomlContents = fromTOML (builtins.readFile ../Cargo.toml);
            inherit (cargoTomlContents.package) name version;

            craneLib = inputs.crane.mkLib pkgs;
            commonArgs = {
                inherit version;
                src = craneLib.cleanCargoSource ../.;
                strictDeps = true;
            };
            cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "${name}-deps"; });
        in
        {
            checks.clippy = craneLib.cargoClippy (
                commonArgs
                // {
                    inherit cargoArtifacts;
                    pname = name;
                }
            );
            packages.default = craneLib.buildPackage (
                commonArgs
                // {
                    inherit cargoArtifacts;
                    pname = name;
                }
            );
        };
}
