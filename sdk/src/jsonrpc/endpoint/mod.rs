pub mod abci_query;
pub mod subscribe;
pub mod unsubscribe;

use alloc::string::String;
use hex::ToHex;
use serde::Serializer;

pub fn serialize<S, T>(data: T, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToHex,
{
    let s = data.encode_hex::<String>();
    let result = String::from("0x") + &s;
    serializer.serialize_str(&result)
}
