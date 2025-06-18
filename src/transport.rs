use std::io::{self, Write};
use serde_json::Value;

use crate::{Result, McpImageError};

/// Simple synchronous stdio transport for MCP communication
pub struct StdioTransport;

impl StdioTransport {
    pub fn new() -> Self {
        StdioTransport
    }
    
    /// Read a single JSON-RPC message from stdin
    pub fn read_message() -> Option<Value> {
        let stdin = io::stdin();
        let mut line = String::new();
        
        match stdin.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return Some(Value::Null); // Signal empty line
                }
                
                match serde_json::from_str::<Value>(trimmed) {
                    Ok(value) => Some(value),
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        eprintln!("Invalid line: {}", trimmed);
                        // Return parse error as JSON-RPC error
                        Some(serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32700,
                                "message": "Parse error",
                                "data": e.to_string()
                            },
                            "id": null
                        }))
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stdin: {}", e);
                None
            }
        }
    }
    
    /// Write a JSON-RPC message to stdout
    pub fn write_message(message: &Value) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        
        // Convert to JSON string (single line)
        let json = serde_json::to_string(message)
            .map_err(|e| McpImageError::Mcp(format!("Failed to serialize JSON: {}", e)))?;
        
        // Write the JSON followed by a newline
        writeln!(handle, "{}", json)
            .map_err(|e| McpImageError::Mcp(format!("Failed to write to stdout: {}", e)))?;
        
        // Flush to ensure the message is sent immediately
        handle.flush()
            .map_err(|e| McpImageError::Mcp(format!("Failed to flush stdout: {}", e)))?;
        
        Ok(())
    }
}