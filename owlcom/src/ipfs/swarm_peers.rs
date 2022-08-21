#[cfg(test)]
mod test{

    use crate::IpfsApi;

    #[tokio::test]
    async fn get_peers(){
        let mut ipfs = IpfsApi::new_http("127.0.0.1:5001");
        ipfs.build();
        if let Some(client) = ipfs.client{
            let uri = format!("{}/api/",ipfs.entry.to_string());
            client.get(uri.parse().unwrap()).await.unwrap();
        } 
    }
}