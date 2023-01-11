use std::fmt::Debug;
use std::marker::PhantomData;

use libp2p::futures::StreamExt;
use libp2p::swarm::{DialError, NetworkBehaviour, SwarmEvent};
pub use libp2p::Multiaddr;
use libp2p::TransportError;
use tokio::select;
use tokio::sync::*;

use super::behaviour::PingBehaviour;
use super::identity::IdentityUnion;
use super::transport::open_development_transport;

#[derive(Debug)]
pub enum Libp2pSwarmOp {
    Dial {
        addr: Multiaddr,
        callback_tx: oneshot::Sender<Libp2pSwarmOpResult>,
    },
    Listen{
        addr:Multiaddr,
        callback_tx:oneshot::Sender<Libp2pSwarmOpResult>
    },
}

#[derive(Debug)]
pub enum Libp2pSwarmOpResult {
    DialOk,
    DialErr(DialError),
    ListenOk,
    ListenErr(TransportError<std::io::Error>),
}

/// Mailbox for the actual task that manages the swarm.
#[derive(Debug,Clone)]
pub struct Libp2pSwarmManager {
    sender: mpsc::Sender<Libp2pSwarmOp>,

}
impl Libp2pSwarmManager
{
    pub async fn execute(&self, op: Libp2pSwarmOp) {
        self.sender.send(op).await.unwrap()
    }
}

pub struct Libp2pSwarmBuilder<B> {
    behaviour: PhantomData<B>,
}

impl<B> Libp2pSwarmBuilder<B>
where
    B: NetworkBehaviour + Send,
    <B as NetworkBehaviour>::OutEvent: Debug,
{
    pub fn build(
        transport: libp2p::core::transport::Boxed<(
            libp2p::PeerId,
            libp2p::core::muxing::StreamMuxerBox,
        )>,
        behaviour: B,
        identity: IdentityUnion,
        buffer_size: usize,
    ) -> (Libp2pSwarmManager,mpsc::Receiver<<B as NetworkBehaviour>::OutEvent>) {
        let (op_tx, mut op_rx) = mpsc::channel(buffer_size);
        let (event_tx, event_rx) = mpsc::channel(8);
        tokio::spawn(async move {
            let mut swarm =
                libp2p::Swarm::with_tokio_executor(transport, behaviour, identity.get_peer_id());
            loop {
                select! {
                Some(manager_op) = op_rx.recv() =>{
                    match manager_op{
                        Libp2pSwarmOp::Listen{addr,callback_tx}=>{
                            let send_result = match swarm.listen_on(addr){
                                Ok(_)=>{callback_tx.send(Libp2pSwarmOpResult::ListenOk)},
                                Err(e)=>{callback_tx.send(Libp2pSwarmOpResult::ListenErr(e))}
                            };
                            match send_result{
                                Ok(_)=>{},
                                Err(e)=>println!("Cannot send out listen error {:?}",e)
                            }
                        }
                        Libp2pSwarmOp::Dial{addr,callback_tx}=>{
                            if let Err(e) = swarm.dial(addr){
                                println!("{}",e);
                                match callback_tx.send(Libp2pSwarmOpResult::DialErr(e)){
                                    Ok(())=>{},
                                    Err(e)=>println!("Cannot send out dial error {:?}",e)
                                };
                            };
                        }
                    }
                }
                swarm_event = swarm.select_next_some() =>{
                    match swarm_event{
                        SwarmEvent::NewListenAddr{address, ..}=>println!("Listening on {:?}",address),
                        SwarmEvent::Behaviour(event)=>{let _ = event_tx.send(event).await;},
                        SwarmEvent::ConnectionEstablished { .. } => println!("connection established"),
                        SwarmEvent::ConnectionClosed { .. } => println!("connection closed"),
                        SwarmEvent::IncomingConnection { send_back_addr,.. } => println!("incoming connection from {} on node {}",send_back_addr,identity.get_peer_id()),
                        SwarmEvent::IncomingConnectionError { send_back_addr,.. } => println!("incoming connection error for {} on node {}", send_back_addr,identity.get_peer_id()),
                        SwarmEvent::OutgoingConnectionError { .. } => println!("outgoing connection error"),
                        SwarmEvent::BannedPeer { .. } => println!("banned peer"),
                        SwarmEvent::ExpiredListenAddr { .. } => println!("expired listen address"),
                        SwarmEvent::ListenerClosed { ..} => println!("listener closed"),
                        SwarmEvent::ListenerError { ..} => println!("listener error"),
                        SwarmEvent::Dialing(_) => println!("dailing"),
                            }
                        }
                    }
            }
        });
        (Libp2pSwarmManager {sender: op_tx,},event_rx)
    }
}

pub fn setup_ping_peer() -> (Libp2pSwarmManager,mpsc::Receiver<libp2p::ping::Event>) {
    let ident_a = IdentityUnion::generate();
    Libp2pSwarmBuilder::build(
        open_development_transport(&ident_a).unwrap(),
        PingBehaviour::new(),
        ident_a.clone(),
        8,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    impl PartialEq for Libp2pSwarmOpResult {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::DialErr(_), Self::DialErr(_)) => true,
                (Self::ListenErr(_), Self::ListenErr(_)) => true,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }

    #[tokio::test]
    async fn test_two_pinging_peers() {
        let (tx_a, rx_a) = oneshot::channel();
        let (tx_b, _rx_b) = oneshot::channel();

        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/10086".parse().unwrap();
        let (man_a,_ev_a) = setup_ping_peer();
        let (man_b,_ev_b) = setup_ping_peer();
        man_a.execute(Libp2pSwarmOp::Listen{
            addr:addr.clone(),
            callback_tx:tx_a
        }).await;
        man_b
            .execute(Libp2pSwarmOp::Dial {
                addr: addr.clone(),
                callback_tx: tx_b,
            })
            .await;
        assert_eq!(rx_a.await.unwrap(), Libp2pSwarmOpResult::DialOk);
        let _ = tokio::signal::ctrl_c().await;
    }
}
