use product_service::server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::new().await;
    server.start().await?;
    Ok(())
}
