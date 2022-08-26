use crate::traits::{Endpoint, EndpointResponse};
use crate::{endpoint_gen, impl_opt_param, simple_builder_impl};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

pub mod ledger {
    use super::*;
    endpoint_gen!(
        /// Show the current ledger for a peer.
        #[derive(Debug, Endpoint)]
        Ledger
    );

    #[derive(Default)]
    /// Builder for ``Ledger`` API call.
    pub struct Builder;

    impl<'a> Builder {
        /// Required argument: arg[string]: The PeerID (B58) of the ledger to inspect.
        pub fn build(self, client: &'a Client, host: &String, peer_id: String) -> Ledger<'a> {
            Ledger {
                client,
                request: client
                    .post(format!("{}/api/v0/bitswap/ledger?arg={}", host, peer_id))
                    .build()
                    .unwrap(),
            }
        }
    }

    #[derive(Deserialize, Debug, PartialEq, EndpointResponse)]
    /// On success, the call to this endpoint will return with 200 and this response.
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        exchanged: u64,
        peer: String,
        recv: u64,
        sent: u64,
        value: f64,
    }
}
pub mod reprovide {
    use super::*;
    endpoint_gen!(
        /// Trigger reprovider.   
        /// This endpoint takes no argument.
        #[derive(Debug, Endpoint)]
        Reprovide
    );

    simple_builder_impl!(
        Reprovide:"/api/v0/bitswap/reprovide"
    );

    #[derive(Debug, PartialEq, Deserialize, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        message: String,
        code: i64,
        #[serde(rename = "Type")]
        msg_type: String,
    }
}
pub mod stat {
    use crate::builder_impl_with_opt_params;

    use super::*;
    use std::collections::HashMap;

    endpoint_gen!(
        /// Show some diagnostic informati&on on the bitswap agent.
        #[derive(Debug, Endpoint)]
        Stat
    );

    builder_impl_with_opt_params!(
        Stat:"/api/v0/bitswap/stat",
        /// Print extra information. Required: no.
        verbose: bool,
        /// Print sizes in human readable format (e.g., 1K 234M 2G). Required: no.
        human: bool
    );

    #[derive(Deserialize, Debug, PartialEq, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        blocks_received: u64,
        blocks_sent: u64,
        data_received: u64,
        data_sent: u64,
        dup_blks_received: u64,
        dup_data_received: u64,
        messages_received: u64,
        peers: Vec<String>,
        provide_buf_len: i64,
        wantlist: Vec<HashMap<String, String>>,
    }
}
pub mod wantlist {
    use super::*;
    use std::collections::HashMap;

    endpoint_gen!(
        /// Show blocks currently on the wantlist.
        #[derive(Debug, Endpoint)]
        Wantlist
    );

    #[derive(Default)]
    pub struct Builder {
        opt_params: Option<String>,
    }
    impl<'a> Builder {
        pub fn build(self, client: &'a Client, host: &String) -> Wantlist<'a> {
            Wantlist {
                client,
                request: client
                    .post(format!(
                        "{}/api/v0/bitswap/wantlist?{}",
                        host,
                        self.opt_params.unwrap_or("".into())
                    ))
                    .build()
                    .unwrap(),
            }
        }
    }

    impl_opt_param!(
        /// Specify which peer to show wantlist for. Required: no.
        peer: String
    );

    #[derive(Debug, Deserialize, PartialEq, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        keys: Vec<HashMap<String, String>>,
    }
}
