use std::collections::HashMap;
use serde::Deserialize;

#[derive(Default)]
/// Show some diagnostic information on the bitswap agent.
pub struct Stat {
    opt_args: String,
}

impl Stat {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn to_request(self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(&format!(
                    "http://{}/api/v0/bitswap/stat?{}",
                    host, self.opt_args
                ))
                .unwrap(),
            )
            .method("POST")
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    blocks_received: u64,
    blocks_sent: u64,
    data_received: u64,
    data_sent: u64,
    dup_blks_received: u64,
    dup_data_received: u64,
    messages_received: u64,
    peers: Vec<String>,
    provide_buf_len: i64,
    wantlist: Vec<HashMap<String, String>>,
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{Response, Stat};

    #[test]
    fn local_test() {
        let raw_string = r#"{"BlocksReceived": 0,"BlocksSent": 0,"DataReceived": 0,"DataSent": 0,"DupBlksReceived": 0,"DupDataReceived": 0,"MessagesReceived": 0,"Peers": ["a","b","c"],"ProvideBufLen": 1,"Wantlist": [{"/": "a"},{"/a":"b"}]}"#;
        let stru = serde_json::from_str::<Response>(raw_string).unwrap();
        let mut map1 = HashMap::new();
        map1.insert("/".into(), "a".into());
        let mut map2 = HashMap::new();
        map2.insert("/a".into(), "b".into());
        assert_eq!(
            Response {
                blocks_received: 0,
                blocks_sent: 0,
                data_received: 0,
                data_sent: 0,
                dup_blks_received: 0,
                dup_data_received: 0,
                messages_received: 0,
                peers: vec!["a".into(), "b".into(), "c".into()],
                provide_buf_len: 1,
                wantlist: vec![map1, map2]
            },
            stru
        );
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn online_test() {
        let req = Stat::new().to_request(&"127.0.0.1:5001".to_string());
        let res = hyper::Client::new().request(req).await.unwrap();
        println!(
            "{:?}",
            serde_json::from_slice::<Response>(
                &match hyper::body::to_bytes(res.into_body()).await {
                    Ok(b) => b,
                    Err(e) => {
                        panic!("unexpected error: {}", e)
                    }
                }
            )
            .unwrap()
        );
    }
}
