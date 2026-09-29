use anyhow::Result;
use axum::{Json, Router as AxumRouter, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

use crate::config::config;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<()> {
        let address = &config().http.address;
        let listener = TcpListener::bind(address).await?;
        let router = AxumRouter::new().route(
            "/",
            get(async || Json(json!({"message": "Hello from service-kit!"}))),
        );

        log::info!("Running HTTP server on {address}");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
