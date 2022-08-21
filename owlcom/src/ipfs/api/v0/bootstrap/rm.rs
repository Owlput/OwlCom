use crate::{
    endpoint_gen, impl_opt_param,
    traits::{Endpoint, EndpointResponse},
};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

endpoint_gen!(
    /// Remove peers from the bootstrap list.
    #[derive(Debug, Endpoint)]
    Rm
);

#[derive(Debug, Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl<'a> Builder {
    /// Required Argument: `peer_id`: The peer to remove from bootstrap list.
    pub fn build(self, client: &'a Client, host: &String, peer_id: String) -> Rm<'a> {
        Rm {
            client,
            request: client
                .post(format!(
                    "{}/api/v0/bootstrap/rm?arg={}&{}",
                    host,
                    peer_id,
                    self.opt_params.unwrap_or("".into())
                ))
                .build()
                .unwrap(),
        }
    }
}

impl_opt_param!(
    /// Remove all bootstrap peers. (Deprecated, use `RmAll` endpoint). Required: no.
    #[deprecated]
    all: bool
);

#[derive(Debug, Deserialize, EndpointResponse)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    peers: Vec<String>,
}
