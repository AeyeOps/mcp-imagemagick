use mcp_imagemagick::McpImageServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set panic handler to log crashes
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("MCP server panic: {}", panic_info);
    }));
    
    let server = McpImageServer::new();
    if let Err(e) = server.run().await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
