use crate::traits::EndpointResponse;
use async_trait::async_trait;
use owlcom_derive::EndpointResponse;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::Deserialize;
use std::path::Path;

use crate::{impl_opt_param, ipfs::api::FileTransferError, traits::EndpointOnce};

/// Store input as an IPFS block.  
/// The request will be constructed on `exec()` called and can only be used once.
pub struct Put<'a, 'b> {
    client: &'a Client,
    path: &'b Path,
    host: String,
    opt_params: Option<String>,
}

impl<'a, 'b> Put<'a, 'b> {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[async_trait]
impl<'a, 'b> EndpointOnce<Response, FileTransferError> for Put<'a, 'b> {
    async fn exec(self) -> Result<Response, FileTransferError> {
        if !self.path.exists() {
            return Err(FileTransferError::FileNotExist);
        }
        if !self.path.is_file() {
            return Err(FileTransferError::NotAFile);
        }
        let filename = self.path.file_name().unwrap().to_str().unwrap().to_string(); // This is probably FileTransferError-prone
        let file = match tokio::fs::read(self.path).await {
            Ok(v) => v,
            Err(e) => return Err(FileTransferError::Fs(e)),
        };
        match self
            .client
            .post(format!(
                "{}/api/v0/block/put{}",
                self.host,
                self.opt_params.unwrap_or("".into())
            ))
            .multipart(Form::new().part(filename, Part::bytes(file)))
            .send()
            .await
        {
            Ok(res) => match res.json::<Response>().await {
                Ok(res) => return Ok(res),
                Err(e) => return Err(FileTransferError::Reqwest(e)),
            },
            Err(e) => return Err(FileTransferError::Reqwest(e)),
        }
    }
}

#[derive(Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl_opt_param!(
    /// Multihash hash function. Default: `sha2-256`. Required: no.
    mhtype: String,
    /// Multihash hash length. Default: `-1`. Required: no.
    mhlen: isize,
    /// Pin added blocks recursively. Default: `false`. Required: no.
    pin: bool,
    /// Use legacy format for returned CID (DEPRECATED). Required: no.
    #[deprecated]
    format: String
);

impl<'a, 'b> Builder {
    /// Return a new `Builder` for `/api/v0/block/put` endpoint
    pub fn new() -> Self {
        Builder::default()
    }
    pub fn build(self, client: &'a Client, host: &String, file: &'b Path) -> Put<'a, 'b> {
        Put {
            client,
            path: file,
            host: host.clone(),
            opt_params: self.opt_params,
        }
    }
    pub fn cid_codec(self, codec: String) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("cid-codec={}", codec)),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&cid-codec={}", v, codec)),
            },
        }
    }
    pub fn allow_big_block(self, arg: bool) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("allow-big-block={}", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&allow-big-block={}", v, arg.to_string())),
            },
        }
    }
}

#[derive(Deserialize, Debug, EndpointResponse)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    key: String,
    size: isize,
}
