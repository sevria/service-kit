use std::sync::OnceLock;

use envconfig::Envconfig;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get().expect("config::init() must be called first")
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested)]
    pub http: Http,
}

#[derive(Envconfig)]
pub struct Http {
    #[envconfig(from = "HTTP_ADDRESS", default = "0.0.0.0:3000")]
    pub address: String,
}
