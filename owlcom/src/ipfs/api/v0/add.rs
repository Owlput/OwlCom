use hyper::{Body, Request, Uri};
use serde::Deserialize;

use crate::traits::url::ToParam;

use crate::request_constructor::*;
#[derive(Debug, PartialEq, Clone)]
pub enum Params {
    Quiet(bool),
    Quieter(bool),
    Silent(bool),
    Progress(bool),
    Trickle(bool),
    OnlyHash(bool),
    WrapWithDirectory(bool),
    Chunker(String),
    Pin(bool),
    RawLeaves(bool),
    Nocopy(bool),
    Fscache(bool),
    CidVersion(u64),
    Hash(String),
    Inline(bool),
    InlineLimit(u64),
    Finished(String),
    None,
}
impl ToParam for Params {
    fn to_param(&self) -> String {
        match self {
            Params::Quiet(val) => format!("quiet={}", val),
            Params::Quieter(val) => format!("quieter={}", val),
            Params::Silent(val) => format!("silent={}", val),
            Params::Progress(val) => format!("progress={}", val),
            Params::Trickle(val) => format!("trickle={}", val),
            Params::OnlyHash(val) => format!("only-hash={}", val),
            Params::WrapWithDirectory(val) => format!("wrap-with-directory={}", val),
            Params::Chunker(val) => format!("chunker={}", val),
            Params::Pin(val) => format!("pin={}", val),
            Params::RawLeaves(val) => format!("raw-leaves={}", val),
            Params::Nocopy(val) => format!("nocopy={}", val),
            Params::Fscache(val) => format!("fscache={}", val),
            Params::CidVersion(val) => format!("cid-version={}", val),
            Params::Hash(val) => format!("hash={}", val),
            Params::Inline(val) => format!("inline={}", val),
            Params::InlineLimit(val) => format!("inline-limit={}", val),
            Params::Finished(val) => val.clone(),
            Params::None => String::new(),
        }
    }
}

pub struct Add;

pub struct Builder{
    host:Uri,
    params:String
}

impl Builder{
    pub fn quiet(&mut self,val:bool)->&mut self{
        self.params = format!("{}"self.params)
    }
}

impl Add {
    fn to_request(params: Option<Vec<Params>>, host: &String) -> Request<Body> {
        hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri(format!("{}/api/v0/add?{}", host, construct_params(&params)).as_str())
            .body(hyper::Body::from(""))
            .unwrap()
    }
    pub fn build(host:Uri)->Builder{
        Builder { host, params: "".into() }
    }
}
#[derive(Deserialize)]
pub struct Response {
    bytes: Option<u64>,
    hash: Option<String>,
    name: Option<String>,
    size: Option<String>,
}

#[cfg(test)]
mod test {

    use super::Params;
    use super::*;
    #[test]
    fn process_req() {
        let host = String::from("http://127.0.0.1:5001");
        let req_tgt = hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri("http://127.0.0.1:5001/api/v0/add?")
            .body(hyper::Body::from(""))
            .unwrap();
        assert_eq!(
            format!("{:#?}", Add::to_request(None, &host)),
            format!("{:#?}", req_tgt)
        );
        let req = Add::to_request(
            Some(vec![
                Params::Silent(true),
                Params::Inline(true),
                Params::Hash("omg".into()),
            ]),
            &host,
        );
        let req_tgt = hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri("http://127.0.0.1:5001/api/v0/add?silent=true&inline=true&hash=omg")
            .body(hyper::Body::from(""))
            .unwrap();
        assert_eq!(format!("{:#?}", req), format!("{:#?}", req_tgt));
    }
}
