use abcf::module::Event;
use abcf_macros::Event as MacroEvent;
use abcf_sdk::error::*;
use abcf_sdk::jsonrpc::Request;
use abcf_sdk::providers::{Provider, WsProvider};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use tokio::runtime::Runtime;

pub async fn subscribe<P: Provider>(param: Value, p: &mut P) -> Result<Option<Value>> {
    let subscribe_req = Request::new_to_value("subscribe", param);
    let resp = p.request::<Value,String>("subscribe", &subscribe_req).await?;

    return if let Some(val) = resp {
        let json = serde_json::from_str::<Value>(&val)?;
        Ok(Some(json))
    } else {
        Ok(None)
    };
}

#[derive(Clone, Debug, Deserialize, Serialize, MacroEvent)]
pub struct TestEvent {
    pub jsonrpc: String,
    pub id: i64,
    pub result: Value,
}

fn main() {
    let rt = Runtime::new().unwrap();
    let query = json!(["tm.event='NewBlock'"]);

    let mut provider = WsProvider::new();

    rt.block_on(async {
        let r = subscribe(query, &mut provider).await;
        println!("{:?}", r);

        let mut te = TestEvent {
            jsonrpc: "".to_string(),
            id: 0,
            result: Default::default(),
        };


        for _ in 0..5 {
            let r = provider.receive().await.unwrap().unwrap();
            println!("{:?}", r);
            te.from_abci_event_string(r);
            println!("{:#?}", te);
        }
    });
}
