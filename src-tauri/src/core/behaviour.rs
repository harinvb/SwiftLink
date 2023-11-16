use libp2p::{mdns::tokio::Behaviour as MdnsBehaviour, swarm::NetworkBehaviour, PeerId, identity::Keypair};

#[derive(NetworkBehaviour)]
pub struct CrossShareBehaviour {
    mdns: MdnsBehaviour,
}

impl CrossShareBehaviour {
    pub fn new(key: &Keypair) -> Result<Self,Box<dyn std::error::Error>> {
        let mdns = MdnsBehaviour::new(libp2p::mdns::Config::default(),
        PeerId::from(&key.public()))?;
        Ok(Self {
            mdns
        })
    }
}