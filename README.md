# bm2

CLI for the Beat Saber mod repository BeatMods2

## Enabling backtraces

Use a nightly rust compiler ([rustup](https://rustup.rs/) is your friend), then
compile the crate with the nightly feature, and enable the environment variable
`RUST_BACKTRACE`: `RUST_BACKTRACE=1 cargo +nightly run --features nightly`
