use crate::traits::{Endpoint, EndpointResponse};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

use crate::{endpoint_gen, impl_opt_param};

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
    /// Ignore nonexistent blocks.
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
