use anyhow::{anyhow, Result};
use libp2p::{tcp, tls, yamux, SwarmBuilder};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::num::NonZeroU8;
use std::time::Duration;
use tauri::{App, AppHandle, Manager};
use tracing::info;
use tracing::log::LevelFilter;

use behaviour::SwiftLink;

use migration::{Migrator, MigratorTrait};

use crate::core::behaviour::SLSwarm;

mod behaviour;
mod gossipsub_behaviour;
mod json_behaviour;
mod mdns_behaviour;

#[derive(Clone)]
pub struct Context {
    pub app_handle: AppHandle,
    pub db: DatabaseConnection,
}

impl Context {
    fn new(app: AppHandle, db: DatabaseConnection) -> Self {
        Self {
            app_handle: app,
            db,
        }
    }
}

pub async fn init_core(app: &mut App) -> Result<()> {
    info!("checking db");
    let mut app_handle = app.handle().clone();
    let db_url = create_or_get_db_url(app_handle).await?;
    info!("db url: {}", db_url);
    let db = init_db(db_url).await?;
    info!("initializing core backend");
    let mut swarm = init_swarm()?;
    info!("swarm profile initialized");
    bind(&mut swarm)?;
    info!("core initialized successfully");
    app_handle = app.handle().clone();
    let context = Context::new(app_handle, db);
    info!("context created successfully");
    behaviour::spawn_behaviour_process(context, swarm);
    info!("process spawned successfully");
    // app.get_window("splashscreen").unwrap().close()?;
    Ok(())
}

pub async fn init_db(db_url: String) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(format!("sqlite://{}?mode=rwc", db_url));
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(60))
        .acquire_timeout(Duration::from_secs(3 * 60))
        .idle_timeout(Duration::from_secs(5 * 60))
        .max_lifetime(Duration::from_secs(60 * 60))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Trace);
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

pub async fn create_or_get_db_url(handle: AppHandle) -> Result<String> {
    let path_resolver = handle.path();
    let mut db_path = path_resolver.app_data_dir()?;
    tokio::fs::create_dir_all(&db_path).await?;
    db_path.push("swiftlink.db");
    let db_url = db_path
        .to_str()
        .ok_or(anyhow!("db path is not valid utf8"))?;
    Ok(db_url.to_string())
}

fn bind(swarm: &mut SLSwarm) -> Result<()> {
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    // swarm.listen_on("/ip6/::/tcp/0".parse()?)?;
    Ok(())
}

fn init_swarm() -> Result<SLSwarm> {
    let swarm = SwarmBuilder::with_new_identity()
        // Runtime Config
        .with_tokio()
        // TCP Config
        .with_tcp(
            tcp::Config::default(),
            tls::Config::new,
            yamux::Config::default,
        )?
        .with_dns()?
        // Behaviour Config
        .with_behaviour(|key| SwiftLink::new(key).unwrap())?
        // Swarm Config
        .with_swarm_config(|mut cfg| {
            // Edit cfg here.
            cfg = cfg.with_idle_connection_timeout(Duration::from_secs(3 * 60));
            cfg = cfg.with_dial_concurrency_factor(NonZeroU8::new(10).unwrap());
            cfg
        })
        .build();
    Ok(swarm)
}
