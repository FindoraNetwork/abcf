/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```

use abcf::{abci::Context, Application, Genesis, Module, ModuleMetadata, RPCResponse};
use abcf_macros::rpcs;
use abcf_node::Node;
use serde::{Deserialize, Serialize};

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
impl MockRPCs {
    pub async fn get_account(
        &mut self,
        _ctx: &mut Context<'_>,
        params: GetAccountRequest,
    ) -> abcf::Result<GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        Ok(resp)
    }
}

// ------------ module -----------------

pub struct MockModule {}

impl Module for MockModule {
    type RPCs = MockRPCs;

    type Application = MockApplicaion;

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
}

fn main() {
    env_logger::init();
    let mut node = Node::new("./target/abcf").unwrap();
    let mock_module = MockModule {};
    // node.regist(&mock_module);
    let _tdn = node.start().unwrap();
    loop {}
}
