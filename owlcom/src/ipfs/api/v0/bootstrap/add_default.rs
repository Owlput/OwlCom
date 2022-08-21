use crate::{
    simple_builder_impl,
    traits::{Endpoint, EndpointResponse},
};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

use crate::endpoint_gen;

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
