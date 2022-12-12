pub const MIN_DHCP_MSG_SIZE: usize = 300;
pub const DHCP_OPCODE_REQUEST: u8 = 1;
pub const DHCP_OPCODE_REPLY: u8 = 2;
pub const DHCP_MAGIC_COOKIE: u32 = 1_669_485_411;
pub const DHCP_MAGIC_COOKIE_ARR: [u8; 4] = [99, 130, 83, 99];

pub const HARDWARE_ADDR_TYPE_ETHERNET: u8 = 1;
pub const HARDWARE_ADDR_LEN_ETHERNET: u8 = 6;
