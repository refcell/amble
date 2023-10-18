# amble

[![Build Status]][actions]
[![License]][mit-license]
[![Latest Version]][crates.io]
[![amble: rustc 1.31+]][Rust 1.31]

[Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/amble/ci.yml?branch=main
[actions]: https://github.com/refcell/amble/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/amble.svg
[crates.io]: https://crates.io/crates/amble
[amble: rustc 1.31+]: https://img.shields.io/badge/amble-rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/License-MIT-orange.svg
[mit-license]: https://github.com/refcell/amble/blob/main/LICENSE.md

**First class, scalable rust project generator with batteries included.**

![](./etc/banner.png)

**[Install](./docs/install/installation.md)**
| [User Book](https://amble.refcell.org)
| [Developer Docs](./docs/developers/developers.md)
| [Crate Docs](https://crates.io/crates/amble)

_The project is still work in progress, see the [disclaimer below](#status-httpsgithubcomrefcellamblelabelsalpha)._

## What is amble?

`amble` is a fairly minimal cli application for generating rust projects
with batteries included and with an architecture that scales well, using a
workspace and sub-crates.

You can think of `amble` as an extension of `cargo new`.
Where `cargo new` ...


## Usage

Install `amble` using cargo.

```sh
cargo install amble
amble --version
```

Alternatively, `amble` can be built from source.

```sh
git clone git@github.com:refcell/amble.git
cd amble
cargo build --release
amble --version
```

## Status https://github.com/refcell/amble/labels/alpha

`amble` is in **alpha** mode, and should be used for
experimentation only.

Local and devnet experimentation is highly encouraged.
New issues are also welcome.

In the meantime, contribute, experiment, and have fun!

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/amble/issues/new)
if no relevant issue already exists.

## Contributions

All contributions are welcome!

## License

This project is licensed under the [MIT License](LICENSE.md).
Free and open-source, forever.

_All our rust are belong to you._
