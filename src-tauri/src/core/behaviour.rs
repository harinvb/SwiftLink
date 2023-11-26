use std::error::Error;
use std::time::Duration;

use libp2p::futures::StreamExt;
use libp2p::{
    gossipsub::{Config as GossipsubConfig, IdentTopic, MessageAuthenticity},
    identity::Keypair,
    mdns::Config as MdnsConfig,
    request_response::{Config as ReqRespConfig, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId, StreamProtocol, Swarm,
};
use tokio::time::interval;
use tokio::{select, spawn};
use tracing::{error, info};

use crate::core::{
    gossipsub_behaviour::{process_gossipsub_event, Gossipsub},
    json_behaviour::{exchange_info, process_json_event, JsonReqResp},
    mdns_behaviour::{process_mdns_event, Mdns},
    Context,
};

pub type SLSwarm = Swarm<SwiftLink>;

#[derive(NetworkBehaviour)]
pub struct SwiftLink {
    pub mdns: Mdns,
    pub gossipsub: Gossipsub,
    pub json: JsonReqResp,
}

impl SwiftLink {
    pub fn new(key: &Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = PeerId::from(&key.public());
        let mdns_config = MdnsConfig {
            query_interval: Duration::from_secs(2),
            ..Default::default()
        };
        let mdns = Mdns::new(mdns_config, peer_id)?;
        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(key.clone()),
            GossipsubConfig::default(),
        )?;
        let json = JsonReqResp::new(
            [(StreamProtocol::new("/slink/1.0"), ProtocolSupport::Full)],
            ReqRespConfig::default(),
        );
        Ok(Self {
            mdns,
            json,
            gossipsub,
        })
    }
}

pub fn process_event(context: Context, event: SwarmEvent<SwiftLinkEvent>, swarm: &mut SLSwarm) {
    match event {
        SwarmEvent::Behaviour(swift_link_event) => match swift_link_event {
            SwiftLinkEvent::Mdns(event) => {
                process_mdns_event(context, event, swarm);
            }
            SwiftLinkEvent::Json(event) => {
                process_json_event(context, event, swarm);
            }
            SwiftLinkEvent::Gossipsub(event) => {
                process_gossipsub_event(context, event, swarm);
            }
        },
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
        SwarmEvent::ConnectionClosed {
            peer_id,
            connection_id,
            num_established,
            cause,
            endpoint,
        } => {
            info!("connection closed: peer_id: {}, connection_id: {}, num_established: {}, cause: {:?}, endpoint: {:?}",peer_id, connection_id, num_established, cause, endpoint);
        }
        SwarmEvent::IncomingConnection {
            connection_id,
            local_addr,
            send_back_addr,
        } => {
            info!(
                "incoming connection: connection_id: {}, local_addr: {}, send_back_addr: {}",
                connection_id, local_addr, send_back_addr
            );
        }
        SwarmEvent::IncomingConnectionError {
            connection_id,
            error,
            local_addr,
            send_back_addr,
        } => {
            error!("incoming connection error: connection_id: {}, local_addr: {}, send_back_addr: {}, error: {}",connection_id,local_addr,send_back_addr,error);
        }
        SwarmEvent::OutgoingConnectionError {
            peer_id,
            connection_id,
            error,
        } => {
            let mut peer = "Unknown".to_string();
            if let Some(p) = peer_id {
                peer = p.to_string();
            }
            error!(
                "outgoing connection error: peer_id: {}, connection_id: {}, error: {}",
                peer, connection_id, error
            );
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
        SwarmEvent::Dialing {
            connection_id,
            peer_id,
        } => {
            info!(
                "dialing: connection_id: {}, peer_id: {}",
                connection_id,
                peer_id.unwrap()
            );
        }
        SwarmEvent::NewExternalAddrCandidate { .. } => {}
        SwarmEvent::ExternalAddrConfirmed { .. } => {}
        SwarmEvent::ExternalAddrExpired { .. } => {}
        _ => {
            info!("unhandled swarm event: {:?}", event);
        }
    };
}

pub fn spawn_behaviour_process(context: Context, mut swarm: SLSwarm) {
    let core_topic = IdentTopic::new("slink");
    swarm
        .behaviour_mut()
        .gossipsub
        .subscribe(&core_topic)
        .expect("failed to subscribe to root gossipsub topic");
    let mut interval = interval(Duration::from_secs(5));
    spawn(async move {
        loop {
            select! {
                event = swarm.select_next_some() => {
                        process_event(context.clone(), event, &mut swarm)
                    },
                _ = interval.tick() => {
                    let connected_peer_cnt = swarm.connected_peers().count();
                    if connected_peer_cnt >0 {
                        let core_topic = IdentTopic::new("slink");
                        swarm.behaviour_mut().gossipsub.publish(core_topic, "hello world").expect("failed to publish to root gossipsub topic");
                    }
                }
            }
        }
    });
}
