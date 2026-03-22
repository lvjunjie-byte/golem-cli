# Changelog - MCP Server

## [Unreleased]

### Added
- **MCP Server Implementation** - Complete MCP (Model Context Protocol) server for Golem CLI
  - stdio transport mode for local AI assistant integration
  - streamable-http transport mode for remote connections
  - Full support for MCP specification version 2025-11-25

- **8 MCP Tools**:
  - `component_new` - Create new Golem components from templates
  - `component_build` - Build components with customizable profiles
  - `component_deploy` - Deploy components to Golem Cloud
  - `app_new` - Create new Golem applications
  - `app_deploy` - Deploy applications to Golem Cloud
  - `worker_create` - Create workers for components
  - `worker_invoke` - Invoke worker functions with parameters
  - `get_status` - Get CLI status and diagnostics

- **2 MCP Resources**:
  - `golem://manifest/current` - Access current directory manifest
  - `golem://manifest/parent` - Access parent directory manifest

- **CLI Integration**:
  - New command: `golem-cli server mcp`
  - HTTP mode flag: `--http`
  - Custom port: `--port <number>`
  - Feature flag: `mcp-server`

- **Testing**:
  - Unit tests for helper functions (3 tests)
  - Integration tests for tools (15 tests)
  - Integration tests for resources (7 tests)
  - Schema validation tests

- **Documentation**:
  - Complete MCP_README.md with usage examples
  - Inline code documentation
  - API reference for all tools and resources
  - Configuration examples for Claude Desktop

### Changed
- Enhanced error handling with detailed messages
- Improved JSON Schema definitions for better AI understanding
- Optimized CLI command execution with tokio async runtime

### Technical Details
- **Dependencies**:
  - `rust-mcp-sdk` v0.9.0 (server, macros, stdio, streamable-http features)
  - `rust-mcp-schema` v0.9.0
  - `tokio` (async runtime)
  - `serde_json` (JSON handling)
  - `anyhow` (error handling)

- **File Structure**:
  ```
  src/
  ├── command_handler/
  │   └── server/
  │       ├── mcp_server.rs    # MCP server core
  │       ├── tools.rs         # Tools definition and implementation
  │       ├── resources.rs     # Resources definition
  │       └── mod.rs           # Module exports
  ├── mcp_tools_impl.rs        # Tool function implementations
  ├── command.rs               # CLI command definitions (updated)
  └── main.rs                  # Main entry point (updated)
  
  tests/
  └── mcp/
      ├── mod.rs              # MCP integration tests
      ├── tools.rs            # Tools tests
      └── resources.rs        # Resources tests
  ```

- **Build Configuration**:
  ```toml
  [features]
  mcp-server = ["dep:rust-mcp-sdk", "dep:rust-mcp-schema"]
  
  [dependencies]
  rust-mcp-sdk = { version = "0.9.0", features = ["server", "macros", "stdio", "streamable-http"], optional = true }
  rust-mcp-schema = { version = "0.9.0", optional = true }
  ```

### Usage Examples

**stdio mode** (for local AI assistants):
```bash
golem-cli server mcp
```

**HTTP mode** (for remote connections):
```bash
golem-cli server mcp --http --port 1232
```

**Claude Desktop Configuration**:
```json
{
  "mcpServers": {
    "golem": {
      "command": "golem-cli",
      "args": ["server", "mcp"]
    }
  }
}
```

### Testing

Run all tests:
```bash
cargo test --features mcp-server
```

Run MCP-specific tests:
```bash
cargo test --features mcp-server --test mcp
```

### Known Issues
- None at this time

### Future Enhancements
- [ ] Add more MCP resources (component status, worker status, etc.)
- [ ] Implement MCP notifications for long-running operations
- [ ] Add support for MCP prompts
- [ ] Enhance logging with structured output
- [ ] Add performance monitoring and metrics
- [ ] Support for custom templates in component_new
- [ ] Batch operations for multiple components

---

## Previous Versions

### [0.0.0] - Initial Release
- Basic CLI functionality
- Component management
- Application management
- Worker management
- Cloud integration

---

**Date**: 2026-03-22  
**Author**: Golem Team  
**Related Issue**: #275 (MCP Server Implementation)
