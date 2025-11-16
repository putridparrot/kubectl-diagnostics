# kubectl-diagnostics

### NOTE: This is work in progress. This README is just an outline of potential features.

kube-diagnostics is aimed at helping troubleshoot Kubernetes clusters and applications hosted within it. It can be deployed as a 
kubectl plugin, or run using it's executable name. 

## Usage

Subcommands:

- configmaps: Show configmap status
- deployments: Show deployment status
- events: Show event status
- ingress: Show ingress status
- nodes: Show node status
- pods: Show pod status and container errors
- resources: Show resource status
- secrets: Show secret status
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

- Check if rollout failed
- Check is has unavailble replicas

## Event Diagnostics

TODO:

## Ingress Diagnostics

- Check if loadbalancer is ready
- Check if ingress has endpoints
- Check if ingress is healthy

## Node Diagnostics

- Check if nodes are ready

## Pod Diagnostics

- Check pod phase
- Check pod status
- Check pod restarts

## Resources Diagnostics

- Check if pod containers has resource requests and limits
- Check if pod containers has limits less than requests
- Check if pod containers was OOMKilled
- Check Memory, Disk and PID pressures

## Secrets Diagnostics

- Check if key is empty
- Check if key is unusually large
- Check if secret has no data
- check if secret is unused by any pods in the namespace

## Service Diagnostics

- Check if endpoints are missing
