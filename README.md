# ABCF

> Application Blockchain Framework

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
  - [ ] Store.
- [ ] Add example using raw define.
- [X] Module Register(multi module).
  - [ ] RPC
    - [ ] Macros.
    - [ ] Client SDK.
  - [ ] Event
    - [ ] Macros
    - [ ] Client SDK.
- [ ] Add example using macro.
- [ ] Dynamic update.
  - [X] Modular `no_std` support.
  - [ ] Add Webassembly engine.
  - [ ] Running Module on Webassembly.

## Using framewrok

Cargo.toml

``` toml
abcf = { git = "git://git@github.com/FindoraNetwork/abcf.git" }
```

Using raw trait:

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

    fn application(&self) -> Self::Application {
        ()
    }

    fn events(&self) -> Self::Events {
        ()
    }

    fn rpcs() -> Self::RPCs {
        ()
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

Using macro.

``` rust
use abcf::{Module, ModuleMetadata, ModuleRegister, Node};

// Define RPC

struct MockRPCS {}

#[rpc::rpcs]
impl RPCs for MockRPCs {}

#[derive(Deserialize)]
pub struct GetAccountRequest {}

#[derive(Serialize)]
pub struct GetAccountResponse {}

#[rpc::rpcs]
impl MockRPCs {
    #[rpc::method("get_account")]
    pub async fn get_account(&mut self, ctx: &mut Context, params: GetAccountRequest) 
      -> Response<GetAccountResponse> {
      // ..
    }
}

// Define Module

struct Mock {}

#[abcf::module(MockRPCs, (), (), ())]
impl Module for Mock {
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

