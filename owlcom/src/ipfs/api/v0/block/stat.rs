use crate::traits::{Endpoint, EndpointResponse};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

use crate::endpoint_gen;

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
