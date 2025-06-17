use mcp_imagemagick::McpImageServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = McpImageServer::new();
    server.run().await?;
    Ok(())
}
