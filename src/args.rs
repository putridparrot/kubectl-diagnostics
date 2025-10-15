use clap::{Parser};

#[derive(Parser)]
#[command(name = "kubectl-diagnostics")]
#[command(about = "Explain why a pod restarted", long_about = None)]
pub struct Args {
    /// The Kubernetes namespace to use
    #[arg(short, long)]
    pub namespace: Option<String>,
}

// #[derive(Parser)]
// enum DiagnoseTarget {
//     Pods(PodsArgs),
//     Events(EventsArgs),
//     Nodes(NodesArgs),
//     Services(ServicesArgs),
// }