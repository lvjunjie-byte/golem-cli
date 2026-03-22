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
//! 
//! Full implementation with actual CLI command integration

use rust_mcp_sdk::schema::*;
use serde_json::{Value, json};
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::process::Command as TokioCommand;

use crate::context::Context;

/// Golem CLI Tools manager
pub struct GolemTools {
    tools: Vec<Tool>,
}

impl GolemTools {
    pub fn new() -> Self {
        let tools = vec![
            Tool {
                name: "component_new".into(),
                description: Some("Create a new Golem component".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "description": "Name of the component"},
                        "template": {"type": "string", "description": "Template (rust, typescript, etc.)", "default": "rust"},
                        "path": {"type": "string", "description": "Output path", "default": "<name>"}
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
                        "path": {"type": "string", "description": "Path to component", "default": "."},
                        "profile": {"type": "string", "description": "Profile name", "default": "default"},
                        "release": {"type": "boolean", "description": "Release mode", "default": false}
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
                        "path": {"type": "string", "description": "Path to component", "default": "."},
                        "component_name": {"type": "string", "description": "Component name"},
                        "profile": {"type": "string", "description": "Profile name", "default": "default"}
                    },
                    "required": ["path"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "app_new".into(),
                description: Some("Create a new Golem application".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "description": "Application name"},
                        "path": {"type": "string", "description": "Output path", "default": "<name>"}
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
                description: Some("Deploy a Golem application".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "Path to application", "default": "."},
                        "app_name": {"type": "string", "description": "Application name"},
                        "profile": {"type": "string", "description": "Profile name", "default": "default"}
                    },
                    "required": ["path"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "worker_create".into(),
                description: Some("Create a new worker for a component".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "worker_name": {"type": "string", "description": "Worker name"},
                        "component_id": {"type": "string", "description": "Component ID"},
                        "profile": {"type": "string", "description": "Profile name", "default": "default"}
                    },
                    "required": ["worker_name", "component_id"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "worker_invoke".into(),
                description: Some("Invoke a worker function".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "worker_name": {"type": "string", "description": "Worker name"},
                        "function_name": {"type": "string", "description": "Function to invoke"},
                        "params": {"type": "array", "items": {"type": "string"}, "description": "Function parameters"},
                        "profile": {"type": "string", "description": "Profile name", "default": "default"}
                    },
                    "required": ["worker_name", "function_name"]
                }),
                icons: vec![],
                execution: None,
                annotations: None,
                meta: None,
            },
            Tool {
                name: "get_status".into(),
                description: Some("Get Golem CLI status".into()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "profile": {"type": "string", "description": "Profile name", "default": "default"}
                    }
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
    
    pub async fn call_tool(&self, name: &str, arguments: Option<Value>, _ctx: &Arc<Context>) -> Result<CallToolResult> {
        let result = match name {
            "component_new" => self.component_new(arguments).await?,
            "component_build" => self.component_build(arguments).await?,
            "component_deploy" => self.component_deploy(arguments).await?,
            "app_new" => self.app_new(arguments).await?,
            "app_deploy" => self.app_deploy(arguments).await?,
            "worker_create" => self.worker_create(arguments).await?,
            "worker_invoke" => self.worker_invoke(arguments).await?,
            "get_status" => self.get_status(arguments).await?,
            _ => return Err(anyhow::anyhow!("Unknown tool: {}", name)),
        };
        
        Ok(CallToolResult {
            content: vec![Content::TextContent(TextContent {
                r#type: "text".into(),
                text: result,
                annotations: None,
                meta: None,
            })],
            is_error: None,
            meta: None,
        })
    }
    
    async fn component_new(&self, args: Option<Value>) -> Result<String> {
        let args = args.context("Missing arguments")?;
        let name = args.get("name").and_then(|v| v.as_str()).context("name is required")?;
        let template = args.get("template").and_then(|v| v.as_str()).unwrap_or("rust");
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(name);
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("component").arg("new").arg(name)
            .arg("--template").arg(template)
            .arg("--path").arg(path);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            Ok(format!("✅ Component '{}' created successfully from template '{}'", name, template))
        } else {
            Err(anyhow::anyhow!("❌ Component creation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn component_build(&self, args: Option<Value>) -> Result<String> {
        let path = args.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or(".");
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        let release = args.as_ref().and_then(|a| a.get("release")).and_then(|v| v.as_bool()).unwrap_or(false);
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("component").arg("build")
            .arg("--path").arg(path)
            .arg("--profile").arg(profile);
        if release { cmd.arg("--release"); }
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            Ok(format!("✅ Component built successfully from '{}'", path))
        } else {
            Err(anyhow::anyhow!("❌ Build failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn component_deploy(&self, args: Option<Value>) -> Result<String> {
        let path = args.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or(".");
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("component").arg("deploy")
            .arg("--path").arg(path)
            .arg("--profile").arg(profile);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(format!("✅ Component deployed successfully from '{}'\n{}", path, stdout))
        } else {
            Err(anyhow::anyhow!("❌ Deploy failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn app_new(&self, args: Option<Value>) -> Result<String> {
        let name = args.as_ref().and_then(|a| a.get("name")).and_then(|v| v.as_str()).context("name is required")?;
        let path = args.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or(name);
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("app").arg("new").arg(name).arg("--path").arg(path);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            Ok(format!("✅ Application '{}' created successfully", name))
        } else {
            Err(anyhow::anyhow!("❌ Application creation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn app_deploy(&self, args: Option<Value>) -> Result<String> {
        let path = args.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or(".");
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("app").arg("deploy")
            .arg("--path").arg(path)
            .arg("--profile").arg(profile);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            Ok(format!("✅ Application deployed successfully from '{}'", path))
        } else {
            Err(anyhow::anyhow!("❌ Deploy failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn worker_create(&self, args: Option<Value>) -> Result<String> {
        let worker_name = args.as_ref().and_then(|a| a.get("worker_name")).and_then(|v| v.as_str()).context("worker_name is required")?;
        let component_id = args.as_ref().and_then(|a| a.get("component_id")).and_then(|v| v.as_str()).context("component_id is required")?;
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("worker").arg("create").arg(worker_name)
            .arg("--component").arg(component_id)
            .arg("--profile").arg(profile);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            Ok(format!("✅ Worker '{}' created successfully for component '{}'", worker_name, component_id))
        } else {
            Err(anyhow::anyhow!("❌ Worker creation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn worker_invoke(&self, args: Option<Value>) -> Result<String> {
        let worker_name = args.as_ref().and_then(|a| a.get("worker_name")).and_then(|v| v.as_str()).context("worker_name is required")?;
        let function_name = args.as_ref().and_then(|a| a.get("function_name")).and_then(|v| v.as_str()).context("function_name is required")?;
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("worker").arg("invoke").arg(worker_name).arg(function_name)
            .arg("--profile").arg(profile);
        
        if let Some(params) = args.and_then(|a| a.get("params")).and_then(|v| v.as_array()) {
            for param in params {
                if let Some(p) = param.as_str() {
                    cmd.arg(p);
                }
            }
        }
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(format!("✅ Worker invoked successfully\n{}", stdout))
        } else {
            Err(anyhow::anyhow!("❌ Invocation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn get_status(&self, args: Option<Value>) -> Result<String> {
        let profile = args.as_ref().and_then(|a| a.get("profile")).and_then(|v| v.as_str()).unwrap_or("default");
        
        let mut cmd = TokioCommand::new("golem-cli");
        cmd.arg("diagnose").arg("--profile").arg(profile);
        
        let output = cmd.output().await.context("Failed to execute command")?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(format!("📊 Golem CLI Status\n{}", stdout))
        } else {
            Err(anyhow::anyhow!("❌ Status check failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
}

impl Default for GolemTools {
    fn default() -> Self {
        Self::new()
    }
}
