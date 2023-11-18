use anyhow::Result;
use libp2p::{futures::StreamExt, noise, Swarm, SwarmBuilder, yamux};
use sqlx::{migrate::MigrateDatabase, Sqlite};
use tauri::{App, AppHandle};
use tokio::spawn;
use tracing::info;

use behaviour::SwiftLink;

mod behaviour;
mod cbor_behaviour;
mod mdns_behaviour;

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

pub fn init_core(app: &mut App) -> Result<()> {
    let context = Context::new(app);
    info!("initializing core backend");
    let mut swarm = init_swarm()?;
    info!("swarm profile initialized");
    bind(&mut swarm)?;
    info!("core initialized successfully");
    spawn_process(&context, swarm);
    info!("process spawned successfully");
    // app.get_window("splashscreen").unwrap().close()?;
    Ok(())
}

pub async fn init_db(context: Context) -> Result<()> {
    let path_resolver = context.app_handle.path_resolver();
    //TODO: Replace unwrap with proper error handling
    let db_path = path_resolver.app_local_data_dir().unwrap()
        .join("slink.db");
    let db_url = db_path.to_str().expect("failed to convert db path to str");
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("creating database {}", db_url);
        Sqlite::create_database(db_url).await?;
    }
    Ok(())
}

fn spawn_process(context: &Context, mut swarm: Swarm<SwiftLink>) {
    let context = context.clone();
    spawn(async move {
        loop {
            match swarm.next().await {
                Some(event) => {
                    spawn(async move {
                        behaviour::process_event(context.clone(), event)
                    });
                }
                None => {
                    info!("swarm select next some returned none");
                }
            }
        }
    });
}

fn bind(swarm: &mut Swarm<SwiftLink>) -> Result<()> {
    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;
    swarm.listen_on("/ip6/::/tcp/0".parse()?)?;
    Ok(())
}

fn init_swarm() -> Result<Swarm<SwiftLink>> {
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
