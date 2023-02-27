use std::net::Ipv4Addr;

#[derive(Debug, Default)]
pub struct ClientState {
    pub server_identifier: Option<Ipv4Addr>,
    pub offered_ip_address: Option<Ipv4Addr>,
    pub offered_lease_time: Option<u32>,
    pub rebinding_time: Option<u32>,
    pub renewal_time: Option<u32>,
    pub transaction_id: u32,

    // Timers
    pub rebinding_time_left: Option<u32>,
    pub renewal_time_left: Option<u32>,
}
