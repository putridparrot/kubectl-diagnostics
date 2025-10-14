use chrono::{DateTime, Utc};
use colored::Colorize;
use k8s_openapi::api::core::v1::Event;
use kube::{Api, Client};
use kube::api::ListParams;
use crate::diagnostic::Diagnostic;
use crate::diagnostic_report::{DiagnosticIssue, DiagnosticReport, Severity};
use crate::output_mode::OutputMode;

#[derive(Debug)]
pub struct EventsDiagnostic {
    pub output_mode: OutputMode,
}

impl Diagnostic for EventsDiagnostic {
    async fn run(&self, client: Client, namespace: &str) -> anyhow::Result<DiagnosticReport> {
        let events: Api<Event> = Api::namespaced(client.clone(), &namespace);
        let lp = ListParams::default().limit(100);
        let event_list = events.list(&lp).await?;

        let mut issues = vec![];
        let items = event_list.items;
        let count = items.len();

        for e in items.iter().rev() {
            let involved = e.involved_object.name.as_deref().unwrap_or("<unknown>");
            let msg = e.message.as_deref().unwrap_or("<no message>");
            let reason = e.reason.as_deref().unwrap_or("<no reason>");
            let ts_str = extract_timestamp(e)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "<no time>".to_string());
            let source = e.source.as_ref().and_then(|s| s.component.as_deref()).unwrap_or("<unknown>");

            issues.push(DiagnosticIssue::new(
                involved,
                format!("({}) {} {} {}", reason.bright_yellow(), ts_str.dimmed(), msg, source.dimmed()),
                Severity::Warning,
            ));
        }

        Ok(DiagnosticReport {
            summary: format!("{} events analyzed", count),
            issues,
        })
    }
}


fn extract_timestamp(e: &Event) -> Option<DateTime<Utc>> {
    if let Some(mt) = &e.event_time {
        mt.0.to_owned().into()
    } else if let Some(ts) = &e.last_timestamp {
        Some(ts.0.to_owned())
    } else {
        None
    }
}
