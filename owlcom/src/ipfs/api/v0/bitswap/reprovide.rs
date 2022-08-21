use reqwest::Client;
use serde::Deserialize;

/// Trigger reprovider.   
/// This endpoint takes no argument.
pub struct Reprovide;

impl Reprovide {
    pub async fn exec(client: &Client, host: &String) -> Result<Response, reqwest::Error> {
        client
            .post(format!("{}/api/v0/bitswap/reprovide", host))
            .send()
            .await?
            .json::<Response>()
            .await
    }
}
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    message: String,
    code: i64,
    #[serde(rename = "Type")]
    msg_type: String,
}

#[cfg(test)]
mod test {
    use reqwest::Client;

    use super::Reprovide;
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn online_test() {
        Reprovide::exec(&Client::new(), &"http://localhost:5001".into())
            .await
            .unwrap();
    }
}
