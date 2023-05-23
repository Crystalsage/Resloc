pub enum IPAddress {
    IPv4(u32),
    IPv6(u128),
}

pub enum HostType {
    Domain,
    IPAddress,
    Opaque,
    Empty
}

pub struct Host {
    pub value: String,
    pub host_type: HostType,
}
