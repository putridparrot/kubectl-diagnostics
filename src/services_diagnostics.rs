use k8s_openapi::api::core::v1::{Endpoints, Event, Node};
use kube::{Api, Client};
use kube::api::ListParams;
use kube::runtime::reflector::Lookup;
use crate::diagnostic::Diagnostic;
use crate::diagnostic_report::{DiagnosticIssue, DiagnosticReport, Severity};
use crate::output_mode::OutputMode;

#[derive(Debug)]
pub struct ServicesDiagnostic {
    pub output_mode: OutputMode,
}

impl Diagnostic for ServicesDiagnostic {
    async fn run(&self, client: Client, namespace: &str) -> anyhow::Result<DiagnosticReport> {
        let endpoints: Api<Endpoints> = Api::namespaced(client.clone(), &namespace);

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

        Ok(DiagnosticReport {
            summary: format!("{} services analyzed", count),
            issues,
        })
    }
}