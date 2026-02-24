# Node.js OTel SDK (Logs)

## Run with Docker

```bash
docker build -t node-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  node-otel-sdk
```

## Run locally

```bash
npm install
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
npm start
```
