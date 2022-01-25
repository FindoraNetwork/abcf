use abcf_sdk::{providers::{HttpGetProvider, Provider}, jsonrpc::endpoint};

#[tokio::main]
async fn main() {
    let mut provider = HttpGetProvider {
        url: "http://127.0.0.1:26657".to_string(),
    };

    let result = provider
        .request::<(), endpoint::net_info::Response>("net_info", &())
        .await;
    
    println!("{:#?}", result);
}
