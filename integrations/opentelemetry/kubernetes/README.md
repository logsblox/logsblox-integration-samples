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
   Replace `LOGSBLOX_ENDPOINT` and `YOUR_API_KEY` with Logsblox endpoint and API key from the Logsblox dashboard. The collector will use this to authenticate when sending logs.
   ```sh
   kubectl create secret generic otel-collector-secrets \
     --from-literal=LOGSBLOX_ENDPOINT=LOGSBLOX_ENDPOINT \
     --from-literal=LOGSBLOX_API_KEY=YOUR_API_KEY
   ```

3. Download the OpenTelemetry Collector values file  
   Download the example Helm values file from the Logsblox integration samples repository. Update `LOGSBLOX_ENDPOINT` and `YOUR_API_KEY` with your actual values before installing.
   ```sh
   curl -O https://raw.githubusercontent.com/logsblox/logsblox-integration-samples/refs/tags/v0.1.0/integrations/opentelemetry/kubernetes/opentelemetry-kube-stack-values.yaml
   ```

4. Install the OpenTelemetry Collector stack  
   Deploy the collector using the values file you downloaded and configured in the previous step.
   ```sh
   helm upgrade --install opentelemetry-kube-stack \
     open-telemetry/opentelemetry-kube-stack \
     -f opentelemetry-kube-stack-values.yaml
   ```