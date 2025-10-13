# kubectl-diagnostics

### NOTE: This is work in progress. This README is just an outline of potential features.

kube-diagnosis --namespace my-ns

Subcommands:

- events: Show sorted events
- pods: Show pod status and container errors
- describe: Describe specific resources
- report: Save findings to file

## Pod Filtering

kubectl get pods -n my-ns -o json

Filter pods by:
- status.phase != Running
- containerStatuses[].state.waiting.reason
- restartCount > 3

Parse JSON and extract:
- CrashLoopBackOff
- ImagePullBackOff
- Readiness probe failures

## Event Correlation

kubectl get events -n my-ns --sort-by='.lastTimestamp'

Highlight:
- Failed scheduling
- Probe failures
- Resource quota issues

Output Formats:
- Console (colorized), for local dev
- Markdown for CI logs
- JSON for pipeline parsing
- File artifact for Azure DevOps upload
 
## Azure Devops Integration

- Save report to diagnostics.md or diagnostics.json
- Upload with PublishBuildArtifacts
- Optionally emit ##[error] or ##[warning] for pipeline annotations

