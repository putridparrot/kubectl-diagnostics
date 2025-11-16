mod args;
mod diagnostics;

use crate::args::{Args, DiagnoseTarget};
use anyhow::Result;
use clap::Parser;
use kube::Client;
use crate::diagnostics::config_map::config_map_diagnostics::ConfigMapDiagnostic;
use crate::diagnostics::deployment::deployment_diagnostics::DeploymentDiagnostic;
use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::events::events_diagnostic::EventsDiagnostic;
use crate::diagnostics::ingress::ingress_diagnostics::IngressDiagnostic;
use crate::diagnostics::nodes::nodes_diagnostic::NodesDiagnostic;
use crate::diagnostics::pods::pods_diagnostics::PodsDiagnostic;
use crate::diagnostics::resources::resources_diagnostics::ResourcesDiagnostic;
use crate::diagnostics::secrets::secrets_diagnostics::SecretsDiagnostic;
use crate::diagnostics::services::services_diagnostics::ServicesDiagnostic;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;

    let cli = Args::parse();

    match cli.target {
        DiagnoseTarget::ConfigMaps(args) => {
            let namespace = get_namespace(args.namespace);
            ConfigMapDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Deployments(args) => {
            let namespace = get_namespace(args.namespace);
            DeploymentDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Events(args) => {
            let namespace = get_namespace(args.namespace);
            EventsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Ingress(args) => {
            let namespace = get_namespace(args.namespace);
            IngressDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Nodes(_args) => {
            NodesDiagnostic::new(&client)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Pods(args) => {
            let namespace = get_namespace(args.namespace);
            PodsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Resources(args) => {
            let namespace = get_namespace(args.namespace);
            ResourcesDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Secrets(args) => {
            let namespace = get_namespace(args.namespace);
            SecretsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }

        DiagnoseTarget::Services(args) => {
            let namespace = get_namespace(args.namespace);
            ServicesDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
        }


        DiagnoseTarget::All(args) => {
            let namespace = get_namespace(args.namespace);
            ConfigMapDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            DeploymentDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            EventsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            IngressDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            NodesDiagnostic::new(&client)
                .generate_report().await?
                .output_report();
            PodsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            ResourcesDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            SecretsDiagnostic::new(&client, &namespace)
                .generate_report().await?
                .output_report();
            ServicesDiagnostic::new(&client, &namespace)
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