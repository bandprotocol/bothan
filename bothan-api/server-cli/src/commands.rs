pub mod config;
pub mod start;

#[async_trait::async_trait]
pub trait CliExec {
    async fn run(&self);
}
