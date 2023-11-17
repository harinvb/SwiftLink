use behaviour::{SwiftLink, SwiftLinkEvent};

use libp2p::{futures::StreamExt, noise, swarm::SwarmEvent, yamux, Swarm, SwarmBuilder};
use std::{error::Error, time::Duration};

use tauri::{App, AppHandle, Manager};
use tokio::{select, spawn, time::interval};
use tracing::{error, info};
mod behaviour;

#[derive(Debug, Clone)]
struct Context {
    pub app_handle: AppHandle,
}

impl Context {
    fn new(app: &mut App) -> Self {
        Self {
            app_handle: app.handle(),
        }
    }
}

pub fn init_core(app: &mut App) -> Result<(), Box<dyn Error>> {
    let context = Context::new(app);
    info!("initializing core backend");
    let mut swarm = init_swarm()?;
    info!("swarm profile initialized");
    bind(&mut swarm)?;
    info!("core initialized successfully");
    let mut interval = interval(Duration::from_secs(1));
    spawn(async move {
        loop {
            select! {
                event = swarm.select_next_some() => process_event(context.clone(),event),
                _ = interval.tick() => {
                    let context = context.clone();
                    if let Err(err) = context.app_handle.emit_all("test", "Test Event") {
                        error!("Unable to emit event {:?}", err);
                    }
                }
            }
        }
    });
    Ok(())
}

fn bind(swarm: &mut Swarm<SwiftLink>) -> Result<(), Box<dyn Error>> {
    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;
    swarm.listen_on("/ip6/::/tcp/0".parse()?)?;
    Ok(())
}

fn init_swarm() -> Result<Swarm<SwiftLink>, Box<dyn Error>> {
    let swarm = SwarmBuilder::with_new_identity()
        // Runtime Config
        .with_tokio()
        // TCP Config
        .with_tcp(
            Default::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        // Behaviour Config
        .with_behaviour(|key| SwiftLink::new(&key).unwrap())?
        // Swarm Config
        .with_swarm_config(|cfg| {
            // Edit cfg here.
            cfg
        })
        .build();
    Ok(swarm)
}

fn process_event(context: Context, event: SwarmEvent<SwiftLinkEvent>) {
    match event {
        e => {
            info!("{:?}", e);
            if let Err(err) = context.app_handle.emit_all("test", "Test Event") {
                error!("Unable to emit event {:?}", err);
            }
        }
    };
}
