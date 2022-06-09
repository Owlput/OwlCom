use serde::Deserialize;

pub struct Ledger {
    peer_id: String,
}

impl Ledger {
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(
                    format!("{}/api/v0/bitswap/ledger?arg={}", host, self.peer_id).as_str(),
                )
                .unwrap(),
            )
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

impl Ledger {
    pub fn new(peer_id: String) -> Self {
        Ledger { peer_id }
    }
}

#[derive(Deserialize)]
pub struct Response {
    exchanged: u64,
    peer: String,
    recv: u64,
    sent: u64,
    value: f64,
}
