import os

from opentelemetry._logs import SeverityNumber, set_logger_provider
from opentelemetry.exporter.otlp.proto.http import Compression
from opentelemetry.exporter.otlp.proto.http._log_exporter import OTLPLogExporter
from opentelemetry.sdk._logs import LoggerProvider
from opentelemetry.sdk._logs.export import BatchLogRecordProcessor
from opentelemetry.sdk.resources import Resource

provider = LoggerProvider(
    resource=Resource.create(
        {
            "deployment.environment": "production",
            "service.name": "python-service",
        }
    )
)
set_logger_provider(provider)

exporter = OTLPLogExporter(
    endpoint=os.getenv("LOGSBLOX_ENDPOINT", ""),
    headers={"x-api-key": os.getenv("LOGSBLOX_API_KEY", "")},
    compression=Compression.Gzip,
    timeout=15,
)
provider.add_log_record_processor(BatchLogRecordProcessor(exporter))

logger = provider.get_logger("logsblox-logger")
logger.emit(
    severity_text="INFO",
    severity_number=SeverityNumber.INFO,
    body="Welcome to Logsblox!",
)

provider.shutdown()
