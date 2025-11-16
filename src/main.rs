mod args;
mod diagnostics;

use crate::args::{Args, DiagnoseTarget};
use anyhow::Result;
use clap::Parser;
use kube::Client;
use crate::diagnostics::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;

    let cli = Args::parse();

    match cli.target {
        DiagnoseTarget::Services(args) => {
            let namespace = get_namespace(args.namespace);
            ServicesDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Nodes(_args) => {
            NodesDiagnostic::new(&client)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Events(args) => {
            let namespace = get_namespace(args.namespace);
            EventsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Pods(args) => {
            let namespace = get_namespace(args.namespace);
            PodsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }
        
        DiagnoseTarget::All(args) => {
            let namespace = get_namespace(args.namespace);
            ServicesDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            NodesDiagnostic::new(&client)
                .generate_report().await?
                .output_report();
            EventsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            PodsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }
    }
    Ok(())
}

fn get_namespace(namespace: Option<String>) -> String {
    namespace
        .unwrap_or("default".to_string())
}