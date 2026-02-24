const { logs, SeverityNumber } = require("@opentelemetry/api-logs");
const { LoggerProvider, SimpleLogRecordProcessor } = require("@opentelemetry/sdk-logs");
const { OTLPLogExporter } = require("@opentelemetry/exporter-logs-otlp-proto");
const { resourceFromAttributes } = require("@opentelemetry/resources");

async function main() {
  const exporter = new OTLPLogExporter({
    url: process.env.LOGSBLOX_ENDPOINT,
    headers: { "x-api-key": process.env.LOGSBLOX_API_KEY },
    timeoutMillis: 15000,
  });

  const provider = new LoggerProvider({
    resource: resourceFromAttributes({
      "deployment.environment": "production",
      "service.name": "node-service",
    }),
    processors: [new SimpleLogRecordProcessor(exporter)],
  });
  logs.setGlobalLoggerProvider(provider);

  const logger = logs.getLogger("logsblox-logger");
  logger.emit({
    severityNumber: SeverityNumber.INFO,
    severityText: "INFO",
    body: "Welcome to Logsblox!",
  });

  await provider.shutdown();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
