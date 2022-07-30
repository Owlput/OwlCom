use serde::Deserialize;

/// Trigger reprovider.   
/// This endpoint takes no argument.
pub struct Reprovide;

impl Reprovide {
    pub fn new() -> Self {
        Self
    }
    pub fn to_request(self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(
                    format!("http://{}/api/v0/bitswap/reprovide", host).as_str(),
                )
                .unwrap(),
            )
            .method("POST")
            .body(hyper::Body::from(""))
            .unwrap()
    }
}
#[derive(Debug,PartialEq,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    message: String,
    code: i64,
    #[serde(rename = "Type")]
    msg_type: String,
}

#[cfg(test)]
mod test {
    use super::Reprovide;
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn online_test() {
        let req = Reprovide::new().to_request(&"127.0.0.1:5001".to_string());
        let res = hyper::Client::new().request(req).await.unwrap();
        println!(
            "{:?}",
            String::from_utf8(
                match hyper::body::to_bytes(res.into_body()).await {
                    Ok(b) => b,
                    Err(e) => {
                        panic!("unexpected error: {}", e)
                    }
                }
                .to_vec()
            )
            .unwrap()
        );
    }
}
