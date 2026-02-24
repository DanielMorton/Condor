use clap::Parser;
use log::info;
use inaturalist_client::{INaturalistClient, ClientError, cli::{Cli, execute_command}};

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
    info!("Created iNaturalist client with rate limit: {} req/s", cli.rate_limit);

    // Execute the command
    execute_command(&client, cli.command, cli.format).await
}