use std::path::PathBuf;

use clap::Parser;
use dhcp::Client;

use crate::{cli::Cli, config::Config};

mod cli;
mod config;

// TODO (Techassi): Use anyhow
fn main() {
    let cli = Cli::parse();

    let config_path = cli
        .config
        .unwrap_or(PathBuf::from("/etc/vulcan-dhcpc/config.toml"));

    let config = match Config::from_file(config_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    };

    let mut client = match Client::builder()
        .with_write_timeout(config.write_timeout)
        .with_bind_timeout(config.bind_timeout)
        .with_read_timeout(config.read_timeout)
        .with_interface_name(config.interface)
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    };

    if let Err(err) = client.run() {
        panic!("{}", err)
    }
}
