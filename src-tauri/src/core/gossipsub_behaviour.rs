#![allow(dead_code)]
use crate::core::behaviour::SLSwarm;
use crate::core::Context;
use libp2p::gossipsub::{Behaviour, Event};
use tracing::{info, warn};

pub type Gossipsub = Behaviour;

pub fn process_gossipsub_event(_context: Context, event: Event, _swarm: &mut SLSwarm) {
    match event {
        Event::Message {
            message,
            propagation_source,
            ..
        } => {
            info!(
                "Got message: {} from {}",
                String::from_utf8_lossy(&message.data),
                propagation_source
            );
        }
        Event::Subscribed { peer_id, topic } => {
            info!("Peer {} subscribed to topic {}", peer_id, topic.as_str());
        }
        Event::Unsubscribed { peer_id, topic } => {
            info!(
                "Peer {} unsubscribed from topic {}",
                peer_id,
                topic.as_str()
            );
        }
        Event::GossipsubNotSupported { peer_id } => {
            warn!("Peer {} does not support gossipsub", peer_id);
        }
    }
}
