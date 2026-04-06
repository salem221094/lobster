use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    pub args: Option<HashMap<String, WorkflowArg>>,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowArg {
    pub default: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowStep {
    pub id: String,
    pub run: Option<String>,
    pub command: Option<String>,
    pub pipeline: Option<String>,
    pub approval: Option<serde_json::Value>,
    pub stdin: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub when: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub engine: EngineSettings,
    pub mcp: McpSettings,
    pub telemetry: TelemetrySettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineSettings {
    pub workflows_dir: String,
    pub state_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpSettings {
    pub endpoints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetrySettings {
    pub otlp_endpoint: String,
    pub service_name: String,
}
