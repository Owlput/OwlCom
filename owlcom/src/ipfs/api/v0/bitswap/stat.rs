use reqwest::{Client, Request};
use serde::Deserialize;
use std::collections::HashMap;

use crate::impl_opt_param;

/// Show some diagnostic informati&on on the bitswap agent.
pub struct Stat<'a> {
    client: &'a Client,
    request: Request,
}

impl<'a> Stat<'a> {
    pub fn builder() -> Builder {
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

#[derive(Debug, Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl<'a> Builder {
    pub fn build(self, client: &'a Client, host: &String) -> Stat<'a> {
        Stat {
            client,
            request: client
                .post(format!(
                    "{}/api/v0/bitswap/stat?{}",
                    host,
                    self.opt_params.unwrap_or("".into())
                ))
                .build()
                .unwrap(),
        }
    }
}
impl_opt_param!(
    /// Print extra information. Required: no.
    verbose: bool,
    /// Print sizes in human readable format (e.g., 1K 234M 2G). Required: no.
    human: bool
);

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
        let client = reqwest::Client::new();
        Stat::builder()
            .build(&client, &"http://localhost:5001".into())
            .exec()
            .await
            .unwrap();
    }
}
