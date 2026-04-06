use crate::models::Workflow;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn parse_workflow_file<P: AsRef<Path>>(path: P) -> Result<Workflow> {
    let content = fs::read_to_string(path.as_ref())?;
    let ext = path.as_ref().extension().and_then(|s| s.to_str()).unwrap_or("");

    if ext == "md" {
        parse_markdown_workflow(&content)
    } else {
        parse_yaml_workflow(&content)
    }
}

fn parse_yaml_workflow(content: &str) -> Result<Workflow> {
    serde_yaml::from_str(content).context("Failed to parse YAML workflow")
}

fn parse_markdown_workflow(content: &str) -> Result<Workflow> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n").unwrap();
    if let Some(caps) = re.captures(content) {
        let yaml_frontmatter = caps.get(1).map_or("", |m| m.as_str());
        parse_yaml_workflow(yaml_frontmatter)
    } else {
        anyhow::bail!("No YAML frontmatter found in markdown file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml_workflow() {
        let yaml = r#"
name: test-workflow
steps:
  - id: step1
    run: echo "hello"
"#;
        let workflow = parse_yaml_workflow(yaml).unwrap();
        assert_eq!(workflow.name, "test-workflow");
        assert_eq!(workflow.steps[0].id, "step1");
        assert_eq!(workflow.steps[0].run, Some("echo \"hello\"".to_string()));
    }

    #[test]
    fn test_parse_markdown_workflow() {
        let md = r#"---
name: md-workflow
steps:
  - id: step1
    run: |
      echo "hello"
      echo "world"
---
# Documentation
This is a test workflow in markdown.
"#;
        let workflow = parse_markdown_workflow(md).unwrap();
        assert_eq!(workflow.name, "md-workflow");
        assert_eq!(workflow.steps[0].id, "step1");
        assert!(workflow.steps[0].run.as_ref().unwrap().contains("world"));
    }
}
