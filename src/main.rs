mod args;
mod output_mode;
mod diagnostic_report;
mod pods_diagnostics;
mod diagnostic;
mod events_diagnostic;
mod nodes_diagnostic;
mod services_diagnostics;

use crate::args::Args;
use crate::diagnostic::Diagnostic;
use crate::events_diagnostic::EventsDiagnostic;
use crate::nodes_diagnostic::NodesDiagnostic;
use crate::pods_diagnostics::PodsDiagnostic;
use crate::services_diagnostics::ServicesDiagnostic;
use anyhow::Result;
use clap::Parser;
use colored::*;
use kube::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let args = Args::parse();

    let namespace = args.namespace.unwrap_or("default".to_string());

    let services_diagnostics = ServicesDiagnostic {
        output_mode: output_mode::OutputMode::Console,
    };

    let services_report = services_diagnostics.run(client.clone(), &namespace).await?;

    println!("\n{} {}", "Services Diagnostics: ".bold(), services_report.summary.yellow());
    for node_report in services_report.issues {
        println!("{} {} : {}",
                 "•".cyan(),
                 node_report.resource.red(),
                 node_report.message,
        );
    }

    let nodes_diagnostics = NodesDiagnostic {
        output_mode: output_mode::OutputMode::Console,
    };

    let nodes_report = nodes_diagnostics.run(client.clone(), &namespace).await?;

    println!("\n{} {}", "Nodes Diagnostics: ".bold(), nodes_report.summary.yellow());
    for node_report in nodes_report.issues {
        println!("{} {} : {}",
                 "•".cyan(),
                 node_report.resource.red(),
                 node_report.message,
        );
    }

    let events_diagnostics = EventsDiagnostic {
        output_mode: output_mode::OutputMode::Console,
    };

    let events_report = events_diagnostics.run(client.clone(), &namespace).await?;

    println!("\n{} {}", "Events Diagnostics: ".bold(), events_report.summary.yellow());
    for event_report in events_report.issues {
        println!("{} {} : {}",
                 "•".cyan(),
                 event_report.resource.red(),
                 event_report.message,
        );
    }


    let pods_diagnostics = PodsDiagnostic{
        output_mode: output_mode::OutputMode::Console,
    };

    let pods_report = pods_diagnostics.run(client.clone(), &namespace).await?;

    println!("\n{} {}", "Pod Diagnostics: ".bold(), pods_report.summary.yellow());
    for pod_report in pods_report.issues {
        println!("{} {} : {}",
                 "•".cyan(),
                 pod_report.resource.red(),
                 pod_report.message,
        );
    }

    Ok(())
}
