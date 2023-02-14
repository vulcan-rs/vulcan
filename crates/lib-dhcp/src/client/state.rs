use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum DhcpState {
    Init,
    InitReboot,
    Selecting,
    Rebooting,
    Requesting,
    Rebinding,
    Bound,
    Renewing,
}

impl Default for DhcpState {
    fn default() -> Self {
        Self::Init
    }
}

impl Display for DhcpState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DhcpState::Init => write!(f, "INIT"),
            DhcpState::InitReboot => write!(f, "INIT-REBOOT"),
            DhcpState::Selecting => write!(f, "SELECTING"),
            DhcpState::Rebooting => write!(f, "REBOOTING"),
            DhcpState::Requesting => write!(f, "REQUESTING"),
            DhcpState::Rebinding => write!(f, "REBINDING"),
            DhcpState::Bound => write!(f, "BOUND"),
            DhcpState::Renewing => write!(f, "RENEWING"),
        }
    }
}

#[derive(Debug)]
pub struct DhcpStateError {
    from: DhcpState,
    to: DhcpState,
}

impl Error for DhcpStateError {}

impl Display for DhcpStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid DHCP state transition from '{}' to '{}'",
            self.from, self.to
        )
    }
}

impl DhcpStateError {
    pub fn new(from: DhcpState, to: DhcpState) -> Self {
        Self { from, to }
    }
}

pub trait ClientStateMachine {
    fn transition_to(&mut self, state: DhcpState) -> Result<(), DhcpStateError>;
}
