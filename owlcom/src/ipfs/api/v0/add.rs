use std::path::Path;

use async_trait::async_trait;
use owlcom_derive::EndpointResponse;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::Deserialize;

use crate::{
    impl_opt_param,
    ipfs::api::FileTransferError,
    traits::{EndpointOnce, EndpointResponse},
};

/// Add a file or directory(not supported yet) to IPFS.
/// The request will be constructed after `exec()` called and can only be used once.
#[derive(Debug)]
pub struct Add<'a, 'b> {
    client: &'a Client,
    path: &'b Path,
    host: String,
    opt_params: Option<String>,
}

#[async_trait]
impl<'a, 'b> EndpointOnce<Response, FileTransferError> for Add<'a, 'b> {
    async fn exec(self) -> Result<Response, FileTransferError> {
        let filename = self.path.to_str().unwrap().to_string();
        let file = match tokio::fs::read(self.path).await {
            Ok(v) => v,
            Err(e) => return Err(FileTransferError::Fs(e)),
        };
        match self
            .client
            .post(format!(
                "{}/api/v0/add?{}",
                self.host,
                self.opt_params.unwrap_or("".into())
            ))
            .multipart(Form::new().part(filename, Part::bytes(file)))
            .send()
            .await
        {
            Ok(res) => match res.json().await {
                Ok(res) => return Ok(res),
                Err(e) => return Err(FileTransferError::Reqwest(e)),
            },
            Err(e) => return Err(FileTransferError::Reqwest(e)),
        }
    }
}
impl<'a, 'b> Add<'a, 'b> {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

/// For more documemtation of this API, please refer to [official IPFS documentation][https://docs.ipfs.tech/reference/kubo/rpc/#api-v0-add].
#[derive(Debug, Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl<'a, 'b> Builder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self, client: &'a Client, host: &String, path: &'b Path) -> Add<'a, 'b> {
        Add {
            client,
            path,
            host: host.clone(),
            opt_params: self.opt_params,
        }
    }
    /// Only chunk and hash - do not write to disk. Required: no.
    pub fn only_hash(self, arg: bool) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("only-hash={}", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&only-hash={}", v, arg.to_string())),
            },
        }
    }
    /// Wrap files with a directory object. Required: no.
    pub fn wrap_with_directory(self, arg: bool) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("wrap-with-directory={}", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&wrap-with-directory={}", v, arg.to_string())),
            },
        }
    }
    /// Use raw blocks for leaf nodes. Required: no.
    pub fn raw_leaves(self, arg: bool) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("{}=raw-leaves", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&raw-leaves={}", v, arg.to_string())),
            },
        }
    }
    /// CID version. Defaults to 0 unless an option that depends on CIDv1 is passed.
    /// Passing version 1 will cause the raw-leaves option to default to true. Required: no.
    pub fn cid_version(self, arg: isize) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("cid-version={}", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&cid-version={}", v, arg.to_string())),
            },
        }
    }
    /// Maximum block size to inline. (experimental). Default: `32`. Required: no.
    pub fn inline_limit(self, arg: isize) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("inline-limit={}", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&inline-limit={}", v, arg.to_string())),
            },
        }
    }
}
impl_opt_param!(
    /// Write minimal output. Required: no.
    quiet: bool,
    /// Write only final hash. Required: no.
    quieter: bool,
    /// Write no output. Required: no.
    silent: bool,
    /// Stream progress data. Required: no.
    progress: bool,
    /// Use trickle-dag format for dag generation. Required: no.
    trickle: bool,
    /// Chunking algorithm, size-\[bytes\], rabin-\[min\]-\[avg\]-\[max\] or buzhash.   
    /// Default: `size-262144`
    chunker: String,
    /// Pin this object when adding. Default: `true`. Required: no.
    pin: bool,
    /// Add the file using filestore. Implies raw-leaves. (experimental). Required: no.
    nocopy: bool,
    /// Check the filestore for pre-existing blocks. (experimental). Required: no.
    fscache: bool,
    /// Hash function to use. Implies CIDv1 if not sha2-256. (experimental).   
    /// Default: `sha2-256`. Required: no.
    hash: String,
    /// Inline small blocks into CIDs. (experimental). Required: no.
    inline: bool
);

#[derive(Deserialize, Debug, EndpointResponse)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    bytes: Option<u64>,
    hash: Option<String>,
    name: Option<String>,
    size: Option<String>,
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::traits::EndpointOnce;
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "actual-file-needed"]
    async fn test() {
        let client = reqwest::Client::new();
        println!(
            "{:#?}",
            super::Add::builder()
                .quieter(true)
                .build(
                    &client,
                    &"http://127.0.0.1:5001".into(),
                    &Path::new(r#"./Cargo.toml"#)
                )
                .exec()
                .await
        )
    }
}
