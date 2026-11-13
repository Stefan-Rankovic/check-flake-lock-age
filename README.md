<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me> -->

# Check `flake.lock` Age

This is a small tool written in Rust that checks the age of the `flake.lock`
file in the current directory.

## Motivation

A shell `chpwd` hook. I often forget to update my `flake.lock` files. If I set
this program's binary as a `chpwd` hook, it alerts me if I forgot to do that.

Because of that, many features which could be implemented, aren't. I made this
for my use case.

## Errors

None. Any error (failed to read, file doesn't exist, invalid syntax in file,
etc.) is ignored. As said above, I need this to be a shell hook. It would be
annoying if I got an error every time I changed the current directory to one
which doesn't contain a `flake.lock`.
