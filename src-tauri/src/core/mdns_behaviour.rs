use std::collections::HashMap;

use libp2p::mdns::Event;
use tracing::{error, info};

use crate::core::{Context, SLSwarm};

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event, _swarm: &mut SLSwarm) {
    match event {
        Event::Discovered(peers) => {
            let mut _peer_map = HashMap::new();
            for (peer_id, multiaddr) in peers {
                //TODO: Initiate info exchange with peer
                info!("discovered peer: {:?} with multiaddr: {:?}", peer_id, multiaddr);
                if !_peer_map.contains_key(&peer_id) {
                    _peer_map.insert(peer_id.clone(), vec![]);
                }
                match _peer_map.get_mut(&peer_id) {
                    Some(vec) => vec.push(multiaddr),
                    None => error!("error adding multiaddr to peer map"),
                }
                //TODO: Add peer and info to local db
            }
        }
        Event::Expired(peers) => {
            for (_peer_id, _multiaddr) in peers {
                //TODO: Remove peers from local db
            }
        }
    }
}
