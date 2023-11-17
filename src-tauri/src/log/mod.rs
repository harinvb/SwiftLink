use std::error::Error;
use tracing::{subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}
