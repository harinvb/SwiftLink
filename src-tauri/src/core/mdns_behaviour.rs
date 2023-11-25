use std::collections::HashMap;
use libp2p::mdns::Event;
use libp2p::swarm::dial_opts::DialOpts;
use tracing::{error, info};

use crate::core::{Context, SLSwarm};
use crate::core::json_behaviour::Request;

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event, swarm: &mut SLSwarm) {
    match event {
        Event::Discovered(peers) => {
            let mut peer_map = HashMap::new();
            for (peer_id, multiaddr) in peers {
                //TODO: Initiate info exchange with peer
                info!("discovered peer: {:?} with multiaddr: {:?}", peer_id, multiaddr);
                if !peer_map.contains_key(&peer_id) {
                    peer_map.insert(&peer_id, vec![]);
                }
                peer_map.get(&peer_id).unwrap().push(multiaddr);
                //TODO: Add peer and info to local db
            }

            for (peer_id, multiaddrs) in peer_map {
               if let Err(e) = swarm.dial(DialOpts::peer_id(peer_id.clone()).addresses(multiaddrs).build()) {
                   error!("error dialing peer: {:?}, error: {}", peer_id, e);
               }
                else {
                    info!("dialed peer: {:?}", peer_id);
                }
            }
        }
        Event::Expired(peers) => {
            for (peer_id, multiaddr) in peers {
                //TODO: Remove peers from local db
            }
        }
    }
}
