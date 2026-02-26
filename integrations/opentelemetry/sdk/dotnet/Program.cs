using System;
using System.Collections.Generic;
using Microsoft.Extensions.Logging;
using OpenTelemetry.Exporter;
using OpenTelemetry.Logs;
using OpenTelemetry.Resources;

Environment.SetEnvironmentVariable("OTEL_EXPORTER_OTLP_COMPRESSION", "gzip");
Environment.SetEnvironmentVariable("OTEL_EXPORTER_OTLP_LOGS_COMPRESSION", "gzip");

using var loggerFactory = LoggerFactory.Create(builder =>
{
    builder.AddOpenTelemetry(options =>
    {
        options.SetResourceBuilder(
            ResourceBuilder.CreateDefault()
                .AddService("dotnet-service")
                .AddAttributes(new List<KeyValuePair<string, object>>
                {
                    new("deployment.environment", "production"),
                })
        );

        options.AddOtlpExporter(otlpOptions =>
        {
            otlpOptions.Endpoint = new Uri(Environment.GetEnvironmentVariable("LOGSBLOX_ENDPOINT") ?? "");
            otlpOptions.Protocol = OtlpExportProtocol.HttpProtobuf;
            otlpOptions.Headers = $"x-api-key={Environment.GetEnvironmentVariable("LOGSBLOX_API_KEY") ?? ""}";
            otlpOptions.TimeoutMilliseconds = 15000;
        });
    });
});

var logger = loggerFactory.CreateLogger("logsblox-logger");
logger.LogInformation("Welcome to Logsblox!");
