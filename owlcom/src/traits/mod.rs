pub mod url;

pub trait ToRequest{
    fn to_request(&self,host:&String)->hyper::Request<hyper::Body>;
}