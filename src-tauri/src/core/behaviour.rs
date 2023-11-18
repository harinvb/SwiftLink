use std::error::Error;

use libp2p::{identity::Keypair, mdns::Config as MdnsConfig, PeerId, StreamProtocol, Swarm, swarm::NetworkBehaviour};
use libp2p::request_response::{Config as ReqRespConfig, ProtocolSupport};
use libp2p::swarm::SwarmEvent;
use tracing::info;

use crate::core::{Context, SLSwarm};

use super::cbor_behaviour::{CborReqResp, process_cbor_event};
use super::mdns_behaviour::{Mdns, process_mdns_event};

#[derive(NetworkBehaviour)]
pub struct SwiftLink {
    pub mdns: Mdns,
    // gossipsub: Gossipsub,
    pub cbor: CborReqResp,
}

impl SwiftLink {
    pub fn new(key: &Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = PeerId::from(&key.public());
        let mdns = Mdns::new(MdnsConfig::default(),
                             peer_id)?;
        // let gossipsub = Gossipsub::new(
        //     MessageAuthenticity::Author(peer_id),
        //     GossipsubConfig::default(),
        // )?;
        let cbor = CborReqResp::new([(StreamProtocol::new("/slink/1.0"),
                                      ProtocolSupport::Full)], ReqRespConfig::default());
        Ok(Self { mdns, cbor })
    }
}

pub fn process_event(context: Context, event: SwarmEvent<SwiftLinkEvent>, swarm: &mut SLSwarm) {
    match event {
        SwarmEvent::Behaviour(swift_link_event) => {
            match swift_link_event {
                SwiftLinkEvent::Mdns(event) => {
                    process_mdns_event(context, event,swarm);
                }
                SwiftLinkEvent::Cbor(event) => {
                    process_cbor_event(context, event,swarm);
                }
            }
        }
        SwarmEvent::ConnectionEstablished {
            peer_id,
            num_established,
            connection_id,
            established_in,
            ..
        } => {
            info!("connection established: peer_id: {}, num_established: {}, connection_id: {}, established_in: {:?}",
                peer_id, num_established,
                connection_id, established_in);
        }
        SwarmEvent::ConnectionClosed { .. } => {}
        SwarmEvent::IncomingConnection { .. } => {}
        SwarmEvent::IncomingConnectionError { .. } => {}
        SwarmEvent::OutgoingConnectionError { .. } => {}
        SwarmEvent::NewListenAddr { .. } => {}
        SwarmEvent::ExpiredListenAddr { .. } => {}
        SwarmEvent::ListenerClosed { .. } => {}
        SwarmEvent::ListenerError { .. } => {}
        SwarmEvent::Dialing { .. } => {}
        SwarmEvent::NewExternalAddrCandidate { .. } => {}
        SwarmEvent::ExternalAddrConfirmed { .. } => {}
        SwarmEvent::ExternalAddrExpired { .. } => {}
        _ => {
            info!("unhandled swarm event: {:?}", event);
        }
    };
}
