# Rust OTel SDK (Logs)

## Run with Docker

```bash
docker build -t rust-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  rust-otel-sdk
```

## Run locally

```bash
cargo build --release
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
cargo run --release
```
