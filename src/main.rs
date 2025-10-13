mod args;

use kube::{Client, api::{Api, ListParams}};
use k8s_openapi::api::core::v1::{Event, Pod};
use anyhow::Result;
use colored::*;
use clap::Parser;
use crate::args::{Args};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let args = Args::parse();

    let namespace = args.namespace.unwrap_or("default".to_string());

    let events: Api<Event> = Api::namespaced(client.clone(), &namespace);
    let lp = ListParams::default().limit(100);
    let event_list = events.list(&lp).await?;

    println!("{}", "Events:".bold());
    for e in event_list.items.iter().rev() {
        let involved = e.involved_object.name.as_deref().unwrap_or("<unknown>");
        let msg = e.message.as_deref().unwrap_or("<no message>");
        let reason = e.reason.as_deref().unwrap_or("<no reason>");
        println!(
            "{} {} {}",
            "•".cyan(),
            involved.yellow(),
            format!("({}) {}", reason, msg)
        );
    }

    let pods: Api<Pod> = Api::namespaced(client.clone(), &namespace);
    let pod_list = pods.list(&lp).await?;

    println!("\n{}", "Pod Diagnostics:".bold());
    for pod in pod_list {
        let name = pod.metadata.name.unwrap_or_default();
        if let Some(status) = pod.status {
            if let Some(phase) = status.phase {
                if phase != "Running" {
                    println!("{} {} {}", "•".cyan(), name.red(), phase);
                }
            }

            if let Some(conditions) = status.conditions {
                for condition in conditions {
                    if condition.status == "False" {
                        println!("{} {} {}: {}",
                                 "•".cyan(),
                                 name.red(),
                                 condition.type_,
                                 condition.message.unwrap_or_default()
                        );
                    }
                }
            }
        }
    }

    Ok(())
}