# Java OTel SDK (Logs)

## Run with Docker

```bash
docker build -t java-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  java-otel-sdk
```

## Run locally

```bash
mvn -q -DskipTests package
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
java -jar target/java-otel-sdk.jar
```
