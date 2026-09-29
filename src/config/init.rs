use anyhow::Result;
use dotenvy::dotenv;
use env_logger::{Builder, Env};
use envconfig::Envconfig;

use super::config::{CONFIG, Config};

pub fn init() -> Result<()> {
    dotenv().ok();

    let log_level = Env::default().default_filter_or("info");

    Builder::from_env(log_level).init();

    let _ = CONFIG.set(Config::init_from_env()?);

    Ok(())
}
