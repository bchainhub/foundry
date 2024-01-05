# Foxar CLIs

The CLIs are written using [clap's](https://docs.rs/clap) [derive feature](https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md).

## Installation

See [Installation](../README.md#Installation).

## Usage

Read the [📖 Foxar Book][foxar-book]

## Debugging

Debug logs are printed with
[`tracing`](https://docs.rs/tracing/latest/tracing/). You can configure the
verbosity level via the
[`RUST_LOG`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/index.html#filtering-events-with-environment-variables)
environment variable, on a per package level,
e.g.:`RUST_LOG=spark,foxar_evm spark test`

[foxar-book]: https://book.getfoxar.sh
