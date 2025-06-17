use serde_json::{json, Value};

use crate::{Result, McpImageError};
use crate::handlers::ImageHandler;
use crate::transport::StdioTransport;

pub struct McpImageServer {
    handler: ImageHandler,
}

impl McpImageServer {
    pub fn new() -> Self {
        Self {
            handler: ImageHandler::new(),
        }
    }
    
    pub async fn run(self) -> Result<()> {
        // Initialize logging
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into())
            )
            .with_writer(std::io::stderr)
            .init();
        
        tracing::info!("Starting MCP Image Server");
        
        // Set up transport
        let (mut transport, stdout_rx, stdin_tx) = StdioTransport::new();
        StdioTransport::start_stdin_reader(stdin_tx);
        StdioTransport::start_stdout_writer(stdout_rx);
        
        // Main message loop
        while let Some(message) = transport.recv().await {
            let response = self.handle_message(message).await;
            transport.send(response).await?;
        }
        
        Ok(())
    }
    
    async fn handle_message(&self, message: Value) -> Value {
        // Extract basic JSON-RPC fields
        let id = message.get("id").cloned();
        let method = message.get("method").and_then(|m| m.as_str());
        let params = message.get("params").cloned().unwrap_or(json!({}));
        
        let result = match method {
            Some("initialize") => self.handle_initialize(params).await,
            Some("tools/list") => self.handle_tools_list().await,
            Some("tools/call") => self.handle_tool_call(params).await,
            _ => Err(McpImageError::Mcp(format!(
                "Unknown method: {:?}",
                method
            ))),
        };
        
        // Build JSON-RPC response
        match result {
            Ok(result) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": result
                })
            }
            Err(e) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32603,
                        "message": e.to_string()
                    }
                })
            }
        }
    }
    
    async fn handle_initialize(&self, _params: Value) -> Result<Value> {
        Ok(json!({
            "protocolVersion": "2024-11-01",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "mcp-imagemagick",
                "version": "0.1.0"
            }
        }))
    }
    
    async fn handle_tools_list(&self) -> Result<Value> {
        Ok(json!({
            "tools": [
                ImageHandler::get_convert_tool_schema(),
                ImageHandler::get_check_tool_schema()
            ]
        }))
    }
    
    async fn handle_tool_call(&self, params: Value) -> Result<Value> {
        let tool_name = params.get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| McpImageError::InvalidInput("Missing tool name".to_string()))?;
        
        let args = params.get("arguments").cloned().unwrap_or(json!({}));
        
        match tool_name {
            "convert_dng_to_webp" => {
                let convert_args = serde_json::from_value(args)?;
                self.handler.convert_dng_to_webp(convert_args).await
            }
            "check_converters" => {
                let check_args = serde_json::from_value(args)?;
                self.handler.check_converters(check_args).await
            }
            _ => Err(McpImageError::InvalidInput(format!(
                "Unknown tool: {}",
                tool_name
            )))
        }
    }
}