use std::collections::HashMap;

use serde::Deserialize;

///Show blocks currently on the wantlist.
pub struct Wantlist {
    opt_args: String,
}

impl Wantlist {
    fn builder() -> Builder {
        Builder::default()
    }
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(&format!(
                    "http://{}/api/v0/bitswap/wantlist?{}",
                    host, self.opt_args
                ))
                .unwrap(),
            ).method("POST")
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

#[derive(Default)]
pub struct Builder {
    opt_args: String,
}
impl Builder {
    pub fn peer(self, peer: String) -> Self {
        Self {
            opt_args: format!("peer={}", peer),
        }
    }
    pub fn build(self) -> Wantlist {
        Wantlist {
            opt_args: self.opt_args,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    keys: Vec<HashMap<String, String>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn offline_test() {
        let json = r#"{"Keys":[{"/a":"a","/b":"b"},{"/c":"c","/d":"d"}]}"#;
        let result = serde_json::from_str::<Response>(json).unwrap();
        let mut map0 = HashMap::new();
        map0.insert("/a".into(), "a".into());
        map0.insert("/b".into(), "b".into());
        let mut map1 = HashMap::new();
        map1.insert("/c".into(), "c".into());
        map1.insert("/d".into(), "d".into());
        let reference = Response {
            keys: vec![map0, map1],
        };
        assert_eq!(result, reference);
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn online_test() {
        let req = Wantlist::builder()
            .build()
            .to_request(&"127.0.0.1:5001".into());
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
