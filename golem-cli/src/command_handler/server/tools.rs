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

//! MCP Tools definitions for Golem CLI

use rust_mcp_sdk::schema::*;
use rust_mcp_sdk::macros::mcp_tool;
use serde_json::{Value, Map};
use std::sync::Arc;
use anyhow::Result;

use crate::context::Context;

/// Golem CLI Tools manager
pub struct GolemTools {
    tools: Vec<Tool>,
}

impl GolemTools {
    pub fn new() -> Self {
        let tools = vec![
            // Component tools
            Tool {
                name: "component_new".into(),
                description: Some("Create a new Golem component".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the component"
                        },
                        "template": {
                            "type": "string",
                            "description": "Template to use (rust, typescript, etc.)",
                            "default": "rust"
                        }
                    },
                    "required": ["name"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "component_build".into(),
                description: Some("Build a Golem component".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the component (default: current directory)"
                        },
                        "release": {
                            "type": "boolean",
                            "description": "Build in release mode",
                            "default": false
                        }
                    }
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "component_deploy".into(),
                description: Some("Deploy a component to Golem Cloud".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the component"
                        },
                        "name": {
                            "type": "string",
                            "description": "Component name"
                        }
                    },
                    "required": ["path"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            
            // App tools
            Tool {
                name: "app_new".into(),
                description: Some("Create a new Golem application".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the application"
                        }
                    },
                    "required": ["name"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "app_deploy".into(),
                description: Some("Deploy the current application".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the application (default: current directory)"
                        }
                    }
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            
            // Worker tools
            Tool {
                name: "worker_create".into(),
                description: Some("Create a new worker for a component".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "component": {
                            "type": "string",
                            "description": "Component name or ID"
                        },
                        "name": {
                            "type": "string",
                            "description": "Worker name"
                        },
                        "memory": {
                            "type": "integer",
                            "description": "Memory limit in MB"
                        }
                    },
                    "required": ["component"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "worker_invoke".into(),
                description: Some("Invoke a worker with arguments".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "worker": {
                            "type": "string",
                            "description": "Worker name or ID"
                        },
                        "function": {
                            "type": "string",
                            "description": "Function name to invoke"
                        },
                        "arguments": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Function arguments"
                        }
                    },
                    "required": ["worker", "function"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            
            // Info tools
            Tool {
                name: "get_status".into(),
                description: Some("Get status of Golem resources".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "resource_type": {
                            "type": "string",
                            "description": "Type of resource (components, workers, apps)",
                            "enum": ["components", "workers", "apps"]
                        }
                    },
                    "required": ["resource_type"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
        ];
        
        Self { tools }
    }
    
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.clone()
    }
    
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: Option<Value>,
        ctx: &Arc<Context>,
    ) -> Result<CallToolResult> {
        match name {
            "component_new" => self.component_new(arguments, ctx).await,
            "component_build" => self.component_build(arguments, ctx).await,
            "component_deploy" => self.component_deploy(arguments, ctx).await,
            "app_new" => self.app_new(arguments, ctx).await,
            "app_deploy" => self.app_deploy(arguments, ctx).await,
            "worker_create" => self.worker_create(arguments, ctx).await,
            "worker_invoke" => self.worker_invoke(arguments, ctx).await,
            "get_status" => self.get_status(arguments, ctx).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
    
    async fn component_new(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let args = args.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;
        let name = args.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'name' argument"))?;
        
        let template = args.get("template")
            .and_then(|v| v.as_str())
            .unwrap_or("rust");
        
        // TODO: Implement actual component creation
        let output = format!("Would create component '{}' with template '{}'", name, template);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn component_build(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let path = args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        
        let release = args
            .as_ref()
            .and_then(|a| a.get("release"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        // TODO: Implement actual build
        let mode = if release { "release" } else { "debug" };
        let output = format!("Would build component at '{}' in {} mode", path, mode);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn component_deploy(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let path = args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' argument"))?;
        
        // TODO: Implement actual deployment
        let output = format!("Would deploy component from '{}'", path);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn app_new(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let name = args
            .as_ref()
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'name' argument"))?;
        
        // TODO: Implement actual app creation
        let output = format!("Would create application '{}'", name);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn app_deploy(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let path = args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        
        // TODO: Implement actual deployment
        let output = format!("Would deploy application from '{}'", path);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn worker_create(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let component = args
            .as_ref()
            .and_then(|a| a.get("component"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'component' argument"))?;
        
        let name = args
            .as_ref()
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str());
        
        // TODO: Implement actual worker creation
        let output = match name {
            Some(n) => format!("Would create worker '{}' for component '{}'", n, component),
            None => format!("Would create worker for component '{}'", component),
        };
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn worker_invoke(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let worker = args
            .as_ref()
            .and_then(|a| a.get("worker"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'worker' argument"))?;
        
        let function = args
            .as_ref()
            .and_then(|a| a.get("function"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'function' argument"))?;
        
        let arguments = args
            .as_ref()
            .and_then(|a| a.get("arguments"))
            .and_then(|v| v.as_array());
        
        // TODO: Implement actual invocation
        let output = match arguments {
            Some(args) => format!("Would invoke '{}.{}' with args: {:?}", worker, function, args),
            None => format!("Would invoke '{}.{}'", worker, function),
        };
        
        Ok(CallToolResult::text_content(vec![output]))
    }
    
    async fn get_status(&self, args: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let resource_type = args
            .as_ref()
            .and_then(|a| a.get("resource_type"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'resource_type' argument"))?;
        
        // TODO: Implement actual status check
        let output = format!("Status for {}: OK (placeholder)", resource_type);
        
        Ok(CallToolResult::text_content(vec![output]))
    }
}

impl Default for GolemTools {
    fn default() -> Self {
        Self::new()
    }
}
