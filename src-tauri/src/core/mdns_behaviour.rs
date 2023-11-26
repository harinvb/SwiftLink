use libp2p::mdns::Event;

use crate::core::{Context, SLSwarm};

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event, swarm: &mut SLSwarm) {
    match event {
        Event::Discovered(peers) => {
            for (_, multiaddr) in peers {
                swarm.add_external_address(multiaddr);
                //TODO: Add peer and info to local db
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
