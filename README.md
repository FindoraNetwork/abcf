# ABCF

> Application Blockchain Framework

## Features (WIP)

- [ ] Modular abci application
  - [ ] Define Modular trait.
  - [ ] Modular registry.
  - [ ] Application defination.
  - [ ] Storage defination.
  - [ ] Event defination.
  - [ ] RPC defination.
- [ ] Dynamic update.
  - [X] Modular `no_std` support.
- [X] ABCI interface in async.
  - [X] Basic function.
  - [X] Testing (for tendermint 0.34)

### Run test

Use tendermint 0.34.

Start tendermint:

``` bash
TMHOME="/tmp/example1" ./tendermint node tcp://127.0.0.1:26658
```

Start mock server.

``` bash
RUST_LOG=DEBUG cargo run --example mock
```

