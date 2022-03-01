use hyper::Body;

pub trait ToRequest{
    fn to_request(&mut self,host:&String)->hyper::Request<Body>;
}