use owlcom_derive::{Endpoint, EndpointResponse};
use crate::{traits::{Endpoint, EndpointResponse}, simple_builder_impl};
use serde::Deserialize;

use crate::endpoint_gen;

pub mod add;
pub mod add_default;
pub mod list;
pub mod rm;
pub mod rm_all;

endpoint_gen!(
    /// Show or edit the list of bootstrap peers.
    #[derive(Debug,Endpoint)]
    Bootstrap
);

simple_builder_impl!(
    Bootstrap:"/api/v0/bootstrap"
);

#[derive(Debug, Deserialize, EndpointResponse)]
pub struct Response {
    peers: Vec<String>,
}
