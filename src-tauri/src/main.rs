// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use action::init_backend;
use log::init_logging;

mod action;
mod core;
mod log;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;
    init_backend()?;
    Ok(())
}
