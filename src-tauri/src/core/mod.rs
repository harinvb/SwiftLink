use std::num::NonZeroU8;
use std::time::Duration;
use anyhow::{anyhow, Result};
use libp2p::{futures::StreamExt, SwarmBuilder, tls, yamux};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tauri::{App, AppHandle, Manager};
use tokio::spawn;
use tracing::info;

use behaviour::SwiftLink;

use crate::core::behaviour::SLSwarm;

mod behaviour;
mod json_behaviour;
mod mdns_behaviour;


#[derive(Clone)]
pub struct Context {
    pub app_handle: AppHandle,
    pub db: SqlitePool,
}

impl Context {
    async fn new(app: AppHandle, db: Pool<Sqlite>) -> Result<Self> {
        Ok(Self {
            app_handle: app,
            db,
        })
    }
}

pub async fn init_core(app: &mut App) -> Result<()> {
    info!("checking db");
    let mut app_handle = app.handle().clone();
    let db_url = create_or_get_db(app_handle).await?;
    info!("db url: {}", db_url);
    let sqlite = SqlitePool::connect(&db_url).await?;
    info!("initializing core backend");
    let mut swarm = init_swarm()?;
    info!("swarm profile initialized");
    bind(&mut swarm)?;
    info!("core initialized successfully");
    app_handle = app.handle().clone();
    let context = Context::new(app_handle, sqlite).await?;
    info!("context created successfully");
    spawn_process(context, swarm);
    info!("process spawned successfully");
    // app.get_window("splashscreen").unwrap().close()?;
    Ok(())
}

pub async fn create_or_get_db(handle: AppHandle) -> Result<String> {
    let path_resolver = handle.path();
    let db_path = path_resolver.app_local_data_dir()?
        .join("slink.db");
    let db_url = db_path.to_str().ok_or(anyhow!("db path is not valid utf8"))?;
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("creating database {}", db_url);
        Sqlite::create_database(db_url).await?;
    }
    Ok(db_url.to_string())
}

fn spawn_process(context: Context, mut swarm: SLSwarm) {
    spawn(async move {
        loop {
            let context = context.clone();
            match swarm.next().await {
                Some(event) => {
                    behaviour::process_event(context.clone(), event, &mut swarm)
                }
                None => {
                    info!("swarm returned none");
                }
            }
        }
    });
}

fn bind(swarm: &mut SLSwarm) -> Result<()> {
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip6/::/tcp/0".parse()?)?;
    Ok(())
}

fn init_swarm() -> Result<SLSwarm> {
    let swarm = SwarmBuilder::with_new_identity()
        // Runtime Config
        .with_tokio()
        // TCP Config
        .with_tcp(
            Default::default(),
            tls::Config::new,
            yamux::Config::default,
        )?
        .with_dns()?
        // Behaviour Config
        .with_behaviour(|key| SwiftLink::new(key).unwrap())?
        // Swarm Config
        .with_swarm_config(|cfg| {
            // Edit cfg here.
            cfg.with_idle_connection_timeout(Duration::from_secs( 3 * 60));
            cfg
        })
        .build();
    Ok(swarm)
}
