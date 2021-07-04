# ABCF

> Application Blockchain Framework

## Features (WIP)

- [ ] Modular abci application
  - [X] Define Module trait.
  - [X] Modular registry.
  - [X] Application defination.
  - [ ] Storage defination.
  - [ ] Event defination.
  - [ ] RPC defination.
- [ ] Dynamic update.
  - [X] Modular `no_std` support.
  - [ ] Add Webassembly engine.
  - [ ] Running Module on Webassembly.
- [X] ABCI interface in async.
  - [X] Basic function.
  - [X] Testing (for tendermint 0.34)
- [X] Compile go and rust in single binary.

## Run framewrok

### Requirements

- Go
- Rust
- make
- tendermint

Build.

``` bash
make build
```

Setup tendermint.

``` bash
TMHOME="/tmp/example" ./tendermint init
```

Run application.

``` bash
cd target && ./abcf -config "/tmp/example/config/config.toml"
```

