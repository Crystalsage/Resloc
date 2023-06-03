use crate::errors::HostError;

pub enum IPAddress {
    IPv4(u32),
    IPv6(u128),
}

pub enum HostType {
    Domain,
    IPAddress(IPAddress),
    Opaque,
    Empty
}

pub struct Host {
    pub value: String,
    pub host_type: HostType,
}

impl Host {
    pub fn new(value: String, host_type: HostType) -> Self {
        Host {
            value: value.to_string(),
            host_type
        }
    }
}

pub type IPv4 = u32;
pub type Ipv4NumberResult = Result<(u8, bool), HostError>;

pub struct Ipv6Address(pub u128);
pub type Ipv6Pieces = [u16; 8];

impl From<Ipv6Address> for Ipv6Pieces {
    fn from(value: Ipv6Address) -> Self {
        [
            (value.0 >> (0 * 16) & 0xFFFF) as u16,
            (value.0 >> (1 * 16) & 0xFFFF) as u16,
            (value.0 >> (2 * 16) & 0xFFFF) as u16,
            (value.0 >> (3 * 16) & 0xFFFF) as u16,
            (value.0 >> (4 * 16) & 0xFFFF) as u16,
            (value.0 >> (5 * 16) & 0xFFFF) as u16,
            (value.0 >> (6 * 16) & 0xFFFF) as u16,
            (value.0 >> (7 * 16) & 0xFFFF) as u16,
        ]
    }
}

