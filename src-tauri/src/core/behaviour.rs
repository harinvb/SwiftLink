use libp2p::{identity::Keypair, mdns::tokio::Behaviour as Mdns, swarm::NetworkBehaviour, PeerId};

#[derive(NetworkBehaviour)]
pub struct SwiftLink {
    mdns: Mdns,
}

impl SwiftLink {
    pub fn new(key: &Keypair) -> Result<Self, Box<dyn std::error::Error>> {
        let mdns = Mdns::new(libp2p::mdns::Config::default(), PeerId::from(&key.public()))?;
        Ok(Self { mdns })
    }
}
