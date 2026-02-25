# Python OTel SDK (Logs)

## Run with Docker

```bash
docker build -t python-otel-sdk .
docker run --rm \
  -e LOGSBLOX_ENDPOINT \
  -e LOGSBLOX_API_KEY \
  python-otel-sdk
```

## Run locally

```bash
pip install -r requirements.txt
LOGSBLOX_ENDPOINT="YOUR_LOGSBLOX_ENDPOINT" \
LOGSBLOX_API_KEY="YOUR_LOGSBLOX_API_KEY" \
python main.py
```
