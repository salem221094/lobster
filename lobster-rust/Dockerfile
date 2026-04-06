# Build Stage
FROM rust:1-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/lobster-rust /usr/local/bin/lobster-rust
COPY settings.toml .

# Create directory for workflows
RUN mkdir workflows state

VOLUME ["/app/workflows", "/app/state"]

ENTRYPOINT ["lobster-rust"]
CMD ["mcp"]
