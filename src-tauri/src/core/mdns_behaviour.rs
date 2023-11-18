use libp2p::mdns::Event;
// use tracing::info;

use crate::core::Context;

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event) {
    match event {
        Event::Discovered(peers) => {
            for (peer_id, multiaddr) in peers {
                //TODO: Add peers to local db
            }
        }
        Event::Expired(peers) => {
            for (peer_id, multiaddr) in peers {
                //TODO: Remove peers from local db
            }
        }
    }
}