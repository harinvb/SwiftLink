use std::error::Error;

use libp2p::{identity::Keypair, mdns::Config as MdnsConfig, PeerId, StreamProtocol, Swarm, swarm::NetworkBehaviour};
use libp2p::request_response::{Config as ReqRespConfig, ProtocolSupport};
use libp2p::swarm::SwarmEvent;
use tracing::{error, info};

use crate::core::{Context};

use super::json_behaviour::{exchange_info, JsonReqResp, process_json_event};
use super::mdns_behaviour::{Mdns, process_mdns_event};

pub type SLSwarm = Swarm<SwiftLink>;

#[derive(NetworkBehaviour)]
pub struct SwiftLink {
    pub mdns: Mdns,
    // gossipsub: Gossipsub,
    pub json: JsonReqResp,
}

impl SwiftLink {
    pub fn new(key: &Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = PeerId::from(&key.public());
        let mdns_config = MdnsConfig { query_interval: std::time::Duration::from_secs(2), ..Default::default() };
        let mdns = Mdns::new(mdns_config,
                             peer_id)?;
        // let gossipsub = Gossipsub::new(
        //     MessageAuthenticity::Author(peer_id),
        //     GossipsubConfig::default(),
        // )?;
        let json = JsonReqResp::new([(StreamProtocol::new("/slink/1.0"),
                                      ProtocolSupport::Full)], ReqRespConfig::default());
        Ok(Self { mdns, json })
    }
}

pub fn process_event(context: Context, event: SwarmEvent<SwiftLinkEvent>, swarm: &mut SLSwarm) {
    match event {
        SwarmEvent::Behaviour(swift_link_event) => {
            match swift_link_event {
                SwiftLinkEvent::Mdns(event) => {
                    process_mdns_event(context, event, swarm);
                }
                SwiftLinkEvent::Json(event) => {
                    process_json_event(context, event, swarm);
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
            exchange_info(swarm, &peer_id);
            info!("sent a peer info request to {}", peer_id);
        }
        SwarmEvent::ConnectionClosed { peer_id, connection_id, num_established, cause, endpoint } => {
            info!("connection closed: peer_id: {}, connection_id: {}, num_established: {}, cause: {:?}, endpoint: {:?}",peer_id, connection_id, num_established, cause, endpoint);
        }
        SwarmEvent::IncomingConnection { connection_id, local_addr, send_back_addr } => {
            info!("incoming connection: connection_id: {}, local_addr: {}, send_back_addr: {}",connection_id,local_addr,send_back_addr);
        }
        SwarmEvent::IncomingConnectionError { connection_id, error, local_addr, send_back_addr } => {
            error!("incoming connection error: connection_id: {}, local_addr: {}, send_back_addr: {}, error: {}",connection_id,local_addr,send_back_addr,error);
        }
        SwarmEvent::OutgoingConnectionError { peer_id, connection_id, error } => {
            let mut peer = "Unknown".to_string();
            if let Some(p) = peer_id {
                peer = p.to_string();
            }
            error!("outgoing connection error: peer_id: {}, connection_id: {}, error: {}",peer,connection_id,error);
        }
        SwarmEvent::NewListenAddr { address, .. } => {
            info!("new listen address: {}", address);
        }
        SwarmEvent::ExpiredListenAddr { address, .. } => {
            info!("expired listen address: {}", address);
        }
        SwarmEvent::ListenerClosed { addresses, .. } => {
            for address in addresses {
                info!("listener closed: {}", address);
            }
        }
        SwarmEvent::ListenerError { error, .. } => {
            error!("listener error: {}", error);
        }
        SwarmEvent::Dialing { connection_id, peer_id } => {
            info!("dialing: connection_id: {}, peer_id: {}", connection_id, peer_id.unwrap());
        }
        SwarmEvent::NewExternalAddrCandidate { .. } => {}
        SwarmEvent::ExternalAddrConfirmed { .. } => {}
        SwarmEvent::ExternalAddrExpired { .. } => {}
        _ => {
            info!("unhandled swarm event: {:?}", event);
        }
    };
}

