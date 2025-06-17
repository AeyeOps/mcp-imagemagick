# API Documentation

## MCP Protocol Implementation

This server implements the Model Context Protocol (MCP) specification version 2024-11-01.

## Available Tools

### convert_dng_to_webp

Convert a DNG (Digital Negative) image to WebP format without loss.

#### Input Schema
```json
{
  "type": "object",
  "properties": {
    "input_path": {
      "type": "string",
      "description": "Path to DNG file"
    },
    "output_path": {
      "type": "string",
      "description": "Path for WebP output"
    },
    "converter": {
      "type": "string",
      "enum": ["auto", "imagemagick", "darktable"],
      "default": "auto",
      "description": "Which converter to use (auto selects the best available)"
    }
  },
  "required": ["input_path", "output_path"]
}
```

#### Response
```json
{
  "success": true,
  "message": "Successfully converted /path/to/input.dng to /path/to/output.webp using darktable",
  "output_path": "/path/to/output.webp"
}
```

#### Error Response
```json
{
  "error": {
    "code": -32603,
    "message": "Conversion failed: [error details]"
  }
}
```

### check_converters

Check which image converters are available on the system.

#### Input Schema
```json
{
  "type": "object",
  "properties": {}
}
```

#### Response
```json
{
  "converters": [
    {
      "name": "imagemagick",
      "available": true
    },
    {
      "name": "darktable",
      "available": true
    }
  ],
  "available_count": 2
}
```

## Protocol Messages

### Initialize
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {}
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-01",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "mcp-imagemagick",
      "version": "0.1.0"
    }
  }
}
```

### List Tools
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

### Call Tool
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "convert_dng_to_webp",
    "arguments": {
      "input_path": "/path/to/image.dng",
      "output_path": "/path/to/output.webp",
      "converter": "auto"
    }
  }
}
```

## Error Codes

- `-32603`: Internal error (e.g., conversion failed)
- `-32602`: Invalid params
- `-32601`: Method not found

## Converter Details

### ImageMagick Converter
- Command: `convert7` or `magick`
- Priority: 60 (higher = preferred)
- Settings:
  - `webp:lossless=true`
  - `webp:exact=true`
  - `webp:method=6`
  - `webp:partition-limit=0`

### Darktable Converter
- Command: `darktable-cli`
- Priority: 40
- Features:
  - Native RAW processing
  - Better color management
  - XMP sidecar support

## File Validation

- Input file must exist
- Input file must have `.dng` or `.DNG` extension
- Output directory will be created if it doesn't exist
- Existing output files will be overwritten