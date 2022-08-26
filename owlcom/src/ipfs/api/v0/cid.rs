use crate::traits::{Endpoint, EndpointResponse};
use crate::{endpoint_gen, impl_opt_param,builder_impl_with_opt_params};
use owlcom_derive::{Endpoint, EndpointResponse};
use serde::Deserialize;

pub mod base32 {
    use super::*;
    endpoint_gen!(
        /// Convert CIDs to Base32 CID version 1.
        #[derive(Debug, Endpoint)]
        Base32
    );

    #[derive(Debug, Default)]
    pub struct Builder;

    impl<'a> Builder {
        pub fn build(self, client: &'a Client, host: &String, cid: String) -> Base32<'a> {
            Base32 {
                client,
                request: client
                    .post(format!("{}/api/v0/cid/base32?arg={}", host, cid))
                    .build()
                    .unwrap(),
            }
        }
    }

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(rename_all = "PascalCase")]
    pub struct Response {
        cid_str: String,
        error_msg: Option<String>,
        formatted: Option<String>,
    }
}
pub mod bases {
    use super::*;
    endpoint_gen!(
        /// List available multibase encodings.
        #[derive(Debug, Endpoint)]
        Bases
    );

    #[derive(Debug, Default)]
    pub struct Builder {
        opt_params: Option<String>,
    }

    impl_opt_param!(
        /// also include the single letter prefixes in addition to the code.
        prefix: bool,
        /// also include numeric codes.
        numeric: bool
    );

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(transparent)]
    pub struct Response {
        encodings: Vec<MultibaseEncoding>,
    }

    #[derive(Debug, Deserialize)]
    pub struct MultibaseEncoding {
        code: isize,
        name: String,
    }
}
pub mod codecs {
    use super::*;
    endpoint_gen!(
        /// List available CID multicodecs.
        #[derive(Debug, Endpoint)]
        Codecs
    );

    builder_impl_with_opt_params!(
        Codecs:"/api/v0/cid/hashes",
        /// also include numeric codes.
        numeric:bool,
        /// list only codecs supported by go-ipfs commands.
        supported:bool
    );

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(transparent)]
    pub struct Response {
        codecs: Vec<CidMulticodec>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CidMulticodec {
        code: isize,
        name: String,
    }
}
pub mod format {
    use super::*;
    endpoint_gen!(
        /// Format and convert a CID in various useful ways.
        #[derive(Debug, Endpoint)]
        Format
    );

    #[derive(Debug, Default)]
    pub struct Builder {
        opt_params: Option<String>,
    }

    impl<'a> Builder {
        /// Required argument: `cid` :CID to format.
        pub fn build(self, client: &'a Client, host: &String, cid: String) -> Format<'a> {
            Format {
                client,
                request: client
                    .post(format!(
                        "{}/api/v0/cid/format?arg={}&{}",
                        host,
                        cid,
                        self.opt_params.unwrap_or("".into())
                    ))
                    .build()
                    .unwrap(),
            }
        }
    }

    impl_opt_param!(
        /// Printf style format string. Default: `%s`.
        f: String,
        /// CID version to convert to.
        v: String,
        /// CID multicodec to convert to.
        mc: String,
        /// Multibase to display CID in.
        b: String
    );

    #[derive(Debug, Deserialize, EndpointResponse)]
    pub struct Response {
        cid_str: String,
        error_msg: Option<String>,
        formatted: Option<String>,
    }
}
pub mod hashes {
    

    use super::*;
    endpoint_gen!(
        /// List available multihashes.
        Hashes
    );

    builder_impl_with_opt_params!(
        Hashes:"/api/v0/cid/hashes",
        /// also include numeric codes.
        numeric:bool,
        /// list only codecs supported by go-ipfs commands.
        supported:bool
    );

    #[derive(Debug, Deserialize, EndpointResponse)]
    #[serde(transparent)]
    pub struct Response {
        multihashes: Vec<Multihash>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Multihash {
        code: Option<isize>,
        name: String,
    }
}
