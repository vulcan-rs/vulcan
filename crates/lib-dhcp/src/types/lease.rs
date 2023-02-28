use std::{net::Ipv4Addr, time::Instant};

use crate::types::HardwareAddr;

#[derive(Debug)]
pub struct Lease {
    hardware_addr: HardwareAddr,
    leased_until: Instant,
    ip_addr: Ipv4Addr,
    lease_time: u32,
}
