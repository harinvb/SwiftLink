use std::error::Error;
use tauri::{async_runtime, generate_context, generate_handler, Builder};
use tokio::runtime::Handle;
use tracing::info;

use crate::core::init_core;

mod discover;

pub fn init_backend() -> Result<(), Box<dyn Error>> {
    async_runtime::set(Handle::current());
    info!("setting up client backend");
    Builder::default()
        .setup(init_core)
        .invoke_handler(generate_handler![discover::get_discovered_clients,])
        .run(generate_context!())?;
    info!("client setup successful");
    Ok(())
}
