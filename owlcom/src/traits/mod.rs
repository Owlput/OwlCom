use async_trait::async_trait;
use crate::error::Error;

#[async_trait]
pub trait Endpoint<T>
where
    T: EndpointResponse
{
    async fn exec(&self) -> Result<T, Error>;
}

#[async_trait]
pub trait EndpointOnce<T>
where
    T: EndpointResponse,
{
    async fn exec(self) -> Result<T, Error>;
}

pub trait EndpointResponse {}
