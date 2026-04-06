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

## Standalone Productization

Lobster Rust is designed for standalone deployment in critical document pipelines:

- **Zero-Footprint Deployment**: Use the provided `Dockerfile` for a minimal, secure container environment.
- **Workflow Automation**: Utilize `Justfile` for standard operational tasks (`just build`, `just mcp`, `just test`).
- **Telemetry-First Operations**: Native OTLP support allows Lobster to plug directly into enterprise observability stacks (Honeycomb, Datadog, New Relic).
- **Hard Determinism**: Ideal for AEC (Architecture, Engineering, Construction) and Biotech where execution consistency and auditability are non-negotiable.

### Example Workflows
Check the \`examples/\` directory for production-ready templates.
