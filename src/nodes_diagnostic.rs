use k8s_openapi::api::core::v1::{Event, Node};
use kube::{Api, Client};
use kube::api::ListParams;
use crate::diagnostic::Diagnostic;
use crate::diagnostic_report::{DiagnosticIssue, DiagnosticReport, Severity};
use crate::output_mode::OutputMode;

#[derive(Debug)]
pub struct NodesDiagnostic {
    pub output_mode: OutputMode,
}

impl Diagnostic for NodesDiagnostic {
    async fn run(&self, client: Client, namespace: &str) -> anyhow::Result<DiagnosticReport> {
        let nodes: Api<Node> = Api::all(client.clone());
        let node_list = nodes.list(&ListParams::default()).await?;

        let mut issues = vec![];
        let items = node_list.items;
        let count = items.len();

        for node in items.iter().rev() {
            let n = node.status.as_ref().unwrap();
            for node_condition in n.conditions.iter() {
                for condition in node_condition {
                    if condition.type_ == "Ready" && condition.status == "False" {
                        issues.push(DiagnosticIssue::new(
                            "Node".to_string(),
                            condition.reason.clone().unwrap_or_default(),
                            Severity::Info,
                        ));
                    }
                }
            }
        }

        Ok(DiagnosticReport {
            summary: format!("{} nodes analyzed", count),
            issues,
        })
    }
}