use crate::traits::{Endpoint, EndpointResponse};
use crate::{endpoint_gen, error::*, impl_opt_param};
use async_trait::async_trait;
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

pub mod get {
    use super::*;
    endpoint_gen!(
        /// Get a raw IPFS block.
        /// This endpoint receives `octstream`.
        #[derive(Debug)]
        Get
    );
    #[async_trait]
    impl<'a> Endpoint<hyper::body::Bytes, Error> for Get<'a> {
        async fn exec(&self) -> Result<hyper::body::Bytes, Error> {
            let response = match self.client.execute(self.request.try_clone().unwrap()).await {
                Ok(v) => v,
                Err(e) => return Err(Error::new(Kind::Reqwest(e))),
            };
            match response.bytes().await {
                Ok(v) => Ok(v.into()),
                Err(e) => Err(Error::new(Kind::Reqwest(e))),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Builder;
    impl<'a> Builder {
        /// Required argument: `block_cid` The CID of the block wanted.
        pub fn build(self, client: &'a Client, host: &String, block_cid: &String) -> Get<'a> {
            Get {
                client,
                request: client
                    .post(format!("{}/api/v0/block/get?arg={}", host, block_cid))
                    .build()
                    .unwrap(),
            }
        }
    }
}
pub mod put {
    use super::*;
    use crate::traits::EndpointOnce;
    use reqwest::{
        multipart::{Form, Part},
        Client,
    };
    use std::path::Path;

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
    impl<'a, 'b> EndpointOnce<Response, Error> for Put<'a, 'b> {
        async fn exec(self) -> Result<Response, Error> {
            if !self.path.exists() {
                return Err(Error::new(Kind::Fs(std::io::Error::from(
                    std::io::ErrorKind::NotFound,
                ))));
            }
            if !self.path.is_file() {
                return Err(Error::new(Kind::Fs(std::io::Error::from(
                    std::io::ErrorKind::IsADirectory,
                ))));
            }
            let filename = self.path.file_name().unwrap().to_str().unwrap().to_string(); // This is probably FileTransferError-prone
            let file = match tokio::fs::read(self.path).await {
                Ok(v) => v,
                Err(e) => return Err(Error::new(Kind::Fs(e))),
            };
            match self
                .client
                .post(format!(
                    "{}/api/v0/block/put?{}",
                    self.host,
                    self.opt_params.unwrap_or("".into())
                ))
                .multipart(Form::new().part(filename, Part::bytes(file)))
                .send()
                .await
            {
                Ok(res) => match res.json::<Response>().await {
                    Ok(res) => return Ok(res),
                    Err(e) => return Err(Error::new(Kind::Reqwest(e))),
                },
                Err(e) => return Err(Error::new(Kind::Reqwest(e))),
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
}
pub mod rm {
    use super::*;
    endpoint_gen!(
        /// Remove IPFS block from the local datastore.
        #[derive(Debug, Endpoint)]
        Rm
    );

    #[derive(Debug, Default)]
    pub struct Builder {
        opt_params: Option<String>,
    }
    impl_opt_param!(
        /// Ignore nonexistent blocks. Required: no.
        force: bool,
        /// Write minimal output. Required: no.
        quiet: bool
    );

    impl<'a> Builder {
        pub fn new() -> Self {
            Builder::default()
        }
        pub fn build(self, client: &'a Client, host: &String, cid: String) -> Rm<'a> {
            Rm {
                client,
                request: client
                    .post(format!(
                        "{}/api/vo/block/rm?arg={}&{}",
                        host,
                        cid,
                        self.opt_params.unwrap_or("".into())
                    ))
                    .build()
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        error: String,
        hash: String,
    }
}
pub mod stat {
    use super::*;
    endpoint_gen!(
        /// Print information of a raw IPFS block.
        #[derive(Debug, Endpoint)]
        Stat
    );

    #[derive(Debug, Default)]
    pub struct Builder;

    impl<'a> Builder {
        pub fn build(self, client: &'a Client, host: &String, cid: String) -> Stat<'a> {
            Stat {
                client,
                request: client
                    .post(format!("{}/api/v0/block/stat?arg={}", host, cid))
                    .build()
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        key: String,
        size: isize,
    }
}
