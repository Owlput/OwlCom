use libp2p::{core::transport::Boxed,core::muxing::StreamMuxerBox, PeerId};

pub fn open_development_transport(
    identity: &super::identity::IdentityUnion,
) -> Result<Boxed<(PeerId, StreamMuxerBox)>, std::io::Error> {
    libp2p::tokio_development_transport(identity.get_keypair())
}
