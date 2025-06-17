pub mod converters;
pub mod handlers;
pub mod server;
pub mod transport;

pub use server::McpImageServer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpImageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Converter not available: {0}")]
    ConverterNotAvailable(String),
    
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("MCP error: {0}")]
    Mcp(String),
}

pub type Result<T> = std::result::Result<T, McpImageError>;