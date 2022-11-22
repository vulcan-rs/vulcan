mod client;
mod constants;
mod types;

pub use client::*;
pub use constants::*;
pub use types::*;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) {}

    pub async fn stop(&self) {}
}
