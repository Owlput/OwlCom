pub struct BitswapLedger {
    host: String,
    peer_id: String,
}
impl BitswapLedger {
    pub fn new(host: String, peer_id: String) -> Self {
        BitswapLedger { host, peer_id }
    }
    pub fn call(self)->hyper::Uri{
        <hyper::Uri as std::str::FromStr>::from_str(format!("{}/api/v0/bitswap/ledger?arg={}", self.host, self.peer_id).as_str()).unwrap()
    }
}
