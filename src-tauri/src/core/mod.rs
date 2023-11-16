
use libp2p::{SwarmBuilder, yamux, noise, swarm::{SwarmEvent}, Multiaddr,futures::StreamExt};
use behaviour::CrossShareBehaviour;
use tokio::{select, spawn};
use tracing::{info};
use anyhow::Result;
use behaviour::CrossShareBehaviourEvent;

mod behaviour;

pub fn setup_core() -> Result<()> {
    info!("setting up core backend");
    let mut swarm = SwarmBuilder::with_new_identity()
     .with_tokio()
     .with_tcp(
         Default::default(),
         noise::Config::new,
         yamux::Config::default,
     )?
     .with_behaviour(|key|{
         CrossShareBehaviour::new(&key).unwrap()
     })?
     .with_swarm_config(|cfg| {
         // Edit cfg here.
         cfg
     })
     .build();
    info!("swarm profile initialized");
    let ip4: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse()?;
    let ip6: Multiaddr = "/ip6/::/tcp/0".parse()?;
    swarm.listen_on(ip4)?;
    swarm.listen_on(ip6)?;
    info!("core setup successful");
    spawn( async move {
        loop {
                select! {
                    event = swarm.select_next_some() => process_event(event),
                }
            }
        }
    );
    Ok(())
}


fn process_event(event: SwarmEvent<CrossShareBehaviourEvent>) {
    match event {
        e => info!("{:?}",e),
    };
}