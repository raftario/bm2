# bm2

CLI for the Beat Saber mod repository BeatMods2

[![GitHub Actions](https://github.com/raftario/bm2/workflows/Build/badge.svg)](https://github.com/raftario/bm2/actions?workflowID=Build)
[![Crates.io](https://img.shields.io/crates/v/bm2.svg)](https://crates.io/crates/bm2)

## BeatMods1 compatibility

The current version of `bm2` works with BeatMods1,
but your mod must follow the new BeatMods2 standards to be able to use it.

## Manifest

`bm2` makes extensive use of the manifest and validates it.
You can check out the [manifest schema](https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Schema.json) or an [example](https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Example.json) to get started.
Most code editors support JSON Schema validation using various settings, which can make editing the manifest way easier.

## Usage

Just run `bm2 --help` to get started.

### Available commands

* `publish` - Publishes to BeatMods

## Installation

You can either download the tool from the releases page
or clone this repository and run `cargo install --path .` if you have the Rust toolchain installed.

If you install from the releases, you'll need to add the directory where the tool is located
to your `PATH` environment variable.

## Contributing

Contributors are welcome! To get started, you'll just need the Rust toolchain installed.

### Enabling backtraces

Use a nightly rust compiler ([rustup](https://rustup.rs/) is your friend),
then compile the crate with the nightly feature, and enable the environment variable
`RUST_BACKTRACE`: `RUST_BACKTRACE=1 cargo +nightly run --features nightly`

## License

`bm2` is [MIT](LICENSE) licensed.
