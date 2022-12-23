use crate::error::Error;
use crate::traits::Endpoint;
use crate::{builder_impl_with_opt_params, endpoint_gen, simple_builder_impl};
use async_trait::async_trait;

endpoint_gen!(
    /// List all available commands
    #[derive(Debug)]
    Commands
);

type Response = String;

#[async_trait]
impl<'a> Endpoint<Response> for Commands<'a> {
    async fn exec(&self) -> Result<Response, Error> {
        let res = match self.client.execute(self.request.try_clone().unwrap()).await {
            Ok(res) => res,
            Err(e) => return Err(Error::Reqwest(e)),
        }
        .text()
        .await;
        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
}

builder_impl_with_opt_params!(
    Commands:"/api/v0/commands",
    /// Show command flags. Required: no.
    flags:bool
);

pub mod completion {
    use super::*;
    pub mod bash {
        use super::*;

        endpoint_gen!(
            /// Generate bash shell completions.
            /// This endpoint takes no argument.
            #[derive(Debug)]
            Bash
        );

        #[async_trait]
        impl<'a> Endpoint<Response> for Bash<'a> {
            async fn exec(&self) -> Result<Response, Error> {
                let res = match self.client.execute(self.request.try_clone().unwrap()).await {
                    Ok(v) => v,
                    Err(e) => return Err(Error::Reqwest(e)),
                }
                .text()
                .await;
                match res{
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::Reqwest(e))
                }
            }
        }

        simple_builder_impl!(Bash:"/api/v0/commands/completion/bash");
    }
}
