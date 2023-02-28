use std::net::Ipv4Addr;

use crate::types::{
    options::{ClientIdentifier, DhcpMessageType, ParameterRequestList},
    DhcpOption, HardwareAddr, Message, MessageError, OptionData, OptionTag,
};

pub struct MessageBuilder {
    client_hardware_addr: HardwareAddr,
    client_identifier: Option<Vec<u8>>,
    max_dhcp_message_size: u16,
}

impl MessageBuilder {
    pub fn new(
        client_hardware_addr: HardwareAddr,
        client_identifier: Option<Vec<u8>>,
        max_dhcp_message_size: u16,
    ) -> MessageBuilder {
        Self {
            max_dhcp_message_size,
            client_hardware_addr,
            client_identifier,
        }
    }

    /// This creates a new DHCPDISCOVER message with the values described in
    /// RFC 2131 Section 4.
    pub fn make_discover_message(
        &mut self,
        xid: u32,
        destination_addr: Option<Ipv4Addr>,
        requested_addr: Option<Ipv4Addr>,
        requested_lease_time: Option<u32>,
    ) -> Result<Message, MessageError> {
        // The client sets 'ciaddr' to 0x00000000. This is already done in
        // Message::new() (Default value).
        let mut message = Message::new_with_xid(xid);
        self.add_default_options(&mut message)?;

        // Set DHCP message type option
        message.add_option_parts(
            OptionTag::DhcpMessageType,
            OptionData::DhcpMessageType(DhcpMessageType::Discover),
        )?;

        let destination = match destination_addr {
            Some(ip) => ip,
            None => Ipv4Addr::BROADCAST,
        };

        message.add_option_parts(
            OptionTag::ServerIdentifier,
            OptionData::ServerIdentifier(destination),
        )?;

        // The client MAY suggest a network address and/or lease time by
        // including the 'requested IP address' and 'IP address lease time'
        // options.
        if requested_addr.is_some() {
            message.add_option_parts(
                OptionTag::RequestedIpAddr,
                OptionData::RequestedIpAddr(requested_addr.unwrap()),
            )?
        }

        if requested_lease_time.is_some() {
            message.add_option_parts(
                OptionTag::IpAddrLeaseTime,
                OptionData::IpAddrLeaseTime(requested_lease_time.unwrap()),
            )?
        }

        // The client MAY include a different unique identifier in the 'client
        // identifier' option, as discussed in section 4.2.
        let client_identifier = match &self.client_identifier {
            Some(ident) => ident.clone(),
            None => self.client_hardware_addr.as_bytes(),
        };

        message.add_option_parts(
            OptionTag::ClientIdentifier,
            OptionData::ClientIdentifier(ClientIdentifier::from(client_identifier)),
        )?;

        // NOTE (Techassi): Maybe add hostname option

        // The client MAY request specific parameters by including the
        // 'parameter request list' option.
        message.add_option(Self::default_request_parameter_list())?;
        message.end()?;

        message.set_hardware_address(self.client_hardware_addr.clone());
        Ok(message)
    }

    /// This creates a new DHCPREQUEST message with the values described in
    /// RFC 2131 Section 4.
    pub fn make_request_message(
        &self,
        xid: u32,
        destination_addr: Ipv4Addr,
        offered_addr: Ipv4Addr,
        offered_lease_time: u32,
    ) -> Result<Message, MessageError> {
        let mut message = Message::new_with_xid(xid);
        self.add_default_options(&mut message)?;

        // Set DHCP message type option
        message.add_option_parts(
            OptionTag::DhcpMessageType,
            OptionData::DhcpMessageType(DhcpMessageType::Request),
        )?;

        message.add_option_parts(
            OptionTag::ServerIdentifier,
            OptionData::ServerIdentifier(destination_addr),
        )?;

        message.add_option_parts(
            OptionTag::RequestedIpAddr,
            OptionData::RequestedIpAddr(offered_addr),
        )?;

        message.add_option_parts(
            OptionTag::IpAddrLeaseTime,
            OptionData::IpAddrLeaseTime(offered_lease_time),
        )?;

        // NOTE (Techassi): Maybe add hostname option

        message.add_option(Self::default_request_parameter_list())?;
        message.end()?;

        message.set_hardware_address(self.client_hardware_addr.clone());
        Ok(message)
    }

    fn add_default_options(&self, message: &mut Message) -> Result<(), MessageError> {
        message.add_option_parts(
            OptionTag::MaxDhcpMessageSize,
            OptionData::MaxDhcpMessageSize(self.max_dhcp_message_size),
        )
    }

    fn default_request_parameter_list() -> DhcpOption {
        DhcpOption::new(
            OptionTag::ParameterRequestList,
            OptionData::ParameterRequestList(ParameterRequestList::new(vec![
                OptionTag::Router,
                OptionTag::DomainNameServer,
                OptionTag::RenewalT1Time,
                OptionTag::RebindingT2Time,
            ])),
        )
    }
}
