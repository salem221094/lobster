use crate::models::{Workflow, WorkflowStep};
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use tracing::{info, span, Level};

pub struct ExecutionContext {
    pub args: HashMap<String, serde_json::Value>,
    pub env: HashMap<String, String>,
}

pub async fn run_workflow(workflow: &Workflow, ctx: &ExecutionContext) -> Result<()> {
    let mut results: HashMap<String, StepResult> = HashMap::new();

    for step in &workflow.steps {
        let span = span!(Level::INFO, "workflow.step",
            "lobster.step_name" = step.id.as_str(),
            "lobster.tool_id" = workflow.name.as_str()
        );
        let _enter = span.enter();

        info!("Executing step: {}", step.id);
        let result = execute_step(step, ctx, &results).await?;
        results.insert(step.id.clone(), result);
    }

    Ok(())
}

#[derive(Debug)]
pub struct StepResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

async fn execute_step(
    step: &WorkflowStep,
    ctx: &ExecutionContext,
    _previous_results: &HashMap<String, StepResult>
) -> Result<StepResult> {
    if let Some(cmd_str) = step.run.as_ref().or(step.command.as_ref()) {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .envs(&ctx.env)
            .spawn()
            .context("Failed to spawn shell command")?;

        if let Some(stdin_content) = &step.stdin {
             let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
             child_stdin.write_all(stdin_content.as_bytes()).await?;
             child_stdin.flush().await?;
        }

        let output = child.wait_with_output().await?;

        Ok(StepResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status.code().unwrap_or(-1),
        })
    } else {
        // Handle pipeline/approval steps (simplified for now)
        info!("Skipping non-shell step for now: {}", step.id);
        Ok(StepResult { stdout: "".to_string(), stderr: "".to_string(), status: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Workflow, WorkflowStep};

    #[tokio::test]
    async fn test_run_workflow_simple() {
        let workflow = Workflow {
            name: "test".to_string(),
            description: None,
            args: None,
            steps: vec![
                WorkflowStep {
                    id: "step1".to_string(),
                    run: Some("echo 'hello'".to_string()),
                    command: None,
                    pipeline: None,
                    approval: None,
                    stdin: None,
                    env: None,
                    when: None,
                    condition: None,
                }
            ],
        };
        let ctx = ExecutionContext {
            args: HashMap::new(),
            env: HashMap::new(),
        };
        let result = run_workflow(&workflow, &ctx).await;
        assert!(result.is_ok());
    }
}
