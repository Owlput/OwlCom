use async_trait::async_trait;

#[async_trait]
pub trait Endpoint<T, E>
where
    T: EndpointResponse,
    E: std::error::Error,
{
    async fn exec(&self) -> Result<T, E>;
}

#[async_trait]
pub trait EndpointOnce<T,E>
where T:EndpointResponse,
    E:std::error::Error
{
    async fn exec(self)->Result<T,E>;
}

pub trait EndpointResponse {}
