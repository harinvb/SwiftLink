use anyhow::Result;
use tauri::{async_runtime, Builder, generate_handler, generate_context};
use tokio::runtime::Handle;
use tracing::info;

mod discover;

pub fn setup_client() -> Result<()> {
    async_runtime::set(Handle::current());
    info!("setting up client backend");
    Builder::default()
        .invoke_handler(generate_handler![discover::get_discovered_clients])
        .run(generate_context!())?;
    info!("client setup successful");
    Ok(())
}