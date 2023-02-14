use dhcp::Client;

fn main() {
    let mut client = Client::new();

    if let Err(err) = client.run() {
        panic!("{}", err)
    }
}
