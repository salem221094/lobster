# Migration Guide: TypeScript Lobster to Rust Lobster

This guide explains how to migrate your existing Lobster workflows to the new Rust-powered engine.

## Configuration

1.  **Global Settings**: Create a `settings.toml` file.
    ```toml
    [engine]
    workflows_dir = "./workflows"
    state_dir = "./state"

    [mcp]
    endpoints = []

    [telemetry]
    otlp_endpoint = "http://localhost:4317"
    service_name = "lobster-rust"
    ```

2.  **Environment Variables**: The Rust engine still respects most LOBSTER_* environment variables for telemetry and configuration.

## Workflows

1.  **YAML Files**: Most existing YAML workflows should work as-is.
2.  **Markdown Files**: The Rust engine supports Markdown files with YAML frontmatter.
    ```markdown
    ---
    name: my-workflow
    steps:
      - id: hello
        run: echo "Hello World"
    ---
    # My Workflow Documentation
    ```
3.  **Multi-line Scripts**: Use the YAML literal block scalar `|` for clean multi-line scripts.
    ```yaml
    steps:
      - id: multi-line
        run: |
          echo "Step 1"
          echo "Step 2"
    ```

## Execution

Replace `node bin/lobster.js` with the new binary:

```bash
# Run a specific workflow file
./lobster-rust run --file workflows/my-workflow.yaml --args '{"key": "value"}'

# Start in MCP server mode
./lobster-rust mcp
```

## Performance

The new Rust binary is significantly faster and uses less memory than the TypeScript version.
- **Startup**: ~4ms
- **Memory**: <10MB idle
