use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{Api, Client};
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;

pub struct ConfigMapDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> ConfigMapDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct ConfigMapDiagnosticReport {
    pub meta: DiagnosticReport
}

impl ConfigMapDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "ConfigMap Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for ConfigMapDiagnostic<'a> {
    type Report = ConfigMapDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<ConfigMapDiagnosticReport> {
        const TITLE: &str = "ConfigMap";

        let api: Api<ConfigMap> = Api::namespaced(self.client.clone(), &self.namespace);
        let list = api.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = list.items;
        let count = items.len();

        for config_map in items {
            if config_map.data.is_none() || config_map.data.as_ref().unwrap().is_empty() {
                issues.push(DiagnosticIssue::new(
                    TITLE,
                    config_map.name().unwrap().to_string(),
                    Severity::Warning,
                ));
            }
            if let Some(data) = &config_map.data {
                for (key, value) in data {
                    if value.trim().is_empty() {
                        issues.push(DiagnosticIssue::new(
                            TITLE,
                            format!("{}: contains no value", key),
                            Severity::Info),
                        )
                    }
                }
            }
        }

        Ok(ConfigMapDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} config maps analyzed", count),
            issues,
        }})
    }
}