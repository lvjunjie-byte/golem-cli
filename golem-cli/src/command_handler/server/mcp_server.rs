// Copyright 2024-2025 Golem Cloud
//
// Licensed under the Golem Source License v1.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://license.golem.cloud/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! MCP Server core implementation

use anyhow::{Context, Result};
use rust_mcp_sdk::schema::*;
use rust_mcp_sdk::{
    mcp_server::{hyper_server, server_runtime, HyperServerOptions, ServerHandler},
    *,
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, error};

use crate::context::Context;
use crate::command_handler::server::tools::GolemTools;
use crate::command_handler::server::resources::GolemResources;

/// MCP Server handler for Golem CLI
pub struct GolemMcpHandler {
    ctx: Arc<Context>,
    tools: GolemTools,
    resources: GolemResources,
}

impl GolemMcpHandler {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self {
            ctx,
            tools: GolemTools::new(),
            resources: GolemResources::new(),
        }
    }
}

#[async_trait]
impl ServerHandler for GolemMcpHandler {
    /// Handle list tools request
    async fn handle_list_tools_request(
        &self,
        _request: Option<()>,
        _runtime: std::sync::Arc<rust_mcp_sdk::mcp_server::ServerRuntime>,
    ) -> Result<ListToolsResult, ListToolsError> {
        info!("Listing MCP tools");
        Ok(ListToolsResult {
            tools: self.tools.list_tools(),
            meta: None,
            next_cursor: None,
        })
    }

    /// Handle call tool request
    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: std::sync::Arc<rust_mcp_sdk::mcp_server::ServerRuntime>,
    ) -> Result<CallToolResult, CallToolError> {
        info!("Calling tool: {}", params.name);
        
        match self.tools.call_tool(&params.name, params.arguments, &self.ctx).await {
            Ok(result) => Ok(result),
            Err(e) => Err(CallToolError::unknown(e.to_string())),
        }
    }

    /// Handle list resources request
    async fn handle_list_resources_request(
        &self,
        _request: Option<()>,
        _runtime: std::sync::Arc<rust_mcp_server::ServerRuntime>,
    ) -> Result<ListResourcesResult, ListResourcesError> {
        info!("Listing MCP resources");
        Ok(ListResourcesResult {
            resources: self.resources.list_resources(),
            meta: None,
            next_cursor: None,
        })
    }

    /// Handle read resource request
    async fn handle_read_resource_request(
        &self,
        params: ReadResourceRequestParams,
        _runtime: std::sync::Arc<rust_mcp_server::ServerRuntime>,
    ) -> Result<ReadResourceResult, ReadResourceError> {
        info!("Reading resource: {}", params.uri);
        
        match self.resources.read_resource(&params.uri).await {
            Ok(content) => Ok(ReadResourceResult { contents: content }),
            Err(e) => Err(ReadResourceError::unknown(e.to_string())),
        }
    }

    /// Handle initialize request
    async fn handle_initialize_request(
        &self,
        params: InitializeRequestParams,
        runtime: std::sync::Arc<rust_mcp_server::ServerRuntime>,
    ) -> Result<InitializeResult, InitializeError> {
        info!("MCP Client initialized: {:?}", params.client_info);
        
        Ok(InitializeResult {
            server_info: Implementation {
                name: "golem-cli-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: Some("Golem CLI MCP Server".into()),
                description: Some("MCP Server for Golem CLI - Enables AI agents to interact with Golem Cloud".into()),
                icons: vec![],
                website_url: Some("https://golem.cloud".into()),
            },
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: Some(false),
                }),
                resources: Some(ServerCapabilitiesResources {
                    subscribe: Some(false),
                    list_changed: Some(false),
                }),
                ..Default::default()
            },
            protocol_version: ProtocolVersion::V2025_11_25.into(),
            instructions: Some("Use this server to interact with Golem Cloud platform. You can create, build, and deploy WebAssembly components, manage applications and workers, and more.".into()),
            meta: None,
        })
    }
}

/// Start MCP server in stdio mode
pub async fn start_mcp_server_stdio(ctx: Arc<Context>) -> Result<()> {
    info!("Starting Golem MCP Server in stdio mode");
    
    let server_info = InitializeResult {
        server_info: Implementation {
            name: "golem-cli-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("Golem CLI MCP Server".into()),
            description: Some("MCP Server for Golem CLI".into()),
            icons: vec![],
            website_url: Some("https://golem.cloud".into()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools {
                list_changed: Some(false),
            }),
            resources: Some(ServerCapabilitiesResources {
                subscribe: Some(false),
                list_changed: Some(false),
            }),
            ..Default::default()
        },
        protocol_version: ProtocolVersion::V2025_11_25.into(),
        instructions: None,
        meta: None,
    };

    let transport = StdioTransport::new(TransportOptions::default())
        .context("Failed to create stdio transport")?;
    
    let handler = GolemMcpHandler::new(ctx).to_mcp_server_handler();
    let server = server_runtime::create_server(server_info, transport, handler);
    
    info!("MCP Server started (stdio mode)");
    server.start().await.context("MCP server failed")?;
    
    Ok(())
}

/// Start MCP server in streamable HTTP mode
pub async fn start_mcp_server_http(ctx: Arc<Context>, port: u16) -> Result<()> {
    info!("Starting Golem MCP Server in HTTP mode on port {}", port);
    
    let server_info = InitializeResult {
        server_info: Implementation {
            name: "golem-cli-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("Golem CLI MCP Server".into()),
            description: Some("MCP Server for Golem CLI".into()),
            icons: vec![],
            website_url: Some("https://golem.cloud".into()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools {
                list_changed: Some(false),
            }),
            resources: Some(ServerCapabilitiesResources {
                subscribe: Some(false),
                list_changed: Some(false),
            }),
            ..Default::default()
        },
        protocol_version: ProtocolVersion::V2025_11_25.into(),
        instructions: None,
        meta: None,
    };

    let handler = GolemMcpHandler::new(ctx).to_mcp_server_handler();
    let server = hyper_server::create_server(
        server_info,
        handler,
        HyperServerOptions {
            host: "127.0.0.1".to_string(),
            port,
            event_store: Some(std::sync::Arc::new(rust_mcp_sdk::event_store::InMemoryEventStore::default())),
            ..Default::default()
        },
    );
    
    info!("MCP Server started (HTTP mode on port {})", port);
    server.start().await.context("MCP server failed")?;
    
    Ok(())
}
