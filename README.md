# mcp-imagemagick

An MCP (Model Context Protocol) server for image conversion using ImageMagick and darktable.

## Features

- Convert DNG (Digital Negative) files to WebP format losslessly
- Support for both ImageMagick 7 and darktable-cli converters
- Automatic converter selection based on availability
- Full MCP protocol implementation with stdio transport
- Detailed error reporting and logging

## Prerequisites

You need at least one of the following image converters installed:

### ImageMagick 7
- Available as `convert7` or `magick` command
- Install: `sudo apt install imagemagick` (Ubuntu/Debian)
- Verify: `convert7 -version` or `magick -version`

### darktable-cli
- Command-line interface for darktable RAW processor
- Install: `sudo apt install darktable`
- Verify: `darktable-cli --version`

## Installation

### From Source

```bash
git clone https://github.com/yourusername/mcp-imagemagick
cd mcp-imagemagick
cargo build --release
```

The binary will be available at `target/release/mcp-imagemagick`.

### From crates.io (coming soon)

```bash
cargo install mcp-imagemagick
```

## Usage

### As an MCP Server

This tool implements the MCP protocol and can be used with any MCP client like Claude Desktop.

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "imagemagick": {
      "command": "/path/to/mcp-imagemagick",
      "args": [],
      "env": {}
    }
  }
}
```

### Available Tools

#### convert_dng_to_webp
Convert a DNG file to WebP format without loss.

Parameters:
- `input_path` (string, required): Path to the DNG file
- `output_path` (string, required): Path for the WebP output
- `converter` (string, optional): Which converter to use ("auto", "imagemagick", "darktable")

Example:
```json
{
  "tool": "convert_dng_to_webp",
  "arguments": {
    "input_path": "/path/to/image.dng",
    "output_path": "/path/to/output.webp",
    "converter": "auto"
  }
}
```

#### check_converters
Check which image converters are available on the system.

No parameters required.

## Technical Details

### Conversion Methods

#### ImageMagick
Uses the following settings for lossless WebP conversion:
- `webp:lossless=true` - Enable lossless compression
- `webp:exact=true` - Preserve exact pixel values
- `webp:method=6` - Maximum compression
- `webp:partition-limit=0` - Disable partition limit

#### darktable-cli
- Uses darktable's RAW processing pipeline
- Better color management for RAW files
- Automatically applies any XMP sidecar files

### Priority System

When using "auto" converter selection:
1. ImageMagick (priority: 60) - Faster, good for quick conversions
2. darktable (priority: 40) - Better RAW processing, more accurate colors

## Development

### Project Structure

```
mcp-imagemagick/
├── src/
│   ├── main.rs           # Entry point
│   ├── server.rs         # MCP server implementation
│   ├── transport.rs      # Stdio transport
│   ├── handlers/         # Request handlers
│   └── converters/       # Image converter implementations
├── kb/                   # Knowledge base documents
└── docs/                 # Additional documentation
```

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Logging

Set the `RUST_LOG` environment variable to control logging:

```bash
RUST_LOG=debug mcp-imagemagick
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.