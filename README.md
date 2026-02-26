# OpenTelemetry Integration Samples

Minimal OpenTelemetry SDK examples for sending OTLP logs to [Logsblox](https://logsblox.com). Each sample emits a simple INFO log over OTLP HTTP protobuf with resource attributes (`service.name`, `deployment.environment`) for filtering and indexing.

## Integrations

| Language | Path | Description |
|----------|------|-------------|
| Go       | [sdk/go](./integrations/opentelemetry/sdk/go) | OTLP HTTP protobuf logs via the Go OpenTelemetry SDK |
| .NET     | [sdk/dotnet](./integrations/opentelemetry/sdk/dotnet) | OTLP HTTP protobuf logs via the .NET OpenTelemetry SDK |
| Java     | [sdk/java](./integrations/opentelemetry/sdk/java) | OTLP HTTP protobuf logs via the Java OpenTelemetry SDK |
| Node.js  | [sdk/node](./integrations/opentelemetry/sdk/node) | OTLP HTTP protobuf logs via the Node.js OpenTelemetry SDK |
| Python   | [sdk/python](./integrations/opentelemetry/sdk/python) | OTLP HTTP protobuf logs via the Python OpenTelemetry SDK |
| Rust     | [sdk/rust](./integrations/opentelemetry/sdk/rust) | OTLP HTTP protobuf logs via the Rust OpenTelemetry SDK |

## Requirements

- `LOGSBLOX_ENDPOINT` – Logsblox ingest URL
- `LOGSBLOX_API_KEY` – API key for authentication

## Workflow

- OTLP HTTP protobuf log export with gzip compression
- `x-api-key` header for authentication
- Resource attributes: `service.name`, `deployment.environment`
- Emits one INFO log

Each integration includes a `Dockerfile` for easy runs. See the integration-specific README for usage.
