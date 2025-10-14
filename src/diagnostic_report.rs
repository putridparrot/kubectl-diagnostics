use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticIssue {
    pub resource: String,         // e.g. pod name, node name
    pub message: String,          // diagnostic message
    pub severity: Severity,       // severity level
    pub namespace: Option<String>,
    pub reason: Option<String>,    // optional reason code
    pub timestamp: Option<String>, // ISO 8601 string
}

impl DiagnosticIssue {
    pub fn new(resource: impl Into<String>, message: impl Into<String>, severity: Severity) -> Self {
        DiagnosticIssue {
            resource: resource.into(),
            message: message.into(),
            severity,
            namespace: None,
            reason: None,
            timestamp: None,
        }
    }
}

pub struct DiagnosticReport {
    pub summary: String,
    pub issues: Vec<DiagnosticIssue>,
}

