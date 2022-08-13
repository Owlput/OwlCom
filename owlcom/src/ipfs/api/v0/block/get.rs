#[derive(Debug,Default)]
pub struct Get{
    args:String,
}
impl Get{
    pub fn builder()->Builder{
        Builder
    }
    fn to_request(&self, host: &String) -> hyper::Request<hyper::Body> {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(&format!(
                    "http://{}/api/v0/block/get{}",
                    host,self.args
                ))
                .unwrap(),
            ).method("POST")
            .body(hyper::Body::from(""))
            .unwrap()
    }
}

pub struct Builder;
impl Builder{
    pub fn build(block_cid:&String)->Get{
        Get{
            args:format!("?arg={}",block_cid)
        }
    }
}