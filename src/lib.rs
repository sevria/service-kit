mod config;
pub mod http;

pub use anyhow::Result;
pub use config::config;
pub use service_kit_macros::main;
pub use tokio;

pub fn bootstrap() -> Result<()> {
    config::init()?;

    Ok(())
}
