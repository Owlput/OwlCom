use serde::Deserialize;

use crate::endpoint_gen;

endpoint_gen!(
/// Show the current ledger for a peer.
Ledger
);

impl<'a> Ledger<'a> {
    async fn exec(&self) -> Result<Response, reqwest::Error> {
        self.client
            .execute(self.request.try_clone().unwrap())
            .await
            .unwrap()
            .json::<Response>()
            .await
    }
}

#[derive(Default)]
/// Builder for ``Ledger`` API call.
pub struct Builder;

impl<'a> Builder {
    /// Required argument: arg[string]: The PeerID (B58) of the ledger to inspect.
    pub fn build(self, client: &'a Client, host: &String, peer_id: String) -> Ledger<'a> {
        Ledger {
            client,
            request: client
                .post(format!("{}/api/v0/bitswap/ledger?arg={}", host, peer_id))
                .build()
                .unwrap(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
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
        assert_eq!(result, reference);
    }
}
