use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dhcp::Client;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::config::Config;

mod config;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/vulcan/dhcpc.toml"
    )]
    pub config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI args and read config
    let cli = Cli::parse();
    let config = Config::from_file(cli.config)?;

    // Build stdout subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    // Set the above subscriber as the default one
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Build and run client
    let mut client = Client::builder()
        .with_write_timeout(config.write_timeout)
        .with_bind_timeout(config.bind_timeout)
        .with_read_timeout(config.read_timeout)
        .with_interface_name(config.interface)
        .build()?;

    client.run().await?;
    Ok(())
}
