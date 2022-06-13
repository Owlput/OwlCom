use hyper::{Body, Request};
use serde::Deserialize;

use crate::traits::url::ToParam;

use crate::{generate_optional_params_enum, request_constructor::*};

generate_optional_params_enum!(
    Quiet: bool,
    Quieter: bool,
    Silent: bool,
    Progress: bool,
    Trickle:bool,
    OnlyHash:bool,
    WrapWithDirectory:bool,
    Chunker:String,
    Pin:bool,
    RawLeaves:bool,
    Nocopy:bool,
    Fscache:bool,
    CidVersion:u64,
    Hash:String,
    Inline:bool,
    InlineLimit:u64,
    Finished:String
);

pub struct Add;

impl Add {
    fn to_request(params: Option<Vec<OptionalParams>>, host: &String) -> Request<Body> {
        hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri(format!("{}/api/v0/add?{}", host, construct_params(&params)).as_str())
            .body(hyper::Body::from(""))
            .unwrap()
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

    use super::OptionalParams;
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
                OptionalParams::Silent(true),
                OptionalParams::Inline(true),
                OptionalParams::Hash("omg".into()),
            ]),
            &host,
        );
        let req_tgt = hyper::Request::builder()
            .method("POST")
            // .header(key, value)
            .uri("http://127.0.0.1:5001/api/v0/add?silent=true&inline=true&hash=omg&")
            .body(hyper::Body::from(""))
            .unwrap();
        assert_eq!(format!("{:#?}", req), format!("{:#?}", req_tgt));
    }
}
