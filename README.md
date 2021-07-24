# ABCF

> Application Blockchain Framework

## Design principle

- Try to use tendermint function.

## Features (WIP)

- [X] Module defination.
  - [X] Define Module trait.
  - [X] Application defination.
  - [X] Storage defination.
  - [X] Event defination.
  - [X] RPC defination.
- [X] Framework entry(single module).
  - [X] Module application.
  - [X] RPC.
    - [ ] RPC Context.
  - [ ] Event.
  - [ ] Store.
- [X] Module Register(multi module).
- [ ] Dynamic update.
  - [X] Modular `no_std` support.
  - [ ] Add Webassembly engine.
  - [ ] Running Module on Webassembly.

## Using framewrok

Cargo.toml

``` toml
abcf = { git = "git://git@github.com/FindoraNetwork/abcf.git" }
```

In code:

``` rust
use abcf::{Module, ModuleMetadata, ModuleRegister, Node};

struct Mock {}

impl Module for Mock {
    type Storage = ();
    type Event = ();
    type RPCs = ();
    type Application = ();

    fn metadata(&self) -> ModuleMetadata<'_> {
        ModuleMetadata {
            name: "mock",
            version: "0.1.0",
            impl_version: "0.1.3",
            genesis: Genesis {
                target_height: 1,
            }
        }
    }
}

fn main() {
    let node = Node::init_debug();
    let mock_module = Mock {};
    let mr = ModuleRegister::new()
        .regist(mock_module);
    node.register(mr);
    smol::block_on( async {
        node.start().await();
    });
}

```

### Requirements

- Rust
- Go > 1.16 (option): If use `tendermint-sys` as backend (default).
- tendermint > 0.33 (option): Is use `async-abci` as backend.

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
cd target && RUST_LOG=debug ./abcf -config "/tmp/example/config/config.toml"
```

