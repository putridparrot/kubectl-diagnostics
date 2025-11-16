use clap::{Parser};

#[derive(Parser)]
#[command(name = "kubectl-diagnostics")]
#[command(about = "Explain why a pod restarted", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub target: DiagnoseTarget,
}

#[derive(Parser)]
pub enum DiagnoseTarget {
    Pods(PodsArgs),
    Events(EventsArgs),
    Nodes(NodesArgs),
    Services(ServicesArgs),
    All(AllArgs)
}

#[derive(Parser)]
pub struct PodsArgs {
    /// The Kubernetes namespace to use
    #[arg(short, long)]
    pub namespace: Option<String>,
}

#[derive(Parser)]
pub struct EventsArgs {
    /// The Kubernetes namespace to use
    #[arg(short, long)]
    pub namespace: Option<String>,
}

#[derive(Parser)]
pub struct NodesArgs {

}

#[derive(Parser)]
pub struct ServicesArgs {
    /// The Kubernetes namespace to use
    #[arg(short, long)]
    pub namespace: Option<String>,
}

#[derive(Parser)]
pub struct AllArgs {
    /// The Kubernetes namespace to use
    #[arg(short, long)]
    pub namespace: Option<String>,
}