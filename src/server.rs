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
        // Initialize logging to stderr only
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into())
            )
            .with_writer(std::io::stderr)
            .init();
        
        tracing::info!("Starting MCP Image Server");
        
        // Main message loop - synchronous
        loop {
            match StdioTransport::read_message() {
                Some(message) => {
                    if message.is_null() {
                        // Empty line, skip
                        tracing::debug!("Received empty line, skipping");
                        continue;
                    }
                    
                    // Log incoming message (debug level)
                    tracing::debug!("Received message: {}", message);
                    
                    // Check if this is already an error response from transport
                    if message.get("error").is_some() && message.get("jsonrpc").is_some() {
                        // Write the error response directly
                        if let Err(e) = StdioTransport::write_message(&message) {
                            tracing::error!("Failed to write error response: {}", e);
                            // Continue processing - don't break the connection
                        }
                        continue;
                    }
                    
                    let response = self.handle_message(message).await;
                    
                    // Log outgoing response (debug level)
                    tracing::debug!("Sending response: {}", response);
                    
                    if let Err(e) = StdioTransport::write_message(&response) {
                        tracing::error!("Failed to write response: {}", e);
                        // Continue processing - don't break the connection
                    }
                }
                None => {
                    // EOF or error, exit gracefully
                    tracing::info!("Server shutting down (EOF or read error)");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_message(&self, message: Value) -> Value {
        // Extract basic JSON-RPC fields
        let id = message.get("id").cloned();
        let method = message.get("method").and_then(|m| m.as_str());
        let params = message.get("params").cloned().unwrap_or(json!({}));
        
        // Validate JSON-RPC 2.0
        if message.get("jsonrpc").and_then(|v| v.as_str()) != Some("2.0") {
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32600,
                    "message": "Invalid Request",
                    "data": "Missing or invalid jsonrpc version"
                }
            });
        }
        
        // ID must be present and not null
        if id.is_none() || id == Some(Value::Null) {
            return json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {
                    "code": -32600,
                    "message": "Invalid Request",
                    "data": "Missing or null id"
                }
            });
        }
        
        let result = match method {
            Some("initialize") => {
                tracing::info!("Handling initialize request");
                self.handle_initialize(params).await
            }
            Some("tools/list") => {
                tracing::info!("Handling tools/list request");
                self.handle_tools_list().await
            }
            Some("tools/call") => {
                tracing::info!("Handling tools/call request");
                self.handle_tool_call(params).await
            }
            Some(m) => {
                tracing::warn!("Unknown method requested: {}", m);
                Err(McpImageError::Mcp(format!(
                    "Method not found: {}",
                    m
                )))
            }
            None => {
                tracing::warn!("Request missing method field");
                Err(McpImageError::Mcp("Missing method".to_string()))
            }
        };
        
        // Build JSON-RPC response
        match result {
            Ok(result) => {
                tracing::debug!("Request succeeded");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": result
                })
            }
            Err(e) => {
                tracing::error!("Request failed: {}", e);
                let (code, message) = match &e {
                    McpImageError::Mcp(msg) if msg.contains("Method not found") => {
                        (-32601, "Method not found")
                    }
                    McpImageError::Mcp(msg) if msg.contains("Invalid params") => {
                        (-32602, "Invalid params")
                    }
                    _ => (-32603, "Internal error")
                };
                
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": code,
                        "message": message,
                        "data": e.to_string()
                    }
                })
            }
        }
    }
    
    async fn handle_initialize(&self, _params: Value) -> Result<Value> {
        // Return capabilities according to spec
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": env!("CARGO_PKG_NAME"),
                "version": env!("CARGO_PKG_VERSION")
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
        let name = params.get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| {
                tracing::error!("tools/call missing 'name' parameter");
                McpImageError::Mcp("Invalid params: missing tool name".to_string())
            })?;
        
        let arguments = params.get("arguments")
            .cloned()
            .unwrap_or_else(|| {
                tracing::debug!("tools/call missing 'arguments', using empty object");
                json!({})
            });
        
        tracing::info!("Calling tool '{}' with arguments: {}", name, arguments);
        
        match self.handler.handle_tool_call(name, arguments).await {
            Ok(result) => {
                tracing::info!("Tool '{}' succeeded", name);
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Tool '{}' failed: {}", name, e);
                Err(e)
            }
        }
    }
}