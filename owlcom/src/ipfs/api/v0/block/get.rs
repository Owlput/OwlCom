use reqwest::{Client, Request};

#[derive(Debug)]
pub struct Get<'a> {
    client: &'a Client,
    request: Request,
}
impl<'a> Get<'a> {
    pub fn builder() -> Builder {
        Builder
    }
 pub async fn exec(&self)->Result<hyper::body::Bytes, reqwest::Error>{
        self.client
            .execute(self.request.try_clone().unwrap())
            .await?
            .bytes().await
    }
}

pub struct Builder;
impl<'a> Builder {
    pub fn build(self,client: &'a Client,host:&String, block_cid: &String) -> Get<'a> {
        Get {
            client,
            request: client
                .post(format!("{}/api/v0/block/get?arg={}", host, block_cid)).build().unwrap(),
        }
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
