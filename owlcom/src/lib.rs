#![feature(path_file_prefix)]
#![feature(io_error_more)]
#[allow(dead_code)]
#[allow(unused)]
use hyper::{client::HttpConnector, Client};
use serde_json::Value;
use traits::EndpointResponse;

pub mod error;
#[cfg(feature = "ipfs")]
pub mod ipfs;

#[cfg(feature = "p2p")]
pub mod libp2p;
mod macros;
pub mod traits;

impl EndpointResponse for hyper::body::Bytes {}
impl EndpointResponse for Value {}
impl EndpointResponse for String {}