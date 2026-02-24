# Go OTel SDK (Logs)

## Run with Docker

```bash
docker build -t go-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  go-otel-sdk
```

## Run locally

```bash
go mod tidy
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
go run .
```
