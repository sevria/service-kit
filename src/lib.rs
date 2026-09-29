use anyhow::Result;

mod config;
pub mod http;

pub use config::config;

pub fn bootstrap() -> Result<()> {
    config::init()?;

    Ok(())
}
