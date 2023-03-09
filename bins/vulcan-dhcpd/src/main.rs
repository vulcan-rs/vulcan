use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dhcp::Server;

use crate::config::Config;

mod config;
mod constants;

#[derive(Parser)]
struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/vulcan/dhcpd.toml"
    )]
    config: PathBuf,

    /// Enables verbose output on STDOUT
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::from_file(cli.config)?;

    let mut srv = Server::builder()
        .with_rebind_time(cfg.rebind_time)
        .with_renew_time(cfg.renew_time)
        .build()?;

    Ok(srv.run()?)
}
