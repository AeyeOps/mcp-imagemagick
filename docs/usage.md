# Usage Guide

## Running the Server

### Standalone Mode
```bash
./mcp-imagemagick
```

The server will start listening on stdin/stdout for MCP protocol messages.

### With Claude Desktop

1. Build the project:
   ```bash
   cargo build --release
   ```

2. Add to your Claude Desktop configuration file (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):
   ```json
   {
     "mcpServers": {
       "imagemagick": {
         "command": "/path/to/mcp-imagemagick/target/release/mcp-imagemagick",
         "args": [],
         "env": {}
       }
     }
   }
   ```

3. Restart Claude Desktop

## Using the Tools

### Convert DNG to WebP

Basic usage:
```
Convert the file photo.dng to photo.webp
```

With specific converter:
```
Convert photo.dng to photo.webp using darktable
```

### Check Available Converters

```
Check which image converters are available
```

## Environment Variables

- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=debug`)
- `IMAGE_MAGICK_DIR`: Override ImageMagick installation directory
- `PATH`: Should include directories containing `convert7` or `darktable-cli`

## Testing the Server

Use the included test script:
```bash
python3 test_mcp.py
```

Or test manually with JSON-RPC messages:
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./mcp-imagemagick
```

## Tips

1. **For DNG files**: Use darktable converter for best results
2. **For batch processing**: Call the tool multiple times with different file paths
3. **Output formats**: Currently only WebP is supported, but the architecture allows easy extension
4. **Performance**: ImageMagick is faster for supported formats, darktable provides better quality for RAW files