use crate::traits::Endpoint;
use crate::{builder_impl_with_opt_params, endpoint_gen, simple_builder_impl};
use async_trait::async_trait;

endpoint_gen!(
    /// List all available commands
    Commands
);

#[async_trait]
impl<'a> Endpoint<String, reqwest::Error> for Commands<'a> {
    async fn exec(&self) -> Result<String, reqwest::Error> {
        self.client
            .execute(self.request.try_clone().unwrap())
            .await?
            .text()
            .await
    }
}

builder_impl_with_opt_params!(
    Commands:"/api/v0/commands",
    /// Show command flags.
    flags:bool
);

pub mod completion {
    use super::*;
    pub mod bash {
        use super::*;

        endpoint_gen!(
            /// Generate bash shell completions.
            #[derive(Debug)]
            Bash
        );

        #[async_trait]
        impl<'a> Endpoint<String, reqwest::Error> for Bash<'a> {
            async fn exec(&self) -> Result<String, reqwest::Error> {
                self.client
                    .execute(self.request.try_clone().unwrap())
                    .await?
                    .text()
                    .await
            }
        }

        simple_builder_impl!(Bash:"/api/v0/commands/completion/bash");
    }
}
