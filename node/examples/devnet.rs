/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{abci::Context, Application, Genesis, Module, ModuleMetadata, RPCResponse};
use abcf_macros::{module, rpcs};
use abcf_node::Node;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;
use abcf_sdk::Provider;

//------------- include -----------------
/// must first build and then include
pub mod mock_rpcs_call {
    include!(concat!(env!("OUT_DIR"), "/MockRPCs.rs"));
}

// ------------ application -------------

pub struct MockApplicaion {}

#[async_trait::async_trait]
impl Application for MockApplicaion {}

// ------------ rpcs -------------------

pub struct MockRPCs {}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountRequest {
    code: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountResponse {
    name: String,
    code: u8,
}

#[rpcs]
#[module(mock)]
impl MockRPCs {
    pub async fn get_account(
        &mut self,
        _ctx: &mut Context<'_>,
        params: Value,
    ) -> RPCResponse<'_, GetAccountResponse> {
        let req = serde_json::from_value::<GetAccountRequest>(params).unwrap();
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: req.code,
        };
        abcf::RPCResponse::new(resp)
    }
}

// ------------ module -----------------

#[derive(Clone)]
pub struct MockModule {}

impl Module for MockModule {
    type RPCs = MockRPCs;

    type Application = MockApplicaion;
    type Callable = ();

    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            name: "mock".to_string(),
            version: "0.1.0".to_string(),
            impl_version: "0.1".to_string(),
            genesis: Genesis { target_hight: 0 },
        }
    }

    fn application(&self) -> Self::Application {
        MockApplicaion {}
    }

    fn rpcs(&self) -> Self::RPCs {
        MockRPCs {}
    }

    fn callable(&self) -> Self::Callable {
        ()
    }
}

fn main() {
    env_logger::init();
    let mut node = Node::new("./target/tendermint").unwrap();
    let module = MockModule {};
    node.regist(&module);
    node.start().unwrap();
    std::thread::park();
}

#[test]
fn test() {
    env_logger::init();
    let mut node = Node::new("./target/tendermint").unwrap();
    let module = MockModule {};
    node.regist(&module);
    node.start().unwrap();

    let rt = Runtime::new().unwrap();
    let req = serde_json::to_value(&GetAccountRequest { code: 19 }).unwrap();

    let provider = abcf_sdk::rpc_sdk::AbciQueryRpcProvider{};

    rt.block_on(async {
        let r = mock_rpcs_call::get_account(req, provider).await;
        println!("{:#?}", r);
    });

    std::thread::park();
}
