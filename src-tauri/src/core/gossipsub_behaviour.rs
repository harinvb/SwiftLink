use libp2p::gossipsub::{Behaviour, Event};
use crate::core::behaviour::SLSwarm;
use crate::core::Context;

pub type Gossipsub = Behaviour;



pub fn process_gossipsub_event(_context: Context, event: Event, _swarm: &mut SLSwarm) {
    match event {
        Event::Message { .. } => {}
        Event::Subscribed { .. } => {}
        Event::Unsubscribed { .. } => {}
        Event::GossipsubNotSupported { .. } => {}
    }
}