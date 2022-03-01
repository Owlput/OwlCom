pub mod swarm_peers;
pub mod add;
pub mod bitswap_ledger;
use hyper::Client;
use hyper::http::request;

use crate::{IpfsApi, traits::url::ToParam};

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

fn construct_params<T>(params: &Vec<T>) -> String 
where T:ToParam
{
    let len = params.len();
    if len == 0 {
        return "".to_string();
    }
    let mut iter = params.clone().into_iter();
    let mut param_string = String::new();
    for i in 1..len + 1 {
        match i {
            1 => param_string = format!("{}", iter.next().unwrap().to_param()),
            _ => param_string = format!("{}&{}", param_string, iter.next().unwrap().to_param()),
        }
    }
    param_string
}

fn construct_headers<T>(builder:request::Builder,headers:Vec<T>)->request::Builder{
    todo!()
}