use crate::traits::{Endpoint, EndpointResponse};
use crate::{endpoint_gen, impl_opt_param, simple_builder_impl};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

pub mod add {
    use super::*;
    endpoint_gen!(
        /// Add peers to the bootstrap list.
        #[derive(Endpoint)]
        Add
    );

    #[derive(Debug, Default)]
    pub struct Builder {
        opt_params: Option<String>,
    }

    impl<'a> Builder {
        pub fn build(self, client: &'a Client, host: &String, peer_id: String) -> Add<'a> {
            Add {
                client,
                request: client
                    .post(format!(
                        "{}/api/v0/bootstrap/add?arg={}&{}",
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
        /// Add default bootstrap nodes. (Deprecated, use 'default' subcommand instead).
        /// This function is for `default` argument, function name conflicts with `Default` trait.
        #[deprecated]
        is_default: bool
    );

    #[derive(Debug, Deserialize, EndpointResponse)]
    pub struct Response {
        peers: Vec<String>,
    }
}
pub mod add_default {
    use super::*;
    endpoint_gen!(
        /// Add default peers to the bootstrap list.
        /// This endpoint takes no arguments.
        #[derive(Debug, Endpoint)]
        AddDefault
    );

    simple_builder_impl!(
        AddDefault:"/api/v0/bootstrap/add/default"
    );

    #[derive(Debug, EndpointResponse, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        peers: Vec<String>,
    }
}
pub mod list {
    use super::*;
    endpoint_gen!(
        /// Show peers in the bootstrap list.
        #[derive(Debug, Endpoint)]
        List
    );

    simple_builder_impl!(List:"/api/v0/bootstrap/list");

    #[derive(Debug, EndpointResponse, Deserialize)]
    pub struct Response {
        peers: Vec<String>,
    }
}
pub mod rm_all {
    use super::*;
    endpoint_gen!(
        /// Remove all peers from the bootstrap list.
        #[derive(Debug, Endpoint)]
        RmAll
    );

    simple_builder_impl!(RmAll:"/api/v0/bootstrap/rm/all");

    #[derive(Debug, Deserialize, EndpointResponse)]
    pub struct Response {
        peers: Vec<String>,
    }
}
pub mod rm {
    use super::*;
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
}

endpoint_gen!(
    /// Show or edit the list of bootstrap peers.
    #[derive(Debug, Endpoint)]
    Bootstrap
);

simple_builder_impl!(
    Bootstrap:"/api/v0/bootstrap"
);

#[derive(Debug, Deserialize, EndpointResponse)]
pub struct Response {
    peers: Vec<String>,
}
