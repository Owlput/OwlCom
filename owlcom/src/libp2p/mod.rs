use libp2p::identity;
pub mod swarm;

pub struct Libp2pIdentityUnion{
    keypair:identity::Keypair
}

impl Libp2pIdentityUnion{
    pub fn new()->Self{
        Self { keypair: identity::Keypair::generate_ed25519() }
    }
}