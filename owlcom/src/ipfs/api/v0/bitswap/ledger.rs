use serde::Deserialize;

pub struct Ledger;

impl Ledger {
    fn to_request(&self,peer_id:String, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(
                    format!("{}/api/v0/bitswap/ledger?arg={}", host,peer_id).as_str(),
                )
                .unwrap(),
            )
            .body(hyper::Body::from(""))
            .unwrap()
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
