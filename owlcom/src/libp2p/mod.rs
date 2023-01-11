use libp2p::{
    futures::StreamExt,
    swarm::{dummy, SwarmEvent},
    Swarm,
};

pub mod identity;
pub mod swarm;
pub mod transport;
pub mod behaviour;

pub fn setup_peer() {
    tokio::spawn(async move {
        let identity = identity::IdentityUnion::generate();
        let dummy_behaviour = dummy::Behaviour;
        let transport = libp2p::development_transport(identity.get_keypair())
            .await
            .unwrap();
            println!("transport ready");
        let mut swarm =
            Swarm::with_tokio_executor(transport, dummy_behaviour, identity.get_peer_id());
        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();
        loop {
            println!("loop entered");
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("listening on local address {:?}", address)
                }
                SwarmEvent::ConnectionEstablished { peer_id,.. }=>{
                    println!("connection established with peer {}",peer_id)
                }
                _ => {}
            }
        };
    });
}

#[cfg(test)]
mod test {
    use super::setup_peer;

    #[tokio::test]
    async fn peer_test() {
        setup_peer();
        setup_peer();
        println!("all peer dispatched");
        let _ = tokio::signal::ctrl_c().await;
        println!("exited.");
    }
}
