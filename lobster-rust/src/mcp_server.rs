use crate::models::{Settings};
use crate::parser::parse_workflow_file;
use crate::engine::{run_workflow, ExecutionContext};
use std::collections::HashMap;
use std::path::PathBuf;
use glob::glob;

pub struct LobsterMcpHandler {
    pub workflows_dir: PathBuf,
}

impl LobsterMcpHandler {
    pub fn new(workflows_dir: PathBuf) -> Self {
        Self { workflows_dir }
    }

    pub async fn list_tools(&self) -> Vec<String> {
        let mut tools = Vec::new();
        let pattern = format!("{}/**/*", self.workflows_dir.display());
        for entry in glob(&pattern).unwrap() {
            if let Ok(path) = entry {
                if path.is_file() {
                    if let Ok(workflow) = parse_workflow_file(&path) {
                        tools.push(workflow.name);
                    }
                }
            }
        }
        tools
    }

    pub async fn run_tool(&self, name: &str) -> anyhow::Result<()> {
        let pattern = format!("{}/**/*", self.workflows_dir.display());
        for entry in glob(&pattern).unwrap() {
            if let Ok(path) = entry {
                if path.is_file() {
                    if let Ok(workflow) = parse_workflow_file(&path) {
                        if workflow.name == name {
                            let ctx = ExecutionContext {
                                args: HashMap::new(),
                                env: HashMap::new(),
                            };
                            return run_workflow(&workflow, &ctx).await;
                        }
                    }
                }
            }
        }
        anyhow::bail!("Workflow not found: {}", name)
    }
}

pub async fn run_mcp_server(settings: Settings) -> anyhow::Result<()> {
    println!("Lobster MCP Server simulation starting...");
    let handler = LobsterMcpHandler::new(PathBuf::from(settings.engine.workflows_dir));
    let tools = handler.list_tools().await;
    println!("Discovered tools: {:?}", tools);
    Ok(())
}
