use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_report::{DiagnosticIssue, DiagnosticReport, Severity};
use colored::Colorize;
use k8s_openapi::api::core::v1::Node;
use kube::api::ListParams;
use kube::{Api, Client};

pub struct NodesDiagnostic<'a> {
    client: &'a Client
}

/// Convenience constructor
impl<'a> NodesDiagnostic<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client
        }
    }
}

pub struct NodesDiagnosticReport {
    pub meta: DiagnosticReport
}

impl NodesDiagnosticReport {
    pub fn output_report(self) {
        println!("\n{} {}", "Nodes Diagnostics: ".bold(), self.meta.summary.yellow());
        for node_report in self.meta.issues {
            println!("{} {} : {}",
                     "•".cyan(),
                     node_report.resource.red(),
                     node_report.message,
            );
        }
    }
}

impl<'a> Diagnostic for NodesDiagnostic<'a> {
    type Report = NodesDiagnosticReport;

    async fn generate_report(&self) -> anyhow::Result<NodesDiagnosticReport> {
        let nodes: Api<Node> = Api::all(self.client.clone());
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

        Ok(NodesDiagnosticReport { meta: DiagnosticReport {
            summary: format!("{} nodes analyzed", count),
            issues,
        }})
    }
}