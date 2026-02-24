package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"time"

	"go.opentelemetry.io/otel/exporters/otlp/otlplog/otlploghttp"
	otellog "go.opentelemetry.io/otel/log"
	sdklog "go.opentelemetry.io/otel/sdk/log"
	"go.opentelemetry.io/otel/sdk/resource"
	semconv "go.opentelemetry.io/otel/semconv/v1.26.0"
)

func main() {
	ctx := context.Background()
	exporter, err := otlploghttp.New(ctx,
		otlploghttp.WithEndpointURL(os.Getenv("LOGSBLOX_ENDPOINT")),
		otlploghttp.WithHeaders(map[string]string{"x-api-key": os.Getenv("LOGSBLOX_API_KEY")}),
		otlploghttp.WithCompression(otlploghttp.GzipCompression),
		otlploghttp.WithHTTPClient(&http.Client{Timeout: 15 * time.Second}),
		otlploghttp.WithRetry(otlploghttp.RetryConfig{Enabled: false}),
	)
	if err != nil {
		log.Fatal(err)
	}

	res, _ := resource.Merge(resource.Default(),
		resource.NewWithAttributes(semconv.SchemaURL,
			semconv.DeploymentEnvironment("production"),
			semconv.ServiceName("go-service"),
		))
	provider := sdklog.NewLoggerProvider(
		sdklog.WithProcessor(sdklog.NewSimpleProcessor(exporter)),
		sdklog.WithResource(res),
	)
	defer provider.Shutdown(ctx)

	var r otellog.Record
	r.SetSeverity(otellog.SeverityInfo)
	r.SetSeverityText("INFO")
	r.SetBody(otellog.StringValue("Welcome to Logsblox!"))
	provider.Logger("logsblox-logger").Emit(ctx, r)
}
