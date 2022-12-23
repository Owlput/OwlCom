#[allow(unused)]
use crate::traits::{Endpoint, EndpointResponse};
use crate::{endpoint_gen, error::Error, impl_opt_param, simple_builder_impl};
use async_trait::async_trait;
endpoint_gen!(
    /// Get and set IPFS config values. This endpoint returns a JSON object in `serde_json::Value`.
    #[derive(Debug)]
    Config
);
type Response = serde_json::Value;
#[async_trait]
impl<'a> Endpoint<Response> for Config<'a> {
    async fn exec(&self) -> Result<Response, Error> {
        let response = match self.client.execute(self.request.try_clone().unwrap()).await {
            Ok(v) => v,
            Err(e) => return Err(Error::Reqwest(e)),
        };
        let text = match response.text().await {
            Ok(v) => v,
            Err(e) => return Err(Error::Reqwest(e)),
        };
        match serde_json::from_str(&text) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::SerdeJson(e)),
        }
    }
}

#[derive(Default)]
pub struct Builder {
    opt_params: Option<String>,
}

impl<'a> Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Required argument: `key` Target config key entry.
    /// If no other params is supplied, this endpoint will simply returns the current value of the corresponding config key.
    pub fn build(self, client: &'a Client, host: &String, key: &String) -> Config<'a> {
        Config {
            client,
            request: client
                .post(format!(
                    "{}/api/v0/config?arg={}&{}",
                    host,
                    key,
                    self.opt_params.unwrap_or("".into())
                ))
                .build()
                .unwrap(),
        }
    }
    /// Set a boolean value. Required: no.
    pub fn bool(self, arg: bool) -> Self {
        match self.opt_params {
            None => Self {
                opt_params: Some(format!("{}={}", "bool", arg.to_string())),
            },
            Some(v) => Self {
                opt_params: Some(format!("{}&{}={}", v, "bool", arg.to_string())),
            },
        }
    }
}

impl_opt_param!(
    /// The value to set the config entry to. Required: no.
    arg: String,
    /// Parse stringified JSON. Required: no.
    json: bool
);

pub mod edit {
    use super::*;

    endpoint_gen!(
        /// Open the config file for editing in $EDITOR.
        #[derive(Debug)]
        Edit
    );
    #[async_trait]
    impl<'a> Endpoint<Response> for Edit<'a> {
        async fn exec(&self) -> Result<Response, Error> {
            let response = match self.client.execute(self.request.try_clone().unwrap()).await {
                Ok(v) => v,
                Err(e) => return Err(Error::Reqwest(e)),
            };
            let text = match response.text().await {
                Ok(v) => v,
                Err(e) => return Err(Error::Reqwest(e)),
            };
            match serde_json::from_str(&text) {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::SerdeJson(e)),
            }
        }
    }

    simple_builder_impl!(Edit:"/api/v0/config/edit");
}
