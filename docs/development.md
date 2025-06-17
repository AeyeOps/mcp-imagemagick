# Development Guide

## Project Structure

```
mcp-imagemagick/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs           # Library exports and error types
│   ├── server.rs        # MCP server implementation
│   ├── transport.rs     # Stdio transport handling
│   ├── handlers/
│   │   └── image.rs     # Image conversion request handlers
│   └── converters/
│       ├── mod.rs       # Converter trait and auto-selection
│       ├── imagemagick.rs # ImageMagick converter
│       └── darktable.rs   # Darktable converter
└── kb/                  # Knowledge base documents
```

## Adding New Converters

1. Create a new file in `src/converters/`:
   ```rust
   // src/converters/newconverter.rs
   use async_trait::async_trait;
   use super::ImageConverter;
   
   pub struct NewConverter;
   
   #[async_trait]
   impl ImageConverter for NewConverter {
       async fn convert_dng_to_webp(&self, input: &Path, output: &Path) -> Result<()> {
           // Implementation
       }
       
       fn is_available(&self) -> bool {
           which("new-converter-cli").is_ok()
       }
       
       fn name(&self) -> &'static str {
           "newconverter"
       }
       
       fn priority(&self) -> u8 {
           50 // Adjust based on preference
       }
   }
   ```

2. Add to `src/converters/mod.rs`:
   ```rust
   mod newconverter;
   pub use newconverter::NewConverter;
   ```

3. Update `AutoConverter::new()` and `ImageHandler::new()`

## Adding New Image Formats

1. Add new tool definition in `src/handlers/image.rs`
2. Implement conversion method
3. Update tool schema
4. Add to `handle_tool_call` match statement

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
python3 test_mcp.py
```

### Manual Testing with MCP Inspector
```bash
# Install MCP Inspector
npm install -g @modelcontextprotocol/inspector

# Run inspector
mcp-inspector ./target/release/mcp-imagemagick
```

## Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug ./target/release/mcp-imagemagick
```

### Common Issues

1. **Converter not found**: Check PATH and which() results
2. **JSON parsing errors**: Validate with `jq` or online JSON validator
3. **Async issues**: Ensure proper await usage in converter implementations

## Performance Optimization

1. **Parallel Processing**: Consider using `tokio::spawn` for batch operations
2. **Memory Usage**: Stream large files instead of loading entirely
3. **Caching**: Add converter availability caching to avoid repeated `which` calls

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo fmt` and `cargo clippy` pass
5. Submit a pull request

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Tag release: `git tag v0.1.0`
4. Build release binaries:
   ```bash
   cargo build --release --target x86_64-unknown-linux-gnu
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target x86_64-pc-windows-msvc
   ```
5. Create GitHub release with binaries