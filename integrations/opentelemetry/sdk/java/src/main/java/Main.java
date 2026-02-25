import io.opentelemetry.api.common.AttributeKey;
import io.opentelemetry.api.logs.Logger;
import io.opentelemetry.api.logs.Severity;
import io.opentelemetry.exporter.otlp.http.logs.OtlpHttpLogRecordExporter;
import io.opentelemetry.sdk.logs.SdkLoggerProvider;
import io.opentelemetry.sdk.logs.export.SimpleLogRecordProcessor;
import io.opentelemetry.sdk.resources.Resource;
import java.time.Duration;
import java.util.concurrent.TimeUnit;

public class Main {
  public static void main(String[] args) {
    OtlpHttpLogRecordExporter exporter = OtlpHttpLogRecordExporter.builder()
        .setEndpoint(System.getenv("LOGSBLOX_ENDPOINT"))
        .addHeader("x-api-key", System.getenv("LOGSBLOX_API_KEY"))
        .setCompression("gzip")
        .setTimeout(Duration.ofSeconds(15))
        .build();

    Resource resource = Resource.getDefault().toBuilder()
        .put(AttributeKey.stringKey("deployment.environment"), "production")
        .put(AttributeKey.stringKey("service.name"), "java-service")
        .build();

    SdkLoggerProvider loggerProvider = SdkLoggerProvider.builder()
        .setResource(resource)
        .addLogRecordProcessor(SimpleLogRecordProcessor.create(exporter))
        .build();

    Logger logger = loggerProvider.loggerBuilder("logsblox-logger").build();
    logger.logRecordBuilder()
        .setSeverity(Severity.INFO)
        .setSeverityText("INFO")
        .setBody("Welcome to Logsblox!")
        .emit();

    loggerProvider.shutdown().join(10, TimeUnit.SECONDS);
  }
}
