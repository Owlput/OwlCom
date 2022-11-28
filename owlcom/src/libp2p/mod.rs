use libp2p::{swarm::{DummyBehaviour, SwarmEvent}, Swarm, futures::StreamExt};

pub mod identity;
pub mod swarm;


pub async fn setup_peer(){
    let identity = identity::IdentityUnion::generate();
    let dummy_behaviour = DummyBehaviour::default();
    let transport = libp2p::development_transport(identity.get_keypair()).await.unwrap();
    let mut swarm = Swarm::new(transport,dummy_behaviour,identity.get_peer_id());
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    loop{
        match swarm.select_next_some().await{
            SwarmEvent::NewListenAddr { address,.. }=>{
                println!("listening on local address {:?}",address)
            }
            _=>{}
        }

    }
}