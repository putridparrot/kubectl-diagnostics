# kubectl-diagnostics

### NOTE: This is work in progress. This README is just an outline of potential features.

kube-diagnostics is aimed at helping troubleshoot Kubernetes clusters and applications hosted within it. It can be deployed as a 
kubectl plugin, or run using it's executable name. 

## Usage

Subcommands:

- events: Show sorted events
- nodes: Show node status
- pods: Show pod status and container errors
- services: Show service status
- all: Run all subcommands

Each subcommand comes with its own arguments and flags.

Examples:

```aiignore
diagnostics pods --namespace default
diagnostics nodes
diagnostics events --namespace dev
diagnostics services --namespace stg

diagnostics all --namespace tst
```

## Event Diagnostics

TODO:

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

## Node Diagnostics

## Pod Diagnostics

TODO:

kubectl get pods -n my-ns -o json

Filter pods by:
- status.phase != Running
- containerStatuses[].state.waiting.reason
- restartCount > 3

Parse JSON and extract:
- CrashLoopBackOff
- ImagePullBackOff
- Readiness probe failures

## Service Diagnostics

 
## Azure Devops Integration

- Save report to diagnostics.md or diagnostics.json
- Upload with PublishBuildArtifacts
- Optionally emit ##[error] or ##[warning] for pipeline annotations

