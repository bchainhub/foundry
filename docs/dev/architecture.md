# Architecture

This document describes the high-level architecture of foxar.

### `evm/`

foxar's evm tooling. This is built around [`revm`](https://github.com/bluealloy/revm) and has additional
implementation of

-   [cheatcodes](./cheatcodes.md) a set of solidity calls dedicated to testing which can manipulate the environment in
    which the execution is run

### `config/`

Includes all of foxar's settings and how to get them

### `cli/`

The core `spark` and `probe` cli implementation. Includes all subcommands.
