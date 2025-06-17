# Rust MCP Implementation Patterns

## Available Crates

### mcpr (v0.2.3+)
- Most mature and complete implementation
- Available on crates.io
- Includes project generator
- Built-in stdio transport support

### Key Features
- Type-safe message handling with serde
- Async support with tokio
- Modular architecture
- Error handling with Result types

## Implementation Patterns

### 1. Server Structure
```rust
use mcpr::{server::Server, Tool, Resource, Prompt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct ConvertArgs {
    input_path: String,
    output_path: String,
    #[serde(default)]
    converter: String,
}
```

### 2. Tool Implementation
```rust
impl Tool for ConvertTool {
    fn name(&self) -> &str {
        "convert_dng_to_webp"
    }
    
    fn description(&self) -> &str {
        "Convert DNG to WebP losslessly"
    }
    
    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input_path": {
                    "type": "string",
                    "description": "Path to DNG file"
                },
                "output_path": {
                    "type": "string", 
                    "description": "Path for WebP output"
                }
            },
            "required": ["input_path", "output_path"]
        })
    }
    
    async fn call(&self, params: Value) -> Result<Value> {
        let args: ConvertArgs = serde_json::from_value(params)?;
        // Implementation
    }
}
```

### 3. Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConversionError {
    #[error("Input file not found: {0}")]
    FileNotFound(String),
    
    #[error("Converter not available: {0}")]
    ConverterNotAvailable(String),
    
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
}
```

### 4. Async Command Execution
```rust
use tokio::process::Command;

async fn run_command(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .await?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Command failed: {}", 
            String::from_utf8_lossy(&output.stderr)))
    }
}
```

## Best Practices

### 1. Type Safety
- Use strongly typed structs for arguments
- Validate inputs with serde
- Return meaningful error types

### 2. Async Operations
- Use tokio for async runtime
- Handle long-running operations properly
- Consider timeouts for external commands

### 3. Logging
- Use tracing crate for structured logging
- Log tool invocations and results
- Include request IDs for correlation

### 4. Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tool_schema() {
        let tool = ConvertTool::new();
        let schema = tool.input_schema();
        assert!(schema["properties"]["input_path"].is_object());
    }
}
```

## Project Structure Best Practices
- Separate transport, handlers, and business logic
- Use traits for extensibility
- Keep tool implementations modular
- Document public APIs thoroughly