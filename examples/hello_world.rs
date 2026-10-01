use service_kit::{Result, Server};

#[service_kit::main]
async fn main() -> Result<()> {
    service_kit::bootstrap()?;

    let server = Server::new();
    server.run().await?;

    Ok(())
}
