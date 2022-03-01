use hyper::{client::HttpConnector, Body, Request};

use crate::traits::{request::ToRequest, url::ToParam};

use super::construct_params;
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

pub enum Headers {}
pub struct Add {
    optional_params: Option<Vec<Params>>,
}
impl ToRequest for Add {
    fn to_request(&mut self, host: &String) -> Request<Body> {
        let optional_params = match &self.optional_params {
            None => "".to_string(),
            Some(vec) => construct_params(vec),
        };

        hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri(format!("{}/api/v0/add?{}", host, optional_params).as_str())
            .body(hyper::Body::from(""))
            .unwrap()
    }
}
impl Add {
    pub fn new(optional_params: Option<Vec<Params>>) -> Self {
        Self { optional_params }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use super::Params;
    #[test]
    fn process_req() {
        let host = String::from("http://127.0.0.1:5001");
        let mut req = Add::new(None);
        let req_tgt = hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri("http://127.0.0.1:5001/api/v0/add?")
            .body(hyper::Body::from(""))
            .unwrap();
        assert_eq!(format!("{:#?}",req.to_request(&host)),format!("{:#?}",req_tgt));
        let mut req = Add::new(Some(vec![Params::Silent(true),Params::Inline(true),Params::Hash("omg".into())]));
        let req_tgt = hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri("http://127.0.0.1:5001/api/v0/add?silent=true&inline=true&hash=omg")
            .body(hyper::Body::from(""))
            .unwrap();
        assert_eq!(format!("{:#?}",req.to_request(&host)),format!("{:#?}",req_tgt));
        assert_eq!(format!("{:#?}",req.to_request(&host)),format!("{:#?}",req_tgt));
    }
}
