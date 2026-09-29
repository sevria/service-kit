use anyhow::Result;
use axum::{Json, Router as AxumRouter, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

use crate::config::config;

pub struct Server {
    message: String,
}

impl Server {
    pub fn new() -> Self {
        Self {
            message: "Hello, World!".to_string(),
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub async fn run(&self) -> Result<()> {
        let address = &config().http.address;
        let listener = TcpListener::bind(address).await?;
        let message = self.message.clone();
        let router =
            AxumRouter::new().route("/", get(async move || Json(json!({"message": message}))));

        log::info!("Running HTTP server on {address}");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
