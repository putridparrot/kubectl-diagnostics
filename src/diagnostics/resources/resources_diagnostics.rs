use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::{Api, Client, ResourceExt};
use std::str::FromStr;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

pub struct ResourcesDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> ResourcesDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct ResourcesDiagnosticReport {
    pub meta: DiagnosticReport
}

impl ResourcesDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Resources Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for ResourcesDiagnostic<'a> {
    type Report = ResourcesDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<ResourcesDiagnosticReport> {

        let api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let list = api.list(&Default::default()).await?;

        let mut issues = vec![];
        let items = list.items;
        let count = items.len();

        for item in items {
            let pod_name = item.name_any();
            let spec = match &item.spec {
                Some(s) => s,
                None => continue,
            };

            for container in &spec.containers {
                let cname = &container.name;
                let resources = container.resources.as_ref();

                let requests = resources.unwrap().requests.as_ref();
                let limits = resources.unwrap().limits.as_ref();

                if requests.is_none() {
                    issues.push(DiagnosticIssue::new(
                        "Resources".to_string(),
                        format!("Pod `{}` container `{}` has no resource requests", pod_name, cname),
                        Severity::Warning,
                    ));
                }

                if limits.is_none() {
                    issues.push(DiagnosticIssue::new(
                        "Resources".to_string(),
                        format!("Pod `{}` container `{}` has no resource limits", pod_name, cname),
                        Severity::Warning,
                    ));
                }

                if let (Some(req), Some(lim)) = (requests, limits) {
                    for (key, req_val) in req.iter() {
                        if let Some(lim_val) = lim.get(key) {
                            let lhs = parse_bytes(lim_val).unwrap();
                            let rhs = parse_bytes(req_val).unwrap();
                            if lhs < rhs {
                                issues.push(DiagnosticIssue::new(
                                    "Resources".to_string(),
                                    format!("Pod `{}` container `{}` has limit < request for `{}`", pod_name, cname, key),
                                    Severity::Error,
                                ));
                            }
                        }
                    }
                }
            }

            if let Some(status) = &item.status {
                if let Some(statuses) = &status.container_statuses {
                    for cs in statuses {
                        if let Some(last_state) = &cs.last_state {
                            if let Some(term) = &last_state.terminated {
                                if term.reason.as_deref() == Some("OOMKilled") {
                                    issues.push(DiagnosticIssue::new(
                                        "Resources".to_string(),
                                        format!("Pod `{}` container `{}` was OOMKilled", pod_name, cs.name),
                                        Severity::Error,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        let node_api: Api<Node> = Api::all(self.client.clone());
        let nodes = node_api.list(&Default::default()).await?;
        for node in &nodes {
            let node_name = node.name_any();
            if let Some(status) = &node.status {
                if let Some(conditions) = &status.conditions {
                    for cond in conditions {
                        if cond.status == "True" && matches!(cond.type_.as_str(), "MemoryPressure" | "DiskPressure" | "PIDPressure") {
                            issues.push(DiagnosticIssue::new(
                                "Resources".to_string(),
                                format!("Node `{}` is under `{}`", node_name, cond.type_),
                                Severity::Warning,
                            ));
                        }
                    }
                }
            }
        }


        Ok(ResourcesDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} resources analyzed", count),
            issues,
        }})
    }
}

/// Parses a Kubernetes-style quantity string into bytes as u64.
/// Supports suffixes: Ki, Mi, Gi, Ti, Pi, Ei (binary), and assumes raw bytes if no suffix.
pub fn parse_bytes(q: &Quantity) -> Option<u64> {
    let s = q.0.as_str();

    let (num_str, multiplier) = if let Some(stripped) = s.strip_suffix("Ki") {
        (stripped, 1024u64)
    } else if let Some(stripped) = s.strip_suffix("Mi") {
        (stripped, 1024u64.pow(2))
    } else if let Some(stripped) = s.strip_suffix("Gi") {
        (stripped, 1024u64.pow(3))
    } else if let Some(stripped) = s.strip_suffix("Ti") {
        (stripped, 1024u64.pow(4))
    } else if let Some(stripped) = s.strip_suffix("Pi") {
        (stripped, 1024u64.pow(5))
    } else if let Some(stripped) = s.strip_suffix("Ei") {
        (stripped, 1024u64.pow(6))
    } else if let Some(stripped) = s.strip_suffix('m') {
        // "m" is typically used for CPU (millicores), not memory
        // But if you want to treat it as 1/1000 of a byte (unusual), handle here
        return stripped.parse::<f64>().ok().map(|v| (v * 0.001).round() as u64);
    } else {
        (s, 1)
    };

    u64::from_str(num_str.trim()).ok().map(|v| v * multiplier)
}
