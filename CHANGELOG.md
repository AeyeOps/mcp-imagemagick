# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-06-17

### Added
- Proper MCP protocol implementation following specification
- Robust error handling and recovery
- Comprehensive logging to stderr with different verbosity levels
- Fallback converter system with automatic retry
- Panic handler to log crashes before exit
- Request/response logging for debugging
- Tool call error handling with proper JSON-RPC error codes

### Fixed
- JSON-RPC message format compliance (single-line messages)
- Protocol version updated to 2024-11-05
- Synchronous stdio transport implementation
- Response structure properly wrapped in result object
- Server stability - no longer crashes on conversion errors
- Proper error codes for different failure types

### Changed
- Simplified transport layer from async channels to synchronous line reading
- Improved error messages with more context
- Better converter selection logic with priorities

## [0.1.0] - 2024-06-17

### Added
- Initial MCP server implementation
- DNG to WebP conversion support
- ImageMagick and darktable converter backends
- Basic error handling
- Integration test suite