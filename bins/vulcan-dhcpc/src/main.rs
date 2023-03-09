use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dhcp::Client;

use crate::config::Config;

mod config;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/vulcan/dhcpc.toml"
    )]
    pub config: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::from_file(cli.config)?;

    let mut client = Client::builder()
        .with_write_timeout(config.write_timeout)
        .with_bind_timeout(config.bind_timeout)
        .with_read_timeout(config.read_timeout)
        .with_interface_name(config.interface)
        .build()?;

    Ok(client.run()?)
}
