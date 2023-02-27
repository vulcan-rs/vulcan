use std::{
    net::Ipv4Addr,
    process::{Command, ExitStatus},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("Unexpected exist status: {0}")]
    UnexpectedStatus(ExitStatus),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn set_interface_up(interface_name: &String) -> Result<(), CmdError> {
    let status = Command::new("ip")
        .args(["link", "set"])
        .args(["dev", interface_name, "up"])
        .status()?;

    if !status.success() {
        return Err(CmdError::UnexpectedStatus(status));
    }

    Ok(())
}

/// Flushes the IP address of the interface with `interface_name`.
pub fn flush_ip_address(interface_name: &String) -> Result<(), CmdError> {
    // ip -4 addr flush dev ${interface}
    let status = Command::new("ip")
        .arg("-4")
        .args(["addr", "flush"])
        .args(["dev", interface_name])
        .status()?;

    if !status.success() {
        return Err(CmdError::UnexpectedStatus(status));
    }

    Ok(())
}

/// Adds an IP address to the interface with `interface_name`.
pub fn add_ip_address(ip_addr: &Ipv4Addr, interface_name: &String) -> Result<(), CmdError> {
    let status = Command::new("ip")
        .arg("-4")
        .args(["addr", "add", &ip_addr.to_string()])
        .args(["dev", interface_name])
        .status()?;

    if !status.success() {
        return Err(CmdError::UnexpectedStatus(status));
    }

    Ok(())
}
