# git-ignore [![Build Status](https://travis-ci.com/sondr3/git-ignore.svg?token=jVZ9BLfdPx6kBm4z8gXS&branch=master)](https://travis-ci.com/sondr3/git-ignore) [![Crates.io](https://img.shields.io/crates/v/git-ignore-generator.svg)](https://crates.io/crates/git-ignore-generator)

## What

Do you far too often go to www.gitignore.io to quickly copy and paste a few
assorted templates that you always use? I do, and decided that I should do
something about it! So I spent far more hours making this than I would ever save
by simply going to the website.

# Installation

Make sure you have Rust installed (I recommend installing via
[rustup](https://rustup.rs/)), then run `cargo install git-ignore-generator`.
You can now quickly list and fetch all the available templates on
www.gitignore.io, all from the comforts of the command line!

To list all the available templates:

```sh
$ git ignore list
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

You can also search for templates:

```sh
$ git ignore list rust intellij
[
    "intellij",
    "intellij+all",
    "intellij+iml",
    "rust"
]
```

Once you've found the templates you want to use, simply run:

```sh
$ git ignore get rust intellij+all

# Created by https://www.gitignore.io/api/rust,intellij+all
# Edit at https://www.gitignore.io/?templates=rust,intellij+all

[..]

# These are backup files generated by rustfmt
**/*.rs.bk

# End of https://www.gitignore.io/api/rust,intellij+all
```

Finally, if need be, you can always run `git ignore help` to see more options
--- spoiler alert, there are none.