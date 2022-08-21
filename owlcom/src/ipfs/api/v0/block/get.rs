use crate::{
    endpoint_gen,
    traits::{Endpoint, EndpointResponse},
};
use async_trait::async_trait;
use owlcom_derive::EndpointResponse;

endpoint_gen!(
    /// Get a raw IPFS block.
    #[derive(Debug)]
    Get
);

#[async_trait]
impl<'a> Endpoint<Response, reqwest::Error> for Get<'a> {
    async fn exec(&self) -> Result<Response, reqwest::Error> {
        match self
            .client
            .execute(self.request.try_clone().unwrap())
            .await?
            .bytes()
            .await
        {
            Ok(v) => return Ok(v.into()),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Debug, Default)]
pub struct Builder;
impl<'a> Builder {
    pub fn build(self, client: &'a Client, host: &String, block_cid: &String) -> Get<'a> {
        Get {
            client,
            request: client
                .post(format!("{}/api/v0/block/get?arg={}", host, block_cid))
                .build()
                .unwrap(),
        }
    }
}

#[derive(Debug, EndpointResponse)]
pub struct Response {
    bytes: hyper::body::Bytes,
}

impl From<hyper::body::Bytes> for Response {
    fn from(bytes: hyper::body::Bytes) -> Self {
        Response { bytes }
    }
}

impl Into<hyper::body::Bytes> for Response {
    fn into(self) -> hyper::body::Bytes {
        self.bytes
    }
}

// #[cfg(test)]
// mod test {
//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     #[ignore]
//     async fn online_test() {
//         let client = reqwest::Client::new();
//     }
// }
