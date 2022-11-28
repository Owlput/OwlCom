#![feature(path_file_prefix)]
#![feature(io_error_more)]
#[allow(dead_code)]
#[allow(unused)]
use hyper::{client::HttpConnector, Client};
use serde_json::Value;
use traits::EndpointResponse;

pub mod error;
#[allow(unused_imports)]
#[cfg(feature="ipfs")]
pub mod ipfs;

#[cfg(feature="p2p")]
pub mod libp2p;
mod macros;
pub mod traits;

#[derive(Debug, Clone)]
pub struct IpfsApi {
    entry: hyper::Uri,
    client: Option<Client<HttpConnector>>,
}
impl IpfsApi {
    pub fn new_http(api_addr: &str) -> Self {
        Self {
            entry: api_addr.parse().unwrap(),
            client: None,
        }
    }
}

impl EndpointResponse for hyper::body::Bytes {}
impl EndpointResponse for Value {}
impl EndpointResponse for String {}

#[cfg(test)]
mod tests {

    use crate::IpfsApi;

    #[test]
    fn parse_addr() {
        let ipfs_api = IpfsApi::new_http("127.0.0.1:5001");
        assert_eq!(
            ipfs_api,
            IpfsApi {
                entry: "127.0.0.1:5001".parse().unwrap(),
                client: None
            }
        )
    }
}
