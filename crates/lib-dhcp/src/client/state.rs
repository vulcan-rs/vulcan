use std::{error::Error, fmt::Display};

use crate::Client;

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

impl ClientStateMachine for Client {
    fn transition_to(&mut self, state: DhcpState) -> Result<(), DhcpStateError> {
        match self.dhcp_state {
            DhcpState::Init => match state {
                next @ DhcpState::Selecting => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::InitReboot => todo!(),
            DhcpState::Selecting => match state {
                next @ DhcpState::Selecting => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Requesting => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::Rebooting => match state {
                next @ DhcpState::Init => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::InitReboot => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Bound => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::Requesting => match state {
                next @ DhcpState::Init => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Requesting => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Bound => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::Rebinding => match state {
                next @ DhcpState::Init => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Bound => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::Bound => match state {
                next @ DhcpState::Bound => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Renewing => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::Renewing => match state {
                next @ DhcpState::Init => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Rebinding => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Bound => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
        }
    }
}
