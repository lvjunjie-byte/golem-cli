// MCP Server Tool Implementations for Golem CLI
// Implements all 8 tools with full CLI command integration

use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::process::Command as TokioCommand;
use serde_json::{json, Value};

/// Component New - Creates a new Golem component
pub async fn component_new(args: Value) -> Result<Value> {
    let name = args["name"].as_str().context("name is required")?;
    let template = args["template"].as_str().unwrap_or("rust");
    let path = args["path"].as_str().unwrap_or(name);
    
    log::info!("Creating new component '{}' from template '{}'", name, template);
    
    // Execute: golem-cli component new <name> --template <template> --path <path>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("component")
        .arg("new")
        .arg(name)
        .arg("--template")
        .arg(template)
        .arg("--path")
        .arg(path);
    
    let output = cmd.output().await
        .context("Failed to execute component new command")?;
    
    if output.status.success() {
        Ok(json!({
            "success": true,
            "component_name": name,
            "path": path,
            "template": template,
            "message": format!("Component '{}' created successfully", name)
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Component creation failed: {}", stderr))
    }
}

/// Component Build - Builds a Golem component
pub async fn component_build(args: Value) -> Result<Value> {
    let path = args["path"].as_str().unwrap_or(".");
    let profile = args["profile"].as_str().unwrap_or("default");
    let release = args["release"].as_bool().unwrap_or(false);
    
    log::info!("Building component at '{}'", path);
    
    // Execute: golem-cli component build --path <path> --profile <profile> [--release]
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("component")
        .arg("build")
        .arg("--path")
        .arg(path)
        .arg("--profile")
        .arg(profile);
    
    if release {
        cmd.arg("--release");
    }
    
    let output = cmd.output().await
        .context("Failed to execute component build command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(json!({
            "success": true,
            "path": path,
            "profile": profile,
            "release": release,
            "output": stdout.to_string(),
            "message": "Component built successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Component build failed: {}", stderr))
    }
}

/// Component Deploy - Deploys a Golem component
pub async fn component_deploy(args: Value) -> Result<Value> {
    let path = args["path"].as_str().unwrap_or(".");
    let component_name = args["component_name"].as_str();
    let profile = args["profile"].as_str().unwrap_or("default");
    
    log::info!("Deploying component at '{}'", path);
    
    // Execute: golem-cli component deploy --path <path> [--name <name>] --profile <profile>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("component")
        .arg("deploy")
        .arg("--path")
        .arg(path)
        .arg("--profile")
        .arg(profile);
    
    if let Some(name) = component_name {
        cmd.arg("--name").arg(name);
    }
    
    let output = cmd.output().await
        .context("Failed to execute component deploy command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Try to parse component ID from output
        let component_id = extract_component_id(&stdout);
        
        Ok(json!({
            "success": true,
            "path": path,
            "component_name": component_name,
            "component_id": component_id,
            "profile": profile,
            "output": stdout.to_string(),
            "message": "Component deployed successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Component deploy failed: {}", stderr))
    }
}

/// App New - Creates a new Golem application
pub async fn app_new(args: Value) -> Result<Value> {
    let name = args["name"].as_str().context("name is required")?;
    let path = args["path"].as_str().unwrap_or(name);
    
    log::info!("Creating new application '{}'", name);
    
    // Execute: golem-cli app new <name> --path <path>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("app")
        .arg("new")
        .arg(name)
        .arg("--path")
        .arg(path);
    
    let output = cmd.output().await
        .context("Failed to execute app new command")?;
    
    if output.status.success() {
        Ok(json!({
            "success": true,
            "app_name": name,
            "path": path,
            "message": format!("Application '{}' created successfully", name)
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Application creation failed: {}", stderr))
    }
}

/// App Deploy - Deploys a Golem application
pub async fn app_deploy(args: Value) -> Result<Value> {
    let path = args["path"].as_str().unwrap_or(".");
    let app_name = args["app_name"].as_str();
    let profile = args["profile"].as_str().unwrap_or("default");
    
    log::info!("Deploying application at '{}'", path);
    
    // Execute: golem-cli app deploy --path <path> [--name <name>] --profile <profile>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("app")
        .arg("deploy")
        .arg("--path")
        .arg(path)
        .arg("--profile")
        .arg(profile);
    
    if let Some(name) = app_name {
        cmd.arg("--name").arg(name);
    }
    
    let output = cmd.output().await
        .context("Failed to execute app deploy command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let app_id = extract_app_id(&stdout);
        
        Ok(json!({
            "success": true,
            "path": path,
            "app_name": app_name,
            "app_id": app_id,
            "profile": profile,
            "output": stdout.to_string(),
            "message": "Application deployed successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Application deploy failed: {}", stderr))
    }
}

/// Worker Create - Creates a new worker
pub async fn worker_create(args: Value) -> Result<Value> {
    let worker_name = args["worker_name"].as_str().context("worker_name is required")?;
    let component_id = args["component_id"].as_str().context("component_id is required")?;
    let profile = args["profile"].as_str().unwrap_or("default");
    
    log::info!("Creating worker '{}' for component '{}'", worker_name, component_id);
    
    // Execute: golem-cli worker create <name> --component <component_id> --profile <profile>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("worker")
        .arg("create")
        .arg(worker_name)
        .arg("--component")
        .arg(component_id)
        .arg("--profile")
        .arg(profile);
    
    let output = cmd.output().await
        .context("Failed to execute worker create command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let worker_id = extract_worker_id(&stdout);
        
        Ok(json!({
            "success": true,
            "worker_name": worker_name,
            "component_id": component_id,
            "worker_id": worker_id,
            "profile": profile,
            "output": stdout.to_string(),
            "message": "Worker created successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Worker creation failed: {}", stderr))
    }
}

/// Worker Invoke - Invokes a worker function
pub async fn worker_invoke(args: Value) -> Result<Value> {
    let worker_name = args["worker_name"].as_str().context("worker_name is required")?;
    let function_name = args["function_name"].as_str().context("function_name is required")?;
    let params = args["params"].as_array();
    let profile = args["profile"].as_str().unwrap_or("default");
    
    log::info!("Invoking worker '{}' function '{}'", worker_name, function_name);
    
    // Execute: golem-cli worker invoke <name> <function> [params...] --profile <profile>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("worker")
        .arg("invoke")
        .arg(worker_name)
        .arg(function_name)
        .arg("--profile")
        .arg(profile);
    
    if let Some(params) = params {
        for param in params {
            if let Some(param_str) = param.as_str() {
                cmd.arg(param_str);
            }
        }
    }
    
    let output = cmd.output().await
        .context("Failed to execute worker invoke command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        Ok(json!({
            "success": true,
            "worker_name": worker_name,
            "function_name": function_name,
            "output": stdout.to_string(),
            "profile": profile,
            "message": "Worker invoked successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Worker invocation failed: {}", stderr))
    }
}

/// Get Status - Gets the status of Golem CLI and connected services
pub async fn get_status(args: Value) -> Result<Value> {
    let profile = args["profile"].as_str().unwrap_or("default");
    
    log::info!("Getting status for profile '{}'", profile);
    
    // Execute: golem-cli diagnose --profile <profile>
    let mut cmd = TokioCommand::new("golem-cli");
    cmd.arg("diagnose")
        .arg("--profile")
        .arg(profile);
    
    let output = cmd.output().await
        .context("Failed to execute diagnose command")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse status information
        let status_info = parse_diagnose_output(&stdout);
        
        Ok(json!({
            "success": true,
            "profile": profile,
            "status": status_info,
            "message": "Status retrieved successfully"
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Status check failed: {}", stderr))
    }
}

// Helper functions

fn extract_component_id(output: &str) -> Option<String> {
    // Try to extract component ID from output
    // Format: "Component ID: xxx" or "Created component: xxx"
    for line in output.lines() {
        if line.contains("Component ID:") {
            return Some(line.split("Component ID:").nth(1)?.trim().to_string());
        }
        if line.contains("Created component:") {
            return Some(line.split("Created component:").nth(1)?.trim().to_string());
        }
    }
    None
}

fn extract_app_id(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("Application ID:") {
            return Some(line.split("Application ID:").nth(1)?.trim().to_string());
        }
        if line.contains("Created application:") {
            return Some(line.split("Created application:").nth(1)?.trim().to_string());
        }
    }
    None
}

fn extract_worker_id(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("Worker ID:") {
            return Some(line.split("Worker ID:").nth(1)?.trim().to_string());
        }
        if line.contains("Created worker:") {
            return Some(line.split("Created worker:").nth(1)?.trim().to_string());
        }
    }
    None
}

fn parse_diagnose_output(output: &str) -> Value {
    let mut status = json!({
        "cli_version": "unknown",
        "profile": "unknown",
        "connection": "unknown",
        "components": 0,
        "applications": 0,
        "workers": 0
    });
    
    for line in output.lines() {
        if line.contains("Version:") {
            status["cli_version"] = json!(line.split(":").nth(1)?.trim());
        }
        if line.contains("Profile:") {
            status["profile"] = json!(line.split(":").nth(1)?.trim());
        }
        if line.contains("Connection:") {
            status["connection"] = json!(line.split(":").nth(1)?.trim());
        }
    }
    
    status
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_component_id() {
        let output = "Deploying component...\nComponent ID: abc-123-def\nSuccess!";
        assert_eq!(extract_component_id(output), Some("abc-123-def".to_string()));
    }
    
    #[test]
    fn test_extract_worker_id() {
        let output = "Creating worker...\nCreated worker: worker-xyz\nDone!";
        assert_eq!(extract_worker_id(output), Some("worker-xyz".to_string()));
    }
    
    #[test]
    fn test_parse_diagnose_output() {
        let output = "Version: 1.2.3\nProfile: default\nConnection: OK";
        let status = parse_diagnose_output(output);
        assert_eq!(status["cli_version"], "1.2.3");
        assert_eq!(status["profile"], "default");
        assert_eq!(status["connection"], "OK");
    }
}
