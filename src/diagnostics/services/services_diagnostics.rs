use colored::Colorize;
use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{color_severity, DiagnosticIssue, DiagnosticReport, Severity};
use k8s_openapi::api::core::v1::Endpoints;
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;
use kube::{Api, Client};

pub struct ServicesDiagnostic<'a> {
    client: &'a Client,
    namespace: &'a str
}

/// Convenience constructor
impl<'a> ServicesDiagnostic<'a> {
    pub fn new(client: &'a Client, namespace: &'a str) -> Self {
        Self {
            client,
            namespace
        }
    }
}

pub struct ServicesDiagnosticReport {
    pub meta: DiagnosticReport
}

impl ServicesDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Services Diagnostics: ".bold(), self.meta.summary.yellow());
        for issue in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     color_severity(&issue.resource, issue.severity),
                     issue.message,
            );
        }
    }
}

impl<'a> Diagnostic for ServicesDiagnostic<'a> {
    type Report = ServicesDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<ServicesDiagnosticReport> {
        let endpoints: Api<Endpoints> = Api::namespaced(self.client.clone(), &self.namespace);

        // let ep = endpoints.get(&svc_name).await?;
        // if ep.subsets.is_none() || ep.subsets.as_ref().unwrap().is_empty() {
        //     // flag: service has no endpoints
        // }

        let service_list = endpoints.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = service_list.items;
        let count = items.len();

        for endpoint in items {
            if endpoint.subsets.is_none() || endpoint.subsets.as_ref().unwrap().is_empty() {
                issues.push(DiagnosticIssue::new(
                    "Endpoint".to_string(),
                    endpoint.name().unwrap(),
                    Severity::Info,
                ));
            }
        }

        Ok(ServicesDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} services analyzed", count),
            issues,
        }})
    }
}