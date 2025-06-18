# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an MCP (Model Context Protocol) server for image conversion, primarily focused on converting DNG (Digital Negative) files to WebP format using ImageMagick and darktable. The server implements the MCP protocol with stdio transport and provides tools for image conversion.

## Common Development Commands

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing
```bash
# Run unit tests
cargo test

# Run integration tests
python3 test_mcp.py

# Test MCP protocol manually
./test_protocol.py

# Quick server test
./test_server.sh
```

### Running and Debugging
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run release binary
./target/release/mcp-imagemagick

# Test with MCP Inspector (if installed)
mcp-inspector ./target/release/mcp-imagemagick
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues without building
cargo check
```

## High-Level Architecture

The codebase follows a modular architecture with clear separation of concerns:

### Core Components

1. **MCP Server Layer** (`src/server.rs`)
   - Handles JSON-RPC protocol implementation
   - Routes requests to appropriate handlers
   - Manages the message loop and response formatting
   - Implements MCP methods: `initialize`, `tools/list`, `tools/call`

2. **Transport Layer** (`src/transport.rs`)
   - Manages stdio communication (stdin/stdout)
   - Handles async message passing between threads
   - Buffers and parses JSON-RPC messages

3. **Converter System** (`src/converters/`)
   - **Trait-based Design**: All converters implement the `ImageConverter` trait
   - **Auto-selection**: `AutoConverter` automatically selects the best available converter based on priority
   - **ImageMagick Converter**: Priority 60, uses `convert7` or `magick` command
   - **Darktable Converter**: Priority 40, uses `darktable-cli` for better RAW processing
   - Converters are checked for availability using the `which` crate

4. **Request Handlers** (`src/handlers/`)
   - `ImageHandler` manages tool schemas and delegates to converters
   - Validates input parameters
   - Provides tool definitions for MCP clients

### Key Design Patterns

1. **Async-First**: Uses Tokio for async runtime, all converters are async
2. **Error Handling**: Custom error types in `lib.rs` with `thiserror` for ergonomic error handling
3. **Trait Objects**: Dynamic dispatch for converter selection
4. **Message Passing**: Channel-based communication between stdin/stdout threads

### Adding New Features

**New Image Formats**:
1. Add tool definition in `src/handlers/image.rs`
2. Implement conversion method in the appropriate converter
3. Update `handle_tool_call` in `server.rs`

**New Converters**:
1. Create new file in `src/converters/`
2. Implement `ImageConverter` trait
3. Add to `AutoConverter::new()` in `src/converters/mod.rs`
4. Set appropriate priority (0-100)

## Important Context

- The project uses ImageMagick 7 (accessed via `convert7` or `magick` commands)
- Darktable provides better color management for RAW files
- All conversions are lossless (WebP lossless mode)
- The server communicates via stdio, not HTTP
- Logging goes to stderr to avoid interfering with MCP protocol on stdout

## Dependencies to Know

- `mcpr` - MCP protocol implementation (this is not the official SDK)
- `tokio` - Async runtime with full features
- `which` - For checking converter availability
- `tracing` - Structured logging (not `log` crate)