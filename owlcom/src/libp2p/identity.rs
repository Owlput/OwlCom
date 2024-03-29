use std::fs;

use libp2p::{identity::{self, Keypair}, PeerId};

pub struct IdentityUnion {
    keypair: identity::Keypair,
    peer_id: PeerId,
}

impl IdentityUnion {
    pub fn generate() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        Self { keypair, peer_id }
    }
    pub fn from_keypair(keypair: &identity::Keypair) -> Self {
        let peer_id = PeerId::from(keypair.public());
        Self {
            keypair: keypair.clone(),
            peer_id,
        }
    }
    pub fn get_pubkey(&self) -> identity::PublicKey {
        self.keypair.public()
    }
    pub fn get_keypair(&self) -> identity::Keypair {
        self.keypair.clone()
    }
    pub fn get_peer_id(&self) -> PeerId {
        self.peer_id.clone()
    }
    pub fn from_rsa_pkcs8_private(path:&Path)->Self{
        let key = fs::read(path).unwrap();
        let keypair = Keypair::rsa_from_pkcs8(&mut key).unwrap();
        let peer_id = PeerId::from(keypair.public());
        IdentityUnion { keypair, peer_id }
    }
}
