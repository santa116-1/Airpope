# airpope

![crates.io version](https://img.shields.io/crates/v/airpope) ![CI](https://github.com/noaione/airpope-mango/actions/workflows/ci.yml/badge.svg)

[`airpope-mango`](airpope) (or `airpope`) is a downloader but can also be considered an almost full-blown replacement for the app/web version, with the exception of currency purchase, as a simple CLI application.

Currently we support the following source:
- [MU! by SQ](https://crates.io/crates/airpope-musq)
- [KM by KC](https://crates.io/crates/airpope-kmkc)
- [AM by AP](https://crates.io/crates/airpope-amap)
- [SJ/M by V](https://crates.io/crates/airpope-sjv)
- [小豆 (Red Bean) by KRKR](https://crates.io/crates/airpope-rbean)

## Installation

You can install by cloning the repository then building manually...

Or...

```bash
cargo install --locked airpope
```

Or, if you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)...

```bash
cargo binstall --locked airpope
```

## Usage

Refer to the [repo](airpope) on how to authenticate with each source.<br />
For a list of available commands, use the `--help` argument.

[![asciicast](https://asciinema.org/a/636303.svg)](https://asciinema.org/a/636303)

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter on each platform with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/airpope-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

[airpope]: https://github.com/noaione/airpope-mango