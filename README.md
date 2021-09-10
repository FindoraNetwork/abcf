# ABCF

> Application Blockchain Framework

Framework for blockchain based on tendermint.

## Design principle

- Try to use tendermint function.
- Simplilfy logic of blockchain.
  - Only focus on transcation check, state change and block generate.
  Needn't consider how to sync state, storage data and so on.
- Provide unit test, testing, and node.

## Features (WIP)

- [X] Module defination.
  - [X] Define Module trait.
  - [X] Application defination.
  - [X] Storage defination.
  - [X] Event defination.
  - [X] RPC defination.
- [X] Framework entry(single module).
  - [X] Application.
  - [X] RPC.
    - [X] RPC Context.
  - [X] Event.
  - [X] Store.
- [X] Add example using raw define.
- [X] Module Register(multi module).
  - [X] RPC
    - [X] Macros.
    - [X] Client SDK.
  - [X] Event
    - [X] Macros
    - [X] Client SDK.
- [X] Add example using macro.
- [ ] Dynamic update.
  - [X] Genesis info.
  - [ ] Node version match.
  - [X] Modular `no_std` support.
  - [ ] Add Webassembly engine.
  - [ ] Running Module on Webassembly.

## Using framewrok

Cargo.toml

``` toml
abcf = { git = "git://git@github.com/FindoraNetwork/abcf.git" }
```

### Requirements

- Rust
- Go > 1.16 (option): If use `tendermint-sys` as backend (default).
- tendermint > 0.34 (option): Is use `async-abci` as backend.

### Test

``` bash
$ RUST_LOG=debug cargo run --example devnet
```

