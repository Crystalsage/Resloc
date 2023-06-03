use crate::types::hosts::*;
use crate::errors::HostError;
use crate::domains;


use publicsuffix::{List, Psl};


lazy_static::lazy_static! {
    static ref LIST: List = include_str!("../data/psl.dat").parse().unwrap();
}

pub fn get_public_suffix(host: Host) -> Option<String> {
    match host.host_type {
        HostType::Domain => {},
        _ => return None,
    }

    let public_suffix =  LIST.suffix(host.value.as_bytes())
        .unwrap()
        .as_bytes();

    let public_suffix = String::from_utf8(public_suffix.to_vec()).unwrap();

    return Some(public_suffix);
}


pub fn get_registrable_domain(host: Host) -> Option<String> {
    match host.host_type {
        HostType::Domain => {},
        _ => return None,
    }

    let domain =  LIST.domain(host.value.as_bytes());
    if domain.is_none() {
        return None;
    }

    let domain = String::from_utf8(domain.unwrap().as_bytes().to_vec()).unwrap();

    return Some(domain);
}

fn parse_ipv6_address(input: &[char]) -> Host {
    let host = Host::new(input.iter().collect(), HostType::IPAddress(IPAddress::IPv6(0_u128)));
    return host;
}

fn opaque_host_parsing(input: &[char]) -> Host {
    let host = Host::new(input.iter().collect(), HostType::Opaque);
    return host;
}

pub fn host_parser(input: &str, is_not_special: bool) -> Result<Host, HostError> {
    let input: Vec<char> = input.chars().collect();

    if input.get(0) == Some(&'[') {
        if input.last() != Some(&']') {
            eprintln!("{}", HostError::Ipv6Unclosed);
            return Err(HostError::Ipv6Unclosed);
        }

        let result = parse_ipv6_address(&input[1..input.len()]);
        return Ok(result);
    }

    if is_not_special {
        let result = opaque_host_parsing(&input);
        return Ok(result);
    }

    assert_ne!(input.len(), 0);

    // UTF-8 decode without BOM on the percent-decoding of input.
    let domain: String = String::new();

    let ascii_domain = domains::domain_to_ascii(domain, false);
    if ascii_domain.is_err() {
        return Err(HostError::Ipv4Failure);
    }

    if ascii_domain.as_ref().unwrap().chars().last().unwrap().is_digit(10) {
        let ipv4_address = domains::ipv4_parser(input.iter().collect());
        if let Err(e) = ipv4_address {
            return Err(HostError::Ipv4Failure);
        }
        let result = Host::new("".to_string(), HostType::IPAddress(IPAddress::IPv4(ipv4_address.unwrap())));
        return Ok(result); 
    }

    let result = Host::new(ascii_domain.unwrap(), HostType::Domain);
    return Ok(result);
}


pub fn host_serializer(host: Host) -> String {
    match host.host_type {
        HostType::Empty | HostType::Opaque | HostType::Domain => host.value,
        HostType::IPAddress(IPAddress::IPv4(address)) => {
            let result = domains::ipv4_serializer(address);
            return result;
        },
        HostType::IPAddress(IPAddress::IPv6(address)) => {
            let result = domains::ipv6_serializer(domains::Ipv6Pieces::from(domains::Ipv6Address(address)));
            return format!("[{}]", result);
        },
    }
}
