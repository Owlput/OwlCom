use serde::Deserialize;

use crate::traits::ToRequest;

/// Show the current ledger for a peer.
/// Required argument: arg[string]: The PeerID (B58) of the ledger to inspect.
pub struct Ledger {
    required_args: String,
}
impl Ledger {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl ToRequest for Ledger {
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(
                    format!("{}/api/v0/bitswap/ledger?{}", host, self.required_args).as_str(),
                )
                .unwrap(),
            )
            .method("POST")
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

#[derive(Default)]
pub struct Builder;

impl Builder {
    pub fn build(self, arg: String) -> Ledger {
        Ledger {
            required_args: format!("arg={}", arg),
        }
    }
}

#[derive(Deserialize)]
/// On success, the call to this endpoint will return with 200 and this response.
#[serde(rename_all = "PascalCase")]
pub struct Response {
    exchanged: u64,
    peer: String,
    recv: u64,
    sent: u64,
    value: f64,
}
