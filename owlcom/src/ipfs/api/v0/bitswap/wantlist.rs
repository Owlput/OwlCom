use std::collections::HashMap;

use reqwest::{Client, Request};
use serde::Deserialize;

use crate::impl_opt_param;

///Show blocks currently on the wantlist.
pub struct Wantlist<'a> {
    client: &'a Client,
    request: Request,
}

impl<'a> Wantlist<'a> {
    fn builder() -> Builder {
        Builder::default()
    }
    pub async fn exec(&self) -> Result<Response, reqwest::Error> {
        self.client
            .execute(self.request.try_clone().unwrap())
            .await?
            .json::<Response>()
            .await
    }
}

#[derive(Default)]
pub struct Builder {
    opt_params: Option<String>,
}
impl<'a> Builder {
    pub fn build(self, client: &'a Client, host: &String) -> Wantlist<'a> {
        Wantlist {
            client,
            request: client.post(format!(
                "{}/api/v0/bitswap/wantlist?{}",
                host,
                self.opt_params.unwrap_or("".into())
            )).build().unwrap(),
        }
    }
}

impl_opt_param!(
    /// Specify which peer to show wantlist for. Required: no.
    peer: String
);

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
        let client = reqwest::Client::new();
        Wantlist::builder()
            .build(&client, &"http://localhost:5001".into())
            .exec().await
            .unwrap();
    }
}
