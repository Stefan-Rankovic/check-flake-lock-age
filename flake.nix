# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

{
    inputs = {
        flake-parts.url = "github:hercules-ci/flake-parts";
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

        crane.url = "github:ipetkov/crane";
    };

    outputs =
        { flake-parts, ... }@inputs:
        flake-parts.lib.mkFlake { inherit inputs; } {
            imports = [
                ./nix/per_system.nix
            ];
            systems = [ "x86_64-linux" ];
        };
}
