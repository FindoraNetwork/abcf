use super::Provider;
use crate::error::{Error, Result};
use crate::jsonrpc::{Request, Response};
use alloc::vec::Vec;
use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// post
pub struct HttpPostProvider {}

#[async_trait::async_trait]
impl Provider for HttpPostProvider {
    async fn request<Req, Resp>(
        &mut self,
        method: &str,
        params: Option<&Req>,
    ) -> Result<Option<Resp>>
    where
        Req: Serialize + Sync + Send,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        let url = "http://127.0.0.1:26657";

        if let Some(params) = params {
            let req = Request::new(method, params);

            let resp = reqwest::Client::new()
                .post(url)
                .json(&req)
                .send()
                .await?
                .json::<Response<Resp>>()
                .await?;

            return if let Some(e) = resp.result {
                Ok(Some(e))
            } else if let Some(e) = resp.error {
                Err(Error::RPCError(e))
            } else {
                Err(Error::NotImpl)
            };
        }

        Ok(None)
    }

    async fn receive(&mut self) -> Result<Option<String>> {
        Err(Error::NotImpl)
    }
}

/// get
pub struct HttpGetProvider {}

#[async_trait::async_trait]
impl Provider for HttpGetProvider {
    async fn request<Req, Resp>(
        &mut self,
        method: &str,
        params: Option<&Req>,
    ) -> Result<Option<Resp>>
    where
        Req: Serialize + Sync + Send,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        let url = String::from("http://127.0.0.1:26657") + "/" + method;

        let resp = if let Some(params) = params {
            let req = serde_json::to_value(params)?;

            let map = match req {
                serde_json::Value::Object(m) => m,
                _ => return Err(Error::NotImpl),
            };

            let querys: Vec<(String, Value)> =
                map.iter().map(|v| (v.0.clone(), v.1.clone())).collect();
            log::debug!(" Queries is {:?}", querys);

            let resp = reqwest::Client::new()
                .get(url)
                .query(&querys)
                .send()
                .await?
                .json::<Response<Resp>>()
                .await?;
            resp
        } else {
            let resp = reqwest::Client::new()
                .get(url)
                .send()
                .await?
                .json::<Response<Resp>>()
                .await?;
            resp
        };

        if let Some(e) = resp.result {
            Ok(Some(e))
        } else if let Some(e) = resp.error {
            Err(Error::RPCError(e))
        } else {
            Err(Error::NotImpl)
        }
    }

    async fn receive(&mut self) -> Result<Option<String>> {
        Err(Error::NotImpl)
    }
}
