use libp2p::{mdns::Event, swarm::dial_opts::DialOpts};
use std::collections::HashMap;
use tracing::error;

use crate::core::{Context, SLSwarm};

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event, swarm: &mut SLSwarm) {
    match event {
        Event::Discovered(peers) => {
            let mut peer_addr_map = HashMap::new();
            for (peer_id, multiaddr) in peers {
                if !peer_addr_map.contains_key(&peer_id) {
                    peer_addr_map.insert(peer_id, vec![]);
                }
                peer_addr_map.get_mut(&peer_id).unwrap().push(multiaddr);
            }
            for (peer_id, multiaddrs) in peer_addr_map {
                let dial_opts = DialOpts::peer_id(peer_id).addresses(multiaddrs);
                if let Err(e) = swarm.dial(dial_opts.build()) {
                    error!("Failed to dial peer: {}", e);
                }
                //TODO: Add peers to local db
            }
        }
        Event::Expired(peers) => {
            for (_, multiaddr) in peers {
                swarm.remove_external_address(&multiaddr);
                //TODO: Remove peers from local db
            }
        }
    }
}
