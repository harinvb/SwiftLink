use std::error::Error;

use tracing::{Level, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_file(false)
        .with_line_number(false)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}
