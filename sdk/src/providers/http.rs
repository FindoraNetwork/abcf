use super::Provider;
use crate::error::{Error, Result};
use crate::jsonrpc::Response;
use alloc::{boxed::Box, string::String};
use serde_json::Value;
use alloc::string::ToString;
use alloc::vec::Vec;

/// post
pub struct HttpPostProvider {}

#[async_trait::async_trait]
impl Provider for HttpPostProvider {
    async fn request(&mut self, _method: &str, params: &str) -> Result<Option<String>> {
        let url = "http://127.0.0.1:26657";
        let mut resp_val = reqwest::Client::new()
            .post(url)
            .body(String::from(params))
            .send()
            .await?
            .json::<Response<Value>>()
            .await?;

        return if let Some(ref mut result) = resp_val.result {
            result
                .as_object_mut()
                .and_then(|result_map| result_map.get_mut("response"))
                .and_then(|resp_obj| resp_obj.as_object_mut())
                .and_then(|resp_map| resp_map.get_mut("value"))
                .map(|value| {
                    let str = value.as_str()?;
                    let bytes = base64::decode(str).ok()?;
                    let val = serde_json::from_slice::<Value>(bytes.as_slice()).ok()?;
                    *value = val;
                    Some(())
                });
            let json = serde_json::to_string(&resp_val)?;
            Ok(Some(json))
        } else {
            Ok(None)
        };
    }

    async fn receive(&mut self) -> Result<Option<String>> {
        Err(Error::NotImpl)
    }
}

/// get
pub struct HttpGetProvider {}

#[async_trait::async_trait]
impl Provider for HttpGetProvider {
    async fn request(&mut self, method: &str, params: &str) -> Result<Option<String>> {
        //this params must is map string
        let params_value = serde_json::to_value(params)?;
        log::debug!("{:?}", params_value);

        if let Some(params_map) = params_value.as_object() {
            let mut query_vec = Vec::new();
            for (key,val) in params_map {
                query_vec.push((key.clone(),val.clone()));
            }
            let url = "http://127.0.0.1:26657".to_string() + "/" + method;

            let resp_val = reqwest::Client::new()
                .get(url)
                .query(&query_vec)
                .send()
                .await?
                .json::<Value>()
                .await?;
            let op = resp_val.as_str().and_then(|str|Some(str.to_string()));

            Ok(op)
        } else {
            return Err(Error::ErrorString("Must be a string of type map".to_string()))
        }

    }

    async fn receive(&mut self) -> Result<Option<String>> {
        Err(Error::NotImpl)
    }
}
