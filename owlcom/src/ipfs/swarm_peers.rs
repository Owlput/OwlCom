
pub enum Params{
    Verbose(bool),
    Streams(bool),
    Latency(bool),
    Direction(bool),
}
pub struct Stream{
    protocol:String,
}
pub struct Peer{
    addr:String,
    peer:String,
    latency:String,
    muxer:String,
    direction:String,
    streams:Vec<Stream>
}
impl Peer{

}
pub struct SwarmPeers{
    
}

// impl IpfsApi{
//     pub async fn get_peers(&self)->Vec<Peer>{

//     }
// }

use crate::traits::url::ToParam;

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