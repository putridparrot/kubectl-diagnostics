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

## Config Map Diagnostics

TODO:

```aiignore
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-config
data:
  LOG_LEVEL: "info"
  LOG_LEVEL: "debug"  # This overwrites the previous value
```

- Information: Check if keys are missing

## Deployments Diagnostics

TODO:

## Event Diagnostics

TODO:

## Ingress Diagnostics

TODO:

## Node Diagnostics

TODO:

## Pod Diagnostics

TODO:

## Resources Diagnostics

TODO:

## Secrets Diagnostics

TODO:

## Service Diagnostics

TODO:

