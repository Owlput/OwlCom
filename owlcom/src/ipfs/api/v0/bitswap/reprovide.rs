/// Trigger reprovider.   
/// This endpoint takes no argument.
pub struct Reprovide;

impl Reprovide {
    fn to_request(self, host: &String) -> hyper::Request<hyper::Body> {
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
