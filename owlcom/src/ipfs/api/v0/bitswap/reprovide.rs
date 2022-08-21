use crate::{traits::{EndpointResponse, Endpoint}, simple_builder_impl};
use owlcom_derive::{EndpointResponse, Endpoint};
use serde::Deserialize;

use crate::endpoint_gen;

endpoint_gen!(
    /// Trigger reprovider.   
    /// This endpoint takes no argument.
    #[derive(Debug,Endpoint)]
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