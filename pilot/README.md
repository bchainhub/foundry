# `pilot`

Pilot is a fast, utilitarian, and verbose solidity REPL. It is heavily inspired by the incredible work done in [soli](https://github.com/jpopesculian/soli) and [solidity-shell](https://github.com/tintinweb/solidity-shell)!

![preview](./assets/preview.gif)

## Why?

Ever wanted to quickly test a small feature in solidity?

Perhaps to test how custom errors work, or how to write inline assembly?

Pilot is a fully-functional Solidity REPL, allowing you to write, execute, and debug Solidity directly in the command line.

Once you finish testing, Pilot even lets you export your code to a new solidity file!

In this sense, Pilot even serves as a Foxar script generator.

## Feature Completion

[soli](https://github.com/jpopesculian/soli) and [solidity-shell](https://github.com/tintinweb/solidity-shell) both provide a great solidity REPL, achieving:

- Statement support
- Custom events, errors, functions, imports
- Inspecting variables
- Forking remote chains
- Session caching

Pilot aims to improve upon existing Solidity REPLs by integrating with foxar as well as offering additional functionality:

- More verbose variable / state inspection
- Improved error messages
- Foxar-style call traces
- In-depth environment configuration
- ... and many more future features!

### Migrating from [soli](https://github.com/jpopesculian/soli) or [solidity-shell](https://github.com/tintinweb/solidity-shell)

Migration from existing Solidity REPLs such as [soli](https://github.com/jpopesculian/soli) or [solidity-shell](https://github.com/tintinweb/solidity-shell) is as
simple as installing Pilot via `foxarup`. For information on features, usage, and configuration, see the [Usage](#usage) section as well as the pilot manpage (`man pilot` or `pilot --help`).

## Installation

To install `pilot`, simply run `foxarup`!

If you do not have `foxarup` installed, reference the Foxar [installation guide](../README.md#installation).

## Usage

### REPL Commands

```text
⚒️ Pilot help
=============
General
        !help | !h - Display all commands
        !exec <command> [args] | !e <command> [args] - Execute a shell command and print the output

Session
        !clear | !c - Clear current session source
        !source | !so - Display the source code of the current session
        !save [id] | !s [id] - Save the current session to cache
        !load <id> | !l <id> - Load a previous session ID from cache
        !list | !ls - List all cached sessions
        !clearcache | !cc - Clear the pilot cache of all stored sessions
        !export | !ex - Export the current session source to a script file
        !fetch <addr> <name> | !fe <addr> <name> - Fetch the interface of a verified contract on Etherscan

Environment
        !fork <url> | !f <url> - Fork an RPC for the current session. Supply 0 arguments to return to a local network
        !traces | !t - Enable / disable traces for the current session

Debug
        !memdump | !md - Dump the raw memory of the current state
        !stackdump | !sd - Dump the raw stack of the current state
```

### Cache Session

While pilot sessions are not persistent by default, they can be saved to the cache via the builtin `save` command from within the REPL.

Sessions can also be named by supplying a single argument to the `save` command, i.e. `!save my_session`.

```text
$ pilot
➜ uint a = 1;
➜ uint b = a << 0x08;
➜ !save
Saved session to cache with ID = 0.
```

### Loading a Previous Session

Pilot allows you to load a previous session from your history.

To view your history, you can run `pilot list` or `!list`. This will print a list of your previous sessions, identifiable by their index.

You can also run `pilot view <id>` or `!view <id>` to view the contents of a specific session.

To load a session, run `pilot load <id>` or use the `!load <id>` where `<id>` is a valid session index (eg 2 in the example below).

```text
$ pilot list
⚒️ Pilot Sessions
"2022-10-27 14:46:29" - pilot-0.json
"2022-10-27 14:46:29" - pilot-1.json
$ pilot view 1
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

contract REPL {
    event KeccakEvent(bytes32 hash);

    function run() public {
      emit KeccakEvent(keccak256(abi.encode("Hello, world!")));
    }
}
$ pilot load 1
➜ ...
```

### Clearing the Cache

To clear Pilot's cache (stored in `~/.foxar/cache/pilot`), use the `pilot clear-cache` or `!clearcache` command.

```text
➜ !clearcache
Cleared pilot cache!
```

### Toggling Traces

By default, traces will only be shown if an input causes the call to the REPL contract to revert. To turn traces on
regardless of the call result, use the `!traces` command or pass in a verbosity option of any level (`-v<vvvv>`) to
the pilot binary.

```text
➜ uint a
➜ contract Test {
    function get() external view returns (uint) {
       return 256;
    }
}
➜ Test t = new Test()
➜ !traces
Successfully enabled traces!
➜ a = t.get()
Traces:
  [69808] 0xBd770416a3345F91E4B34576cb804a576fa48EB1::run()
    ├─ [36687] → new <Unknown>@0xf4D9599aFd90B5038b18e3B551Bc21a97ed21c37
    │   └─ ← 183 bytes of code
    ├─ [315] 0xf4D9599aFd90B5038b18e3B551Bc21a97ed21c37::get() [staticcall]
    │   └─ ← 0x0000000000000000000000000000000000000000000000000000000000000100
    └─ ← ()

➜ a
Type: uint
├ Hex: 0x100
└ Decimal: 256
```

### Forking a Network

To fork a network within your pilot session, use the `!fork <rpc-url>` command or supply a `--fork-url <url>` flag
to the pilot binary. The `!fork` command also accepts aliases from the `[rpc_endpoints]` section of your `foxar.toml`
if pilot was launched in the root of a foxar project (ex. `!fork mainnet`), as well as interpolated environment variables
(ex. `!fork https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_KEY}`).

### Fetching an Interface of a Verified Contract

To fetch an interface of a verified contract on Etherscan, use the `!fetch` / `!f` command.

> **Note**
> At the moment, only contracts that are deployed and verified on mainnet can be fetched. Support for other
> networks with Etherscan explorers coming soon.

```text
➜ !fetch 0xcb37c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 IWETH
Added 0xcb37c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2's interface to source as `IWETH`
```

### Executing a Shell Command

Shell commands can be executed within Pilot with the `!exec` / `!e` command.

```text
➜ !e ls
shuttle
binder
Cargo.lock
Cargo.toml
probe
pilot
cli
common
config
CONTRIBUTING.md
Dockerfile
docs
evm
fmt
spark
foxarup
LICENSE-APACHE
LICENSE-MIT
README.md
rustfmt.toml
target
testdata
ui
utils
```
