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

//! MCP Resources definitions for Golem CLI
//! 
//! Exposes manifest files (golem.yaml) as MCP resources

use rust_mcp_sdk::schema::*;
use anyhow::{Result, Context};
use std::path::PathBuf;
use tokio::fs;

/// Golem CLI Resources manager
pub struct GolemResources;

impl GolemResources {
    pub fn new() -> Self {
        Self
    }
    
    pub fn list_resources(&self) -> Vec<Resource> {
        vec![
            Resource {
                uri: "golem://manifest/current".into(),
                name: "Current Directory Manifest".into(),
                description: Some("golem.yaml manifest file in the current directory".into()),
                mime_type: Some("application/yaml".into()),
                icons: vec![],
                annotations: None,
                meta: None,
            },
            Resource {
                uri: "golem://manifest/parent".into(),
                name: "Parent Directory Manifest".into(),
                description: Some("golem.yaml manifest file in the parent directory".into()),
                mime_type: Some("application/yaml".into()),
                icons: vec![],
                annotations: None,
                meta: None,
            },
        ]
    }
    
    pub async fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>> {
        match uri {
            "golem://manifest/current" => {
                self.read_manifest(&std::env::current_dir()?).await
            }
            "golem://manifest/parent" => {
                let parent = std::env::current_dir()?
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("No parent directory"))?
                    .to_path_buf();
                self.read_manifest(&parent).await
            }
            _ => Err(anyhow::anyhow!("Unknown resource: {}", uri)),
        }
    }
    
    async fn read_manifest(&self, dir: &PathBuf) -> Result<Vec<ResourceContents>> {
        let manifest_path = dir.join("golem.yaml");
        
        if !manifest_path.exists() {
            return Err(anyhow::anyhow!("No golem.yaml found in {:?}", dir));
        }
        
        let content = fs::read_to_string(&manifest_path)
            .await
            .context("Failed to read manifest file")?;
        
        Ok(vec![ResourceContents::TextResourceContents {
            uri: format!("file://{}", manifest_path.display()),
            mime_type: Some("application/yaml".into()),
            text: content,
        }])
    }
}

impl Default for GolemResources {
    fn default() -> Self {
        Self::new()
    }
}
