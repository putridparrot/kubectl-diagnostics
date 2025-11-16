use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::core::v1::{Pod, Secret};
use kube::{Api, Client, ResourceExt};
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;

pub struct SecretsDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> SecretsDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct SecretsDiagnosticReport {
    pub meta: DiagnosticReport
}

impl SecretsDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Secrets Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for SecretsDiagnostic<'a> {
    type Report = SecretsDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<SecretsDiagnosticReport> {

        let api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);
        let list = api.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = list.items;
        let count = items.len();

        for item in items {
            let secret_type = item.type_.as_deref().unwrap_or("Opaque");
            issues.push(DiagnosticIssue::new(
                "Secrets".to_string(),
                format!("Secret `{}` type: `{}`", item.name().unwrap(), secret_type),
                Severity::Info,
            ));

            if let Some(data) = &item.data {
                for (key, value) in data {
                    if value.0.is_empty() {
                        issues.push(DiagnosticIssue::new(
                            "Secrets".to_string(),
                            format!("Key `{}` in secret `{}` is empty", key, item.name().unwrap()),
                            Severity::Warning,
                        ));
                    } else if value.0.len() > 4096 {
                        issues.push(DiagnosticIssue::new(
                            "Secrets".to_string(),
                            format!("Key `{}` in secret `{}` is unusually large", key, item.name().unwrap()),
                            Severity::Info,
                        ));
                    }
                }
            } else {
                issues.push(DiagnosticIssue::new(
                    "Secrets".to_string(),
                    format!("Secret `{}` has no data", item.name().unwrap()),
                    Severity::Warning,
                ));
            }

            let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
            let pods = pod_api.list(&Default::default()).await?;
            let mut used_by = vec![];

            for pod in &pods {
                let spec = match &pod.spec {
                    Some(s) => s,
                    None => continue,
                };

                if let Some(volumes) = &spec.volumes {
                    let uses_secret = volumes.iter().any(|v| {
                        v.secret.as_ref().map_or(false, |s| s.secret_name.clone().unwrap() == item.name().unwrap())
                    }) || spec.containers.iter().any(|c| {
                        c.env.iter().any(|e| {
                            e.iter().any(|v| {
                                v.value_from.as_ref().map_or(false, |vf| {
                                    vf.secret_key_ref.as_ref().map_or(false, |sk| sk.name.to_string() == item.name().unwrap())
                                })
                            })
                        })
                    });

                    if uses_secret {
                        used_by.push(pod.name_any());
                    }
                }
            }

            if used_by.is_empty() {
                issues.push(DiagnosticIssue::new(
                    "Secrets".to_string(),
                    format!("Secret `{}` is not used by any pods in namespace `{}`", item.name().unwrap(), self.namespace),
                    Severity::Info,
                ));
            }
        }

        Ok(SecretsDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} secrets analyzed", count),
            issues,
        }})
    }
}