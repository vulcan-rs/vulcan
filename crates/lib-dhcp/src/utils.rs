use std::{future::Future, time::Duration};

use network_interface::{Error as InterfaceError, NetworkInterface, NetworkInterfaceConfig};
use tokio::time::timeout as to;

pub enum TimeoutResult<O, E> {
    Timeout,
    Error(E),
    Ok(O),
}

/// Requires a future to complete before the specified duration has elapsed.
/// This type returns one of three distinct variants:
///
/// - [`TimeoutResult::Timeout`] indicates that the specified duration has
///   elapsed.
/// - [`TimeoutResult::Error`] indicates that the future resulted in an error.
/// - [`TimeoutResult::Ok`] indicates the future resolved correctly and has
///   returned a success value.
///
/// ### Example
///
/// ```ignore
/// use std::time::Duration;
///
/// use portal::utils::{timeout, TimeoutResult};
/// use tokio::net::UdpSocket;
///
/// let socket = match timeout(Duration::from_secs(2), UdpSocket::bind("127.0.0.1:0")).await {
///     TimeoutResult::Timeout => panic!("Binding UDP socket timed out"),
///     TimeoutResult::Error(err) => panic!("An error occurred: {}", err),
///     TimeoutResult::Ok(socket) => socket,
/// };
/// ```
pub async fn timeout<T: Future<Output = Result<O, E>>, O, E>(
    d: Duration,
    f: T,
) -> TimeoutResult<O, E> {
    match to(d, f).await {
        Ok(res) => match res {
            Ok(o) => TimeoutResult::Ok(o),
            Err(err) => TimeoutResult::Error(err),
        },
        Err(_) => TimeoutResult::Timeout,
    }
}

pub fn select_network_interface(
    name: &String,
    fallback: bool,
) -> Result<Option<NetworkInterface>, InterfaceError> {
    let interfaces = NetworkInterface::show()?;

    println!("Found {} interfaces", interfaces.len());

    for interface in interfaces {
        println!("{interface:?}");
        // Return immediately when we found the interface with the
        // user-provided name
        if interface.name == *name {
            return Ok(Some(interface));
        }

        // If we don't want to fallback, continue
        if !fallback {
            continue;
        }

        // Filter out interfaces like loopback (lo) and wireguard (wgX)
        if interface.name.starts_with("lo") || interface.name.starts_with("wg") {
            continue;
        }

        // TODO (Techassi): This should also filter out null addresses
        if interface.mac_addr.is_none() {
            continue;
        }

        // Filter out interfaces with IPv6 addresses, as this DHCP
        // implementation is aimed at IPv4
        if interface.addr.filter(|a| a.ip().is_ipv6()).is_some() {
            continue;
        }

        // The fallback interface
        return Ok(Some(interface));
    }

    Ok(None)
}
