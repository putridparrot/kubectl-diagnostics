use colored::Colorize;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
use kube::api::ListParams;
use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};

pub struct PodsDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> PodsDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct PodsDiagnosticReport {
    pub meta: DiagnosticReport
}

impl PodsDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Pod Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for PodsDiagnostic<'a> {
    type Report = PodsDiagnosticReport;
    
    async fn generate_report(&self) -> anyhow::Result<PodsDiagnosticReport> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let lp = ListParams::default().limit(100);
        let pod_list = pods.list(&lp).await?;

        let mut issues = vec![];
        let items = pod_list.items;
        let count = items.len();

        for pod in items {
            let name = pod.metadata.name.unwrap_or_default();
            if let Some(status) = pod.status {
                if let Some(phase) = status.phase {
                    if phase != "Running" {
                        issues.push(DiagnosticIssue::new(
                            name.clone(),
                            format!("Phase: {}", phase),
                            Severity::Warning,
                        ));
                    }
                }

                if let Some(conditions) = status.conditions {
                    for condition in conditions {
                        if condition.status == "False" {
                            issues.push(DiagnosticIssue::new(
                                name.clone(),
                                format!("{}: {}", condition.type_, condition.message.unwrap_or_default()),
                                Severity::Error,
                            ));
                        }
                    }
                }

                if let Some(container_statuses) = status.container_statuses {
                    for cs in container_statuses {
                        if cs.restart_count > 0 {
                            issues.push(DiagnosticIssue::new(
                                name.clone(),
                                format!("Restart Count: {}", cs.restart_count),
                                Severity::Error,
                            ));
                        }
                        if let Some(state) = &cs.state {
                            if let Some(waiting) = &state.waiting {
                                issues.push(DiagnosticIssue::new(
                                    name.clone(),
                                    format!("Waiting: {} {}",
                                            waiting.reason.as_deref().unwrap_or("<no reason>"),
                                            waiting.message.as_deref().unwrap_or("<no message>")),
                                    Severity::Error,
                                ));
                            }
                            if let Some(terminated) = &state.terminated {
                                issues.push(DiagnosticIssue::new(
                                    name.clone(),
                                    format!("Terminated: {} (exit {})",
                                            terminated.reason.as_deref().unwrap_or("<no reason>"),
                                            terminated.exit_code),
                                    Severity::Error,
                                ));
                            }
                        }
                    }

                    // show which node the pod is running on
                    let node_name = status.host_ip.as_deref().unwrap_or("<no node>");
                    issues.push(DiagnosticIssue::new(
                        name.clone(),
                        format!("Terminated: {}",
                                node_name),
                        Severity::Info,
                    ));

                    // show when pod started/age
                    if let Some(start_time) = status.start_time {
                        issues.push(DiagnosticIssue::new(
                            name.clone(),
                            format!("Started at: {}",
                                    start_time.0.to_rfc3339()),
                            Severity::Info,
                        ));
                    }
                }
            }
        }
        Ok(PodsDiagnosticReport{ meta: DiagnosticReport {
            summary: format!("{} pods analyzed", count),
            issues,
        }})
    }
}
