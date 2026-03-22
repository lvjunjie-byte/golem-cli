# Golem CLI MCP Server

[![MCP Spec Version](https://img.shields.io/badge/MCP-2025--11--25-blue)](https://modelcontextprotocol.io/)
[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Golem%20Source%20License%20v1.0-lightgrey)](../LICENSE)

**MCP (Model Context Protocol) Server for Golem CLI** - Enables AI agents to interact with Golem Cloud platform through standardized tools and resources.

## 🚀 Features

- **8 MCP Tools** for complete Golem Cloud workflow
- **2 MCP Resources** for manifest file access
- **Dual Transport Modes**: stdio and streamable-http
- **Full CLI Integration** via `golem-cli server mcp` command
- **Comprehensive Error Handling** with detailed messages
- **AI-Optimized** JSON Schema definitions

## 📦 Installation

### Prerequisites

- Rust 1.75 or later
- Golem CLI dependencies
- MCP-compatible AI assistant (Claude Desktop, etc.)

### Build from Source

```bash
cd golem-cli
cargo build --release --features mcp-server
```

The binary will be available at `target/release/golem-cli`.

## 🎯 Usage

### stdio Mode (Recommended for Local AI Assistants)

```bash
golem-cli server mcp
```

This starts the MCP server using standard input/output, perfect for integration with local AI assistants.

### HTTP Mode (Recommended for Remote Connections)

```bash
golem-cli server mcp --http --port 1232
```

This starts the MCP server on `http://127.0.0.1:1232` with streamable-http transport.

### Custom Port

```bash
golem-cli server mcp --http --port 8080
```

## 🔧 Configuration

### Claude Desktop Configuration

Add the following to your Claude Desktop configuration:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

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

### HTTP Mode Configuration

```json
{
  "mcpServers": {
    "golem": {
      "url": "http://127.0.0.1:1232",
      "transport": "streamable-http"
    }
  }
}
```

## 🛠️ Available Tools

### 1. `component_new` - Create a New Component

Create a new Golem component from a template.

**Parameters:**
- `name` (required): Name of the component
- `template` (optional, default: "rust"): Template type (rust, typescript, etc.)
- `path` (optional, default: "<name>"): Output path

**Example:**
```json
{
  "name": "my-component",
  "template": "rust"
}
```

**Response:**
```
✅ Component 'my-component' created successfully from template 'rust'
```

### 2. `component_build` - Build a Component

Build a Golem component.

**Parameters:**
- `path` (optional, default: "."): Path to component
- `profile` (optional, default: "default"): Build profile
- `release` (optional, default: false): Release mode

**Example:**
```json
{
  "path": "./my-component",
  "release": true
}
```

**Response:**
```
✅ Component built successfully from './my-component'
```

### 3. `component_deploy` - Deploy a Component

Deploy a component to Golem Cloud.

**Parameters:**
- `path` (required): Path to component
- `component_name` (optional): Component name
- `profile` (optional, default: "default"): Profile name

**Example:**
```json
{
  "path": "./my-component",
  "component_name": "my-component-prod"
}
```

**Response:**
```
✅ Component deployed successfully from './my-component'
Component ID: abc-123-def
```

### 4. `app_new` - Create a New Application

Create a new Golem application.

**Parameters:**
- `name` (required): Application name
- `path` (optional, default: "<name>"): Output path

**Example:**
```json
{
  "name": "my-app"
}
```

**Response:**
```
✅ Application 'my-app' created successfully
```

### 5. `app_deploy` - Deploy an Application

Deploy a Golem application.

**Parameters:**
- `path` (required): Path to application
- `app_name` (optional): Application name
- `profile` (optional, default: "default"): Profile name

**Example:**
```json
{
  "path": "./my-app",
  "app_name": "my-app-prod"
}
```

**Response:**
```
✅ Application deployed successfully from './my-app'
```

### 6. `worker_create` - Create a Worker

Create a new worker for a component.

**Parameters:**
- `worker_name` (required): Worker name
- `component_id` (required): Component ID
- `profile` (optional, default: "default"): Profile name

**Example:**
```json
{
  "worker_name": "my-worker",
  "component_id": "abc-123-def"
}
```

**Response:**
```
✅ Worker 'my-worker' created successfully for component 'abc-123-def'
Worker ID: worker-xyz
```

### 7. `worker_invoke` - Invoke a Worker Function

Invoke a worker function.

**Parameters:**
- `worker_name` (required): Worker name
- `function_name` (required): Function to invoke
- `params` (optional): Function parameters (array of strings)
- `profile` (optional, default: "default"): Profile name

**Example:**
```json
{
  "worker_name": "my-worker",
  "function_name": "process",
  "params": ["param1", "param2"]
}
```

**Response:**
```
✅ Worker invoked successfully
Result: processed data
```

### 8. `get_status` - Get Status

Get Golem CLI status and diagnostics.

**Parameters:**
- `profile` (optional, default: "default"): Profile name

**Example:**
```json
{
  "profile": "default"
}
```

**Response:**
```
📊 Golem CLI Status
Version: 1.2.3
Profile: default
Connection: OK
```

## 📚 Available Resources

### 1. `golem://manifest/current`

Reads the `golem.yaml` manifest file from the current directory.

**MIME Type:** `application/yaml`

**Example:**
```json
{
  "uri": "golem://manifest/current"
}
```

**Response:**
```yaml
name: my-component
version: 1.0.0
template: rust
```

### 2. `golem://manifest/parent`

Reads the `golem.yaml` manifest file from the parent directory.

**MIME Type:** `application/yaml`

**Example:**
```json
{
  "uri": "golem://manifest/parent"
}
```

## 🧪 Testing

### Run Unit Tests

```bash
cargo test --features mcp-server
```

### Run Integration Tests

```bash
cargo test --features mcp-server --test mcp
```

### Test MCP Server Manually

```bash
# Start server in stdio mode
golem-cli server mcp

# In another terminal, send a test request
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | golem-cli server mcp
```

## 📝 Example Workflow

Here's a complete example of using the MCP server with an AI assistant:

1. **Create a component:**
   ```
   AI: Create a new Rust component called "hello-world"
   Tool: component_new with {"name": "hello-world", "template": "rust"}
   Result: ✅ Component created
   ```

2. **Build the component:**
   ```
   AI: Build the component
   Tool: component_build with {"path": "./hello-world"}
   Result: ✅ Component built
   ```

3. **Deploy the component:**
   ```
   AI: Deploy to production
   Tool: component_deploy with {"path": "./hello-world", "component_name": "hello-world-prod"}
   Result: ✅ Component deployed, ID: abc-123
   ```

4. **Create a worker:**
   ```
   AI: Create a worker for the component
   Tool: worker_create with {"worker_name": "hello-worker", "component_id": "abc-123"}
   Result: ✅ Worker created
   ```

5. **Invoke the worker:**
   ```
   AI: Test the worker
   Tool: worker_invoke with {"worker_name": "hello-worker", "function_name": "run"}
   Result: ✅ Worker invoked successfully
   ```

## 🔍 Debugging

### Enable Verbose Logging

```bash
RUST_LOG=debug golem-cli server mcp
```

### Check Server Status

```bash
golem-cli diagnose
```

### View Logs

Logs are written to stderr. Redirect to a file:

```bash
golem-cli server mcp 2> mcp-server.log
```

## 🛡️ Security Considerations

- The MCP server runs with the same permissions as the user
- Ensure proper authentication for Golem Cloud API access
- Use HTTPS for remote HTTP mode connections
- Validate all input parameters (already implemented)

## 📊 Architecture

```
┌─────────────────┐
│   AI Assistant  │
│  (Claude, etc.) │
└────────┬────────┘
         │ MCP Protocol
         ▼
┌─────────────────┐
│  GolemMcpHandler│
│  (mcp_server.rs)│
└────────┬────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌─────────┐ ┌──────────┐
│ Tools   │ │ Resources│
│(8 tools)│ │(2 resources)│
└────┬────┘ └────┬─────┘
     │           │
     ▼           ▼
┌─────────────────────┐
│   Golem CLI Core    │
│  (command_handler)  │
└─────────────────────┘
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --features mcp-server`
5. Submit a pull request

## 📄 License

This project is licensed under the Golem Source License v1.0 - see the [LICENSE](../LICENSE) file for details.

## 🔗 Resources

- [Golem Cloud Documentation](https://golem.cloud/docs)
- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust MCP SDK](https://crates.io/crates/rust-mcp-sdk)
- [Golem CLI Repository](https://github.com/golemcloud/golem)

## 💬 Support

For issues and questions:
- GitHub Issues: https://github.com/golemcloud/golem-cli/issues
- Discord: https://discord.gg/golemcloud
- Email: support@golem.cloud

---

**Built with ❤️ by the Golem Team**
