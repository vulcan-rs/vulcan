use std::{path::PathBuf, process::exit};

use clap::Parser;
use dhcp::Server;

use crate::config::Config;

mod config;
mod constants;

#[derive(Parser)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enables verbose output on STDOUT
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let cfg = match Config::read(cli.config) {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    };

    let mut srv = Server::new();

    if let Err(err) = srv.run() {
        println!("{}", err);
        exit(1)
    }
}
