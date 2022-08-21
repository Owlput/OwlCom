use crate::traits::{Endpoint, EndpointResponse};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{endpoint_gen, impl_opt_param};

endpoint_gen!(
    /// Show blocks currently on the wantlist.
    #[derive(Debug, Endpoint)]
    Wantlist
);

#[derive(Default)]
pub struct Builder {
    opt_params: Option<String>,
}
impl<'a> Builder {
    pub fn build(self, client: &'a Client, host: &String) -> Wantlist<'a> {
        Wantlist {
            client,
            request: client
                .post(format!(
                    "{}/api/v0/bitswap/wantlist{}",
                    host,
                    self.opt_params.unwrap_or("".into())
                ))
                .build()
                .unwrap(),
        }
    }
}

impl_opt_param!(
    /// Specify which peer to show wantlist for. Required: no.
    peer: String
);

#[derive(Debug, Deserialize, PartialEq, EndpointResponse)]
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
        let client = reqwest::Client::new();
        Wantlist::builder()
            .build(&client, &"http://localhost:5001".into())
            .exec()
            .await
            .unwrap();
    }
}
