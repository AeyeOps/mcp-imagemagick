# MCP (Model Context Protocol) Implementation Reference

## Protocol Overview
- Open standard for connecting AI assistants to data systems
- Client-server architecture using JSON-RPC 2.0
- Supports stdio and SSE transports
- Released by Anthropic in November 2024

## Core Components

### 1. Transport Layer
- **stdio**: Standard input/output for local processes
- Each line is a complete JSON-RPC 2.0 message
- Newline-delimited JSON format

### 2. Message Format
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "tool_name",
    "arguments": {}
  },
  "id": 1
}
```

### 3. Server Capabilities
- **Tools**: Functions for AI to execute
- **Resources**: Data sources (GET-like, no side effects)
- **Prompts**: Pre-defined templates

## Server Implementation Requirements

### Initialization
1. Capability negotiation via handshake
2. Exchange protocol versions
3. Register available tools/resources/prompts

### Tool Definition Schema
```json
{
  "name": "string",
  "description": "string",
  "inputSchema": {
    "type": "object",
    "properties": {},
    "required": []
  }
}
```

### Security Principles
- Obtain explicit user consent
- Protect user data with access controls
- Treat tool descriptions as untrusted
- Limit prompt visibility

## Rust Implementation with mcpr

### Basic Server Setup
```rust
use mcpr::{
    server::{Server, ServerConfig},
    transport::stdio::StdioTransport,
};

let config = ServerConfig {
    name: "server-name",
    version: "1.0.0",
    tools: vec![/* tools */],
    resources: vec![/* resources */],
    prompts: vec![/* prompts */],
};

let server = Server::new(config);
let transport = StdioTransport::new();
server.start(transport)?;
```

### Tool Handler Pattern
```rust
async fn handle_tool_call(name: &str, args: Value) -> Result<Value> {
    match name {
        "tool_name" => {
            // Implementation
        }
        _ => Err("Unknown tool")
    }
}
```

## Testing
- Use MCP Inspector for protocol validation
- Test with Claude Desktop or other MCP clients
- Verify JSON-RPC message format