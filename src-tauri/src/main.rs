// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod client;
mod log;


use anyhow::Result;



use core::setup_core;
use client::setup_client;
use log::setup_logging;

#[tokio::main]
async fn main() -> Result<()>{
    setup_logging()?;
    setup_core()?;
    setup_client()?;
    Ok(())
}

