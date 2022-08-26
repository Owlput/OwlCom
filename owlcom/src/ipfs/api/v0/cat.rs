use async_trait::async_trait;
use crate::{endpoint_gen, impl_opt_param, traits::Endpoint};

endpoint_gen!(
    /// Show IPFS object data.
    /// This endpoint returns a `text/plain` response body.
    Cat
);

#[async_trait]
impl<'a> Endpoint<String, reqwest::Error> for Cat<'a> {
    /// This endpoint returns a `text/plain` response body.
    async fn exec(&self) -> Result<String, reqwest::Error> {
        self.client
            .execute(self.request.try_clone().unwrap())
            .await?
            .text()
            .await
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl<'a> Builder {
    /// Required argument: `path`:The path to the IPFS object(s) to be outputted.
    pub fn build(self, client: &'a Client, host: &String, path: String) -> Cat<'a> {
        Cat {
            client,
            request: client
                .post(format!(
                    "{}/api/v0/cat?arg={}&{}",
                    host,
                    path,
                    self.opt_params.unwrap_or("".into())
                ))
                .build()
                .unwrap(),
        }
    }
}

impl_opt_param!(
    /// Byte offset to begin reading from. Required: no.
    offset: isize,
    /// Maximum number of bytes to read. Required: no.
    length: isize,
    /// Stream progress data. Default: `true`. Required: no.
    progress: bool
);
