use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;
use crate::traits::{Endpoint, EndpointResponse};

use crate::{endpoint_gen, simple_builder_impl};

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
