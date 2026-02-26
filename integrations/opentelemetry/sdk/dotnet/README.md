# .NET OTel SDK (Logs)

## Run with Docker

```bash
docker build -t dotnet-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  dotnet-otel-sdk
```

## Run locally

```bash
dotnet restore
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
dotnet run --project logsblox-otel-sdk-dotnet.csproj
```
