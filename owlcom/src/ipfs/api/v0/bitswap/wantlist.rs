use std::collections::HashMap;

use crate::traits::ToRequest;

///Show blocks currently on the wantlist.
pub struct Wantlist {
    opt_args: String,
}

impl ToRequest for Wantlist {
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(&format!(
                    "{}/api/v0/bitswap/wantlist?{}",
                    host, self.opt_args
                ))
                .unwrap(),
            )
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

#[derive(Default)]
pub struct Builder {
    opt_args: String,
}
impl Builder {
    pub fn new()->Self{
        Builder::default()
    }
    pub fn peer(self,peer:String)->Self{
        Self { opt_args: format!("peer={}",peer) }
    }
}

pub struct Response {
    keys: Vec<(String, String)>,
}
