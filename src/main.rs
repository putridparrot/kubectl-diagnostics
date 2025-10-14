mod args;

use crate::args::Args;
use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use colored::*;
use k8s_openapi::api::core::v1::{Event, Pod};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{MicroTime, Time};
use kube::{
    Client,
    api::{Api, ListParams},
};

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
        let ts_str = extract_timestamp(e)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "<no time>".to_string());
        let source = e.source.as_ref().and_then(|s| s.component.as_deref()).unwrap_or("<unknown>");
        println!(
            "{} {} {} {}",
            "•".cyan(),
            involved.yellow(),
            format!("({})", reason),
            format!("{} {} {}", ts_str.dimmed(), msg, source.dimmed())
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
                        println!(
                            "{} {} {}: {}",
                            "•".cyan(),
                            name.red(),
                            condition.type_,
                            condition.message.unwrap_or_default()
                        );
                    }
                }
            }

            if let Some(container_statuses) = status.container_statuses {
                for cs in container_statuses {
                    if cs.restart_count > 0 {
                        println!(
                            "{} {} restart count: {}",
                            "•".cyan(),
                            name.magenta(),
                            cs.restart_count.to_string().yellow()
                        );
                    }
                    if let Some(state) = &cs.state {
                        if let Some(waiting) = &state.waiting {
                            println!(
                                "{} {} waiting: {} - {}",
                                "•".cyan(),
                                name.red(),
                                waiting.reason.as_deref().unwrap_or("<no reason>"),
                                waiting.message.as_deref().unwrap_or("<no message>")
                            );
                        }
                        if let Some(terminated) = &state.terminated {
                            println!(
                                "{} {} terminated: {} (exit {})",
                                "•".cyan(),
                                name.red(),
                                terminated.reason.as_deref().unwrap_or("<no reason>"),
                                terminated.exit_code
                            );
                        }
                    }
                }
            }

            // show which node the pod is running on
            let node_name = status.host_ip.as_deref().unwrap_or("<no node>");
            println!(
                "{} {} on node {}",
                "•".cyan(),
                name.blue(),
                node_name.dimmed()
            );

            // show when the pod started/age
            if let Some(start_time) = status.start_time {
                println!(
                    "{} {} started at {}",
                    "•".cyan(),
                    name.green(),
                    start_time.0.to_rfc3339().dimmed()
                );
            }
        }
    }

    Ok(())
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
