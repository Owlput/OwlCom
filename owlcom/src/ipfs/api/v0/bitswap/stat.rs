use serde::Deserialize;

use crate::traits::ToRequest;

/// Show some diagnostic information on the bitswap agent.
pub struct Stat {
    opt_args: String,
}

impl ToRequest for Stat {
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(&format!(
                    "{}/api/v0/bitswap/stat?{}",
                    host, self.opt_args
                ))
                .unwrap(),
            )
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

#[derive(Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Response {
    blocks_received: u64,
    bolcks_sent: u64,
    data_received: u64,
    data_sent: u64,
    dup_blks_received: u64,
    dup_data_received: u64,
    messages_received: u64,
    peers: Vec<String>,
    provide_buf_len: i64,
    want_list: Vec<String>, //This has incompatible data type, to be worked on.
}
