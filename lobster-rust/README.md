# Lobster Rust Engine

High-performance, hard-deterministic workflow execution engine for OpenClaw Lobster, written in Rust.

## Features

- **Hard Determinism**: Built for AEC and Biotech document pipelines.
- **Low Footprint**: <4MB binary, <10MB idle RAM, ~4ms startup.
- **MCP Native**: Acts as an MCP server mapping workflows to tools.
- **OpenTelemetry**: Integrated OTLP tracing for every workflow step.
- **Markdown Support**: Define workflows inside documentation with YAML frontmatter.

## Quick Start

1.  **Build**: `cargo build --release`
2.  **Configure**: Create `settings.toml`.
3.  **Run**: `./target/release/lobster-rust run --file my_workflow.yaml`
4.  **MCP**: `./target/release/lobster-rust mcp`

## Development

- `cargo test`: Run test suite.
- `cargo check`: Fast syntax and type check.
