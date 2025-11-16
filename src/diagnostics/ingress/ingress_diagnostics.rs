use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::core::v1::{Endpoints, Pod, Secret, Service};
use k8s_openapi::api::networking::v1::Ingress;
use kube::{Api, Client};
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;

pub struct IngressDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> IngressDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct IngressDiagnosticReport {
    pub meta: DiagnosticReport
}

impl IngressDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Ingress Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for IngressDiagnostic<'a> {
    type Report = IngressDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<IngressDiagnosticReport> {
        let api: Api<Ingress> = Api::namespaced(self.client.clone(), &self.namespace);
        let list = api.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = list.items;
        let count = items.len();

        for item in items {
            if let Some(lb) = &item.status.clone().and_then(|s| s.load_balancer) {
                if lb.ingress.is_none() {
                    issues.push(DiagnosticIssue::new(
                        "LoadBalancer".to_string(),
                        format!("Ingress `{}` has no external address", item.name().unwrap()),
                        Severity::Warning,
                    ));
                }

                if let Some(tls) = &item.spec.clone().and_then(|s| s.tls) {
                    for entry in tls {
                        if let Some(secret_name) = &entry.secret_name {
                            let secret_api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);
                            if let Err(_) = secret_api.get(secret_name).await {
                                issues.push(DiagnosticIssue::new(
                                    "LoadBalancer".to_string(),
                                    format!("TLS secret `{}` missing", secret_name),
                                    Severity::Warning,
                                ));
                            }
                        }
                    }
                }

                if let Some(rules) = &item.spec.clone().and_then(|s| s.rules) {
                    for rule in rules {
                        if let Some(http) = &rule.http {
                            for path in &http.paths {
                                let svc_name = &path.backend.service.clone().unwrap().name;
                                let svc_api: Api<Service> = Api::namespaced(self.client.clone(), &self.namespace);
                                let ep_api: Api<Endpoints> = Api::namespaced(self.client.clone(), &self.namespace);

                                if let Err(_) = svc_api.get(svc_name).await {
                                    issues.push(DiagnosticIssue::new(
                                        "LoadBalancer".to_string(),
                                        format!("Service `{}` missing", svc_name),
                                        Severity::Warning,
                                    ));
                                }

                                match ep_api.get(svc_name).await {
                                    Ok(ep) => {
                                        let ready = ep.subsets.iter().any(|s| s.iter().any(|q| q.addresses.is_some()));
                                        if !ready {
                                            issues.push(DiagnosticIssue::new(
                                                "LoadBalancer".to_string(),
                                                format!("Service `{}` has no endpoints", svc_name),
                                                Severity::Warning,
                                            ));
                                        }
                                    }
                                    Err(_) => {
                                        issues.push(DiagnosticIssue::new(
                                            "LoadBalancer".to_string(),
                                            format!("Endpoints for service `{}` not found", svc_name),
                                            Severity::Warning,
                                        ));
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), "ingress-nginx");
        let pods = pod_api.list(&Default::default()).await?;
        let healthy = pods.iter().any(|p| {
            p.status.as_ref().map_or(false, |s| s.phase.as_deref() == Some("Running"))
        });

        if !healthy {
            issues.push(DiagnosticIssue::new(
                "Ingress Controller".to_string(),
                "Ingress controller is not healthy",
                Severity::Warning,
            ));
        }

        Ok(IngressDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} ingresses analyzed", count),
            issues,
        }})
    }
}