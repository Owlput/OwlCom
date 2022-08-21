use crate::traits::{Endpoint,EndpointResponse};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

use crate::{endpoint_gen, impl_opt_param};

endpoint_gen!(
    /// Add peers to the bootstrap list.
    #[derive(Endpoint)]
    Add
);


#[derive(Debug,Default)]
pub struct Builder{
    opt_params:Option<String>
}

impl_opt_param!(
    /// A peer to add to the bootstrap list (in the format '<multiaddr>/<peerID>') Required: no
    arg:String,
    /// Add default bootstrap nodes. (Deprecated, use 'default' subcommand instead).
    /// This function is for `default` argument, function name conflicts with `Default` trait.
    #[deprecated]
    is_default:bool
);

#[derive(Debug,Deserialize,EndpointResponse)]
pub struct Response{
    peers:Vec<String>
}