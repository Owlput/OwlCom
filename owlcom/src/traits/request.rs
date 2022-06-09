use hyper::Body;

use super::url::ToParam;

pub trait ConstructRequest {
    fn to_request<T>(params: Vec<T>, host: &String) -> hyper::Request<Body>
    where
        T: ToParam;
}
