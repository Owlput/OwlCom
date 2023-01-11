use libp2p::ping;
pub use libp2p::ping::Behaviour;

pub struct PingBehaviour;
impl PingBehaviour{
    pub fn new()->ping::Behaviour{
        ping::Behaviour::new(ping::Config::new())
    }
}