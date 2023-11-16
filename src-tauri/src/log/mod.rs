use anyhow::Result;
use tracing::{subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;



pub fn setup_logging() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}