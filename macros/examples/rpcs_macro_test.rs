#![feature(generic_associated_types)]

use abcf::entry::Tree;
use abcf::module::StorageTransaction;
use abcf::{Merkle, RPCResponse, Storage};
use abcf_macros::rpcs;
use serde::{Deserialize, Serialize};

pub struct RpcTest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountRequest {
    code: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountResponse {
    name: String,
    code: u8,
}

pub struct EmptyStorage {}

impl EmptyStorage {
    pub fn new() -> Self {
        EmptyStorage {}
    }
}

impl Storage for EmptyStorage {
    fn height(&self) -> abcf::Result<i64> {
        Ok(0)
    }

    fn commit(&mut self) -> abcf::Result<()> {
        Ok(())
    }

    fn rollback(&mut self, _height: i64) -> abcf::Result<()> {
        Ok(())
    }
}

impl StorageTransaction for EmptyStorage {
    type Transaction<'a> = ();

    type Cache = ();

    fn cache(_tx: Self::Transaction<'_>) -> Self::Cache {
        ()
    }

    fn transaction(&self) -> Self::Transaction<'_> {
        ()
    }

    fn execute(&mut self, _transaction: Self::Transaction<'_>) {}
}

impl Merkle<sha3::Sha3_512> for EmptyStorage {
    fn root(&self) -> abcf::Result<digest::Output<sha3::Sha3_512>> {
        Ok(Default::default())
    }
}

impl Tree for EmptyStorage {
    fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub struct EmptyStruct {}

#[rpcs]
impl EmptyStruct {}

#[rpcs]
impl RpcTest {
    pub async fn get_account(
        &mut self,
        _ctx: &mut abcf::manager::RContext<'_, EmptyStorage, EmptyStorage>,
        params: GetAccountRequest,
    ) -> RPCResponse<'_, GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        RPCResponse::new(resp)
    }
}

#[tokio::main]
async fn main() {
    // TODO: use node as example.
    // let _es = EmptyStruct {};
    //
    // let mut rt = RpcTest {};
    //
    // let mut context = Context {
    //     event: None,
    //     storage: StorageContext {},
    //     calls: CallContext { name_index: (), calls: () }
    // };
    //
    // let params = GetAccountRequest { code: 99 };
    // let params = serde_json::to_value(params).unwrap();
    //
    // let resp = rt.call(&mut context, "get_account", params).await.unwrap();
    //
    // let resp = serde_json::from_value::<GetAccountResponse>(resp.unwrap()).unwrap();
    //
    // assert_eq!(resp.name, "jack");
    // assert_eq!(resp.code, 99);
}
