use clap::Parser;
use condor::cli::{Cli, execute_command};
use condor::{ClientError, INaturalistClient};
use log::info;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Create client with specified rate limit
    let client = INaturalistClient::new(cli.rate_limit)?;
    info!(
        "Created iNaturalist client with rate limit: {} req/s",
        cli.rate_limit
    );

    let output_path = cli.output.as_deref();
    execute_command(&client, cli.command, &cli.format, output_path).await?;
    Ok(())
}
