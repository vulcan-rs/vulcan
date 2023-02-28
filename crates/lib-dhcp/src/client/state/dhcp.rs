use std::{error::Error, fmt::Display};

use crate::Client;

#[derive(Debug, Clone)]
pub enum DhcpState {
    Init,
    InitReboot,
    Selecting,
    SelectingSent,
    Rebooting,
    Requesting,
    RequestingSent,
    Rebinding,
    RebindingSent,
    Bound,
    Renewing,
    RenewingSent,
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
            DhcpState::SelectingSent => write!(f, "SELECTING-SENT"),
            DhcpState::Rebooting => write!(f, "REBOOTING"),
            DhcpState::Requesting => write!(f, "REQUESTING"),
            DhcpState::RequestingSent => write!(f, "REQUESTING-SENT"),
            DhcpState::Rebinding => write!(f, "REBINDING"),
            DhcpState::RebindingSent => write!(f, "REBINDING-SENT"),
            DhcpState::Bound => write!(f, "BOUND"),
            DhcpState::Renewing => write!(f, "RENEWING"),
            DhcpState::RenewingSent => write!(f, "RENEWING-SENT"),
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

pub trait DhcpStateMachine {
    fn transition_to(&mut self, state: DhcpState) -> Result<(), DhcpStateError>;
}

impl DhcpStateMachine for Client {
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
                next @ DhcpState::SelectingSent => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::SelectingSent => match state {
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
                next @ DhcpState::RequestingSent => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::RequestingSent => match state {
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
                next @ DhcpState::RebindingSent => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::RebindingSent => match state {
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
                next @ DhcpState::RenewingSent => {
                    self.dhcp_state = next;
                    Ok(())
                }
                _ => Err(DhcpStateError::new(self.dhcp_state.clone(), state)),
            },
            DhcpState::RenewingSent => match state {
                next @ DhcpState::Init => {
                    self.dhcp_state = next;
                    Ok(())
                }
                next @ DhcpState::Renewing => {
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
