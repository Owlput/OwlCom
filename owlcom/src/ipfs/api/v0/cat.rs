use crate::{endpoint_gen, error::*, impl_opt_param, traits::Endpoint};
use async_trait::async_trait;

endpoint_gen!(
    /// Show IPFS object data.
    /// This endpoint returns a `text/plain` response body.
    #[derive(Debug)]
    Cat
);

#[async_trait]
impl<'a> Endpoint<String, Error> for Cat<'a> {
    /// This endpoint returns a `text/plain` response body.
    async fn exec(&self) -> Result<String, Error> {
        let response = match self.client.execute(self.request.try_clone().unwrap()).await {
            Ok(v) => v,
            Err(e) => return Err(Error::new(Kind::Reqwest(e))),
        };
        match response.text().await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(Kind::Reqwest(e))),
        }
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
