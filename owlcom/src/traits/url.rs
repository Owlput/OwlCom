pub trait ToUri{
    fn to_uri(self)->hyper::Uri;
}
pub trait ToParam{
    fn to_param(&self)->String;
}