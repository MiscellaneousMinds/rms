use product_service::server::Server;
use rust_product_api_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::new().await;
    server.start().await?;
    Ok(())
}
