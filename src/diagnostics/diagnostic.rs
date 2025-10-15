use kube::Client;
use crate::diagnostics::diagnostic_report::DiagnosticReport;

pub trait Diagnostic {
    async fn run(&self, client: Client, namespace: &str) -> anyhow::Result<DiagnosticReport>;
}
