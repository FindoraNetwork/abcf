use super::Provider;
use crate::error::{Error, Result};
use crate::jsonrpc::{Request, Response};
use alloc::vec::Vec;
use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// post
pub struct HttpPostProvider {
    pub url: String,
}

impl HttpPostProvider {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
        }
    }
}

#[async_trait::async_trait]
impl Provider for HttpPostProvider {
    async fn request<Req, Resp>(&mut self, method: &str, params: &Req) -> Result<Option<Resp>>
    where
        Req: Serialize + Sync + Send,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        let req = Request::new(method, params);

        let resp = reqwest::Client::new()
            .post(self.url.clone())
            .json(&req)
            .send()
            .await?
            .json::<Response<Resp>>()
            .await?;

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

/// get
pub struct HttpGetProvider {
    pub url: String,
}

impl HttpGetProvider {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
        }
    }
}

#[async_trait::async_trait]
impl Provider for HttpGetProvider {
    async fn request<Req, Resp>(&mut self, method: &str, params: &Req) -> Result<Option<Resp>>
    where
        Req: Serialize + Sync + Send,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        let req = serde_json::to_value(params)?;

        let querys = match req {
            serde_json::Value::Object(m) => {
                let querys: Vec<(String, Value)> = m.iter().map(|v| (v.0.clone(), v.1.clone())).collect();
                log::debug!(" Queries is {:?}", querys);
                querys
            },
            serde_json::Value::Null => Vec::new(),
            _ => return Err(Error::NotImpl),
        };


        let url = self.url.clone() + "/" + method;

        let resp = reqwest::Client::new()
            .get(url)
            .query(&querys)
            .send()
            .await?
            .json::<Response<Resp>>()
            .await?;

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
