pub mod abci_query;
pub mod subscribe;
pub mod unsubscribe;

pub trait ToRequest {
    fn to_request(&self) -> serde_json::Value;
}
