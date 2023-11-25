use libp2p::mdns::Event;
use libp2p::swarm::dial_opts::DialOpts;
use tracing::info;

use crate::core::json_behaviour::Request;
use crate::core::{Context, SLSwarm};

pub type Mdns = libp2p::mdns::tokio::Behaviour;

pub fn process_mdns_event(_context: Context, event: Event, swarm: &mut SLSwarm) {
    match event {
        Event::Discovered(peers) => {
            for (peer_id, multiaddr) in peers {
                //TODO: Initiate info exchange with peer
                let behaviour = swarm.behaviour_mut();
                behaviour.json.add_address(&peer_id, multiaddr);
                behaviour.json.send_request(
                    &peer_id,
                    Request::ExchangeInfo {
                        username: "test".to_string(),
                        device_name: "test".to_string(),
                    },
                );
                info!("sent a peer info request to {}", peer_id);
                //TODO: Add peer and info to local db
            }
        }
        Event::Expired(peers) => {
            for (peer_id, multiaddr) in peers {
                //TODO: Remove peers from local db
            }
        }
    }
}
