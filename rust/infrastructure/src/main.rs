use clean_architecture_infrastructure::cli;
use clean_architecture_shared::error::CliResult;

#[tokio::main]
async fn main() -> CliResult<()> {
    cli::start().await
}
