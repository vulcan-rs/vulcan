use std::net::Ipv4Addr;

use dhcp::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    if let Err(err) = client
        .send(Ipv4Addr::new(255, 255, 255, 255), "0.0.0.0:68")
        .await
    {
        panic!("{}", err)
    }
}
