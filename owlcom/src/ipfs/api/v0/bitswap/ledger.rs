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
/// Builder for ``Ledger`` API call.
pub struct Builder;

impl Builder {
    pub fn build(self, peer_id: String) -> Ledger {
        Ledger {
            required_args: format!("arg={}", peer_id),
        }
    }
}

#[derive(Deserialize,Debug,PartialEq)]
/// On success, the call to this endpoint will return with 200 and this response.
#[serde(rename_all = "PascalCase")]
pub struct Response {
    exchanged: u64,
    peer: String,
    recv: u64,
    sent: u64,
    value: f64,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn offline_test() {
        let json = r#"{"Exchanged": 0,"Peer": "a","Recv": 0,"Sent": 0,"Value": 0.0}"#;
        let result = serde_json::from_str::<Response>(json).unwrap();
        let reference = Response {
            exchanged: 0,
            peer: "a".into(),
            recv: 0,
            sent: 0,
            value: 0.0,
        };
        assert_eq!(result,reference);
    }
}
