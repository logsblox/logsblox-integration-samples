## Installation

OpenTelemetry Collector is a data collection agent that exports logs from your Kubernetes cluster without instrumenting each service. It scrapes logs from pods, nodes, and system components, then forwards them to Logsblox.

Install the OpenTelemetry Collector with Helm:

1. Add the OpenTelemetry Helm chart repository  
   Register the official OpenTelemetry Helm charts to install the OpenTelemetry Collector.
   ```sh
   helm repo add open-telemetry 'https://open-telemetry.github.io/opentelemetry-helm-charts' --force-update
   helm repo update
   ```

2. Create a Kubernetes secret with Logsblox endpoint and API key  
   Replace `LOGSBLOX_ENDPOINT` and `YOUR_API_KEY` with values from the Logsblox dashboard. The collector uses this to authenticate when sending logs.
   ```sh
   kubectl create secret generic otel-collector-secrets \
     --from-literal=LOGSBLOX_ENDPOINT=LOGSBLOX_ENDPOINT \
     --from-literal=LOGSBLOX_API_KEY=YOUR_API_KEY
   ```

3. Install the OpenTelemetry Collector stack  
   Deploy the collector using your values file. Download the example values file `opentelemetry-kube-stack-values.yaml`, update `LOGSBLOX_ENDPOINT` and `YOUR_API_KEY`, then run:
   ```sh
   helm upgrade --install opentelemetry-kube-stack \
     open-telemetry/opentelemetry-kube-stack \
     -f opentelemetry-kube-stack-values.yaml
   ```