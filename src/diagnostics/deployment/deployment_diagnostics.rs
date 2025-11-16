use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client};
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;

pub struct DeploymentDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> DeploymentDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct DeploymentDiagnosticReport {
    pub meta: DiagnosticReport
}

impl DeploymentDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Deployment Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for DeploymentDiagnostic<'a> {
    type Report = DeploymentDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<DeploymentDiagnosticReport> {

        let api: Api<Deployment> = Api::namespaced(self.client.clone(), self.namespace);
        let list = api.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = list.items;
        let count = items.len();

        for item in items {
            if let Some(status) = &item.status {
                if let Some(conditions) = &status.conditions {
                    for cond in conditions {
                        if cond.type_ == "Progressing" && cond.status == "False" {
                            if let Some(reason) = &cond.reason {
                                if reason == "ProgressDeadlineExceeded" {
                                    issues.push(DiagnosticIssue::new(
                                        "Deployment".to_string(),
                                        format!("Rollout failed for deployment `{}`: Progress deadline exceeded", item.name().unwrap()),
                                        Severity::Warning,
                                    ));

                                    continue;
                                }
                            }
                        }
                    }
                }

                if let Some(unavailable) = status.unavailable_replicas {
                    if unavailable > 0 {
                        issues.push(DiagnosticIssue::new(
                            "Deployment".to_string(),
                            format!("Deployment `{}` has {} unavailable replicas", item.name().unwrap(), unavailable),
                            Severity::Warning,
                        ));
                    }
                }
            }
        }

        Ok(DeploymentDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} deployments analyzed", count),
            issues,
        }})
    }
}