use crate::models::Settings;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn load_settings<P: AsRef<Path>>(path: P) -> Result<Settings> {
    let content = fs::read_to_string(path)?;
    let settings: Settings = toml::from_str(&content)?;
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_settings() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"
[engine]
workflows_dir = "./workflows"
state_dir = "./state"

[mcp]
endpoints = ["http://localhost:8080"]

[telemetry]
otlp_endpoint = "http://localhost:4317"
service_name = "lobster-rust"
"#).unwrap();

        let settings = load_settings(file.path()).unwrap();
        assert_eq!(settings.engine.workflows_dir, "./workflows");
        assert_eq!(settings.mcp.endpoints[0], "http://localhost:8080");
        assert_eq!(settings.telemetry.service_name, "lobster-rust");
    }
}
