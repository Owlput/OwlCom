#[allow(dead_code)]
pub mod api;
pub mod swarm_peers;
use hyper::Client;

use crate::IpfsApi;

impl IpfsApi {
    pub fn build(&mut self) {
        self.client = Some(Client::new())
    }
}

impl PartialEq for IpfsApi {
    fn ne(&self, other: &Self) -> bool {
        self.entry != other.entry
    }

    fn eq(&self, other: &Self) -> bool {
        self.entry == other.entry
    }
}
impl Eq for IpfsApi {}
