use crate::traits::url::ToParam;

pub struct Reprovide;

impl Reprovide {
    fn to_request<T>(params: Vec<T>, host: &String) -> hyper::Request<hyper::Body>
    {
        hyper::Request::builder()
            .uri(
                <hyper::Uri as std::str::FromStr>::from_str(
                    format!("{}/api/v0/bitswap/reprovide", host).as_str(),
                )
                .unwrap(),
            )
            .body(hyper::Body::from(""))
            .unwrap()
    }
}
