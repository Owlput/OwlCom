use hyper::{client::HttpConnector, Client};

pub mod ipfs;
pub mod traits;
pub mod request_constructor;
mod macros;

#[derive(Debug,Clone)]
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

#[cfg(test)]
mod tests {

    use crate::IpfsApi;

    #[test]
    fn parse_addr() {
        let ipfs_api = IpfsApi::new_http("127.0.0.1:5001");
        assert_eq!(ipfs_api,IpfsApi{
            entry:"127.0.0.1:5001".parse().unwrap(),
            client:None
        })
    }
}
