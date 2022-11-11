use std::net::Ipv4Addr;

use bootp::Client;

#[tokio::test]
async fn test_client() {
    let client = match Client::new("127.0.0.1:8068").await {
        Ok(client) => client,
        Err(err) => panic!("{}", err),
    };

    if let Err(err) = client.send(Ipv4Addr::new(255, 255, 255, 255)) {
        panic!("{}", err)
    }
}
