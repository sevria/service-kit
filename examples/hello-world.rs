use anyhow::Result;
use service_kit::http::Server;

#[tokio::main]
async fn main() -> Result<()> {
    service_kit::bootstrap()?;

    let server = Server::new();
    server.run().await?;

    Ok(())
}
