use owlcom_derive::{EndpointResponse, Endpoint};
use crate::{traits::{EndpointResponse, Endpoint}, simple_builder_impl};
use serde::Deserialize;

use crate::endpoint_gen;

endpoint_gen!(
    /// Remove all peers from the bootstrap list.
    #[derive(Debug,Endpoint)]
    RmAll
);

simple_builder_impl!(RmAll:"/api/v0/bootstrap/rm/all");

#[derive(Debug, Deserialize, EndpointResponse)]
pub struct Response {
    peers: Vec<String>,
}
