use std::{net::Ipv4Addr, time::Instant};

use serde::{Deserialize, Serialize};

use crate::types::HardwareAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Lease {
    hardware_addr: HardwareAddr,
    // FIXME (Techassi): I guess we should switch to chrono
    // leased_until: Instant,
    ip_addr: Ipv4Addr,
    lease_time: u32,
}
