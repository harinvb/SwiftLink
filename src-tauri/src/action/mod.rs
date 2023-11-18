use std::error::Error;

use tauri::{async_runtime, Builder, generate_context, generate_handler};
use tokio::runtime::Handle;
use tracing::{error, info};

use crate::core::init_core;

mod discover;
mod display;

pub fn init_backend() -> Result<(), Box<dyn Error>> {
    async_runtime::set(Handle::current());
    info!("setting up client backend");
    Builder::default()
        .setup(|app| {
            match init_core(app) {
                Ok(_) => info!("core init successful"),
                Err(e) => {
                    error!("core init failed: {}", e);
                    return Err(e.into());
                }
            }
            Ok(())
        })
        .invoke_handler(generate_handler![
            discover::get_discovered_clients,
            display::close_splashscreen
        ])
        .run(generate_context!())?;
    info!("client setup successful");
    Ok(())
}
