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
