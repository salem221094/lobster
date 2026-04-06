pub mod models;
pub mod config;
pub mod parser;
pub mod engine;
pub mod telemetry;
pub mod mcp_server;

use clap::{Parser, Subcommand};
use crate::config::load_settings;
use crate::parser::parse_workflow_file;
use crate::engine::{run_workflow, ExecutionContext};
use crate::telemetry::{init_telemetry, shutdown_telemetry};
use crate::mcp_server::run_mcp_server;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lobster-rust")]
#[command(about = "High-performance Lobster execution engine", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "settings.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Run a workflow file
    Run {
        /// Workflow file path
        file: PathBuf,
        /// JSON arguments for the workflow
        #[arg(long)]
        args: Option<String>,
    },
    /// Start the MCP server
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let settings = load_settings(&cli.config)?;

    // Initialize telemetry
    init_telemetry(&settings.telemetry.otlp_endpoint, &settings.telemetry.service_name)?;

    match &cli.command {
        Some(Commands::Run { file, args }) => {
            let workflow = parse_workflow_file(file)?;
            let args_map: HashMap<String, serde_json::Value> = if let Some(args_str) = args {
                serde_json::from_str(args_str)?
            } else {
                HashMap::new()
            };
            let ctx = ExecutionContext {
                args: args_map,
                env: HashMap::new(),
            };
            run_workflow(&workflow, &ctx).await?;
        }
        Some(Commands::Mcp) => {
            run_mcp_server(settings).await?;
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }

    shutdown_telemetry();
    Ok(())
}
