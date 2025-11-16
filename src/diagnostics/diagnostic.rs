pub trait Diagnostic {
    type Report;
    async fn generate_report(&self) -> anyhow::Result<Self::Report>;
}
