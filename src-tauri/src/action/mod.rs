use std::error::Error;

use tauri::{async_runtime, Builder, generate_context, generate_handler};
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use tracing::info;

use crate::core::init_core;

mod discover;
mod display;

pub fn init_backend() -> Result<(), Box<dyn Error>> {
    async_runtime::set(Handle::current());
    info!("setting up client backend");
    Builder::default()
        .setup(|app| {
            block_in_place(move || {
                Handle::current().block_on(async move {
                    init_core(app).await.map_err(|e| e.into())
                })
            }
            )
        })
        .invoke_handler(generate_handler![
            discover::get_discovered_clients,
            display::close_splashscreen
        ])
        .run(generate_context!())?;
    info!("client setup successful");
    Ok(())
}
