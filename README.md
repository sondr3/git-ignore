# git-ignore [![Build Status](https://travis-ci.com/sondr3/git-ignore.svg?token=jVZ9BLfdPx6kBm4z8gXS&branch=master)](https://travis-ci.com/sondr3/git-ignore) [![Crates.io](https://img.shields.io/crates/v/git-ignore-generator.svg)](https://crates.io/crates/git-ignore-generator)

## What and why

Far too often I find myself going to [gitignore.io](https://www.gitignore.io/)
to quickly get `.gitignore` templates for my projects, so what would any
reasonable programmer do for repetitive tasks?
[Automate](https://xkcd.com/1319/) [it](https://xkcd.com/1205/)! Now you can
quickly and easily get and list all the templates available on gitignore.io, it
even works offline by caching the templates!

# Installation

There are two ways of installing it, either via Cargo (easiest) or via Nix
(authors preference). See installation and usage instructions below.

## Cargo

Make sure you have Rust installed (I recommend installing via
[rustup](https://rustup.rs/)), then run `cargo install git-ignore-generator`.

## Nix

Run `nix-env -iA nixpkgs.gitAndTools.git-ignore`. This version also includes man
pages.

# Usage

To download and cache all available templates, use `--update`. This can also be
used in combination with any of the other flags/arguments, or be ran as a
standalone flag.

``` sh
$ git ignore -u
```

To list all the available templates:

```sh
$ git ignore --list
[
    "1c",
    "1c-bitrix",
    "a-frame",
    "actionscript",
    "ada",
    [...],
    "zukencr8000"
]
```

You can also search for templates with the `--list` flag. **Note**: Listing
templates doesn't require exact matches, any template matching the start of your
query will be matched. See the example below for this, `intellij` matches all
three templates starting with `intellij`:

```sh
$ git ignore rust intellij --list
[
    "intellij",
    "intellij+all",
    "intellij+iml",
    "rust"
]
```

Then you can print the templates by omitting `--list`. **Note:** While listing
do not require exact matches, printing templates do. Use `--list` to find
templates. There will also be a notice about using cached results, this is
printed to `stderr` as to not interfere with piping.

```sh
$ git ignore rust intellij+all

### Created by https://www.gitignore.io
### Rust ###

[...]

# These are backup files generated by rustfmt
**/*.rs.bk
```

Finally, help is always available with `git ignore -h` (or `--help` if you used
Nix). 
