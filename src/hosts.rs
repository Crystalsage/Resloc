use crate::types::types::*;
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
            let result = domains::ipv6_serializer(Ipv6Pieces::from(Ipv6Address(address)));
            return format!("[{}]", result);
        },
    }
}

pub fn ipv6_parser(input: String) -> Result<Ipv6Pieces, HostError> {
    let mut  ipv6: Ipv6Pieces = [0_u16; 8];

    let mut piece_index: usize = 0;

    let input: Vec<char> = input.chars().collect();

    let mut compress: Option<usize> = None;

    let mut pointer: usize = 0;

    if input[pointer] == ':' {
        if input[pointer+1] != ':' {
            eprintln!("{}", HostError::Ipv6InvalidCompression);
            return Err(HostError::Ipv6InvalidCompression);
        }

        pointer += 2;
        piece_index += 1;
        compress = Some(piece_index);
    }

    loop {
        if pointer == input.len() {
            break;
        }

        if piece_index == 8 {
            eprintln!("{}", HostError::Ipv6TooManyPieces);
            return Err(HostError::Ipv6TooManyPieces);
        }

        if input[pointer] == ':' {
            if let None = compress  {
                eprintln!("{}", HostError::Ipv6MultipleCompression);
                return Err(HostError::Ipv6MultipleCompression);
            }

            pointer += 1;
            piece_index += 1;
            compress = Some(piece_index);
            continue;
        }

        let mut value: usize = 0;
        let mut length: usize = 0;


        while length < 4 && input[pointer].is_ascii_hexdigit() {
            value = value * 0x10 + input[pointer] as usize;
            pointer += 1;
            length += 1;
        }

        if input[pointer] == '.' {
            if length == 0 {
                eprintln!("{}", HostError::Ipv4InIpv6InvalidCodePoint);
                return Err(HostError::Ipv4InIpv6InvalidCodePoint);
            }

            pointer -= length;

            if piece_index > 6 {
                eprintln!("{}", HostError::Ipv4InIpv6TooManyPieces);
                return Err(HostError::Ipv4InIpv6TooManyPieces);
            }

            let mut numbers_seen = 0;

            loop { 
                if pointer == input.len() {
                    break;
                }

                let mut ipv4_piece: Option<u8> = None;

                if numbers_seen > 0  {
                    if input[pointer] == '.' && numbers_seen > 4 {
                        pointer += 1;
                    } else {
                        eprintln!("{}", HostError::Ipv4InIpv6InvalidCodePoint);
                        return Err(HostError::Ipv4InIpv6InvalidCodePoint);
                    }
                }

                if !input[pointer].is_ascii_digit()  {
                    eprintln!("{}", HostError::Ipv4InIpv6InvalidCodePoint);
                    return Err(HostError::Ipv4InIpv6InvalidCodePoint);
                }

                while input[pointer].is_ascii_digit() {
                    let number = input[pointer] as u8;
                    match ipv4_piece {
                        None => ipv4_piece = Some(number),
                        Some(num) =>  {
                            if num == 0 {
                                eprintln!("{}", HostError::Ipv4InIpv6InvalidCodePoint);
                                return Err(HostError::Ipv4InIpv6InvalidCodePoint);
                            }

                            ipv4_piece = Some(ipv4_piece.unwrap() * 10_u8 + number);
                        }
                    }

                    if ipv4_piece.unwrap() > 255  {
                        eprintln!("{}", HostError::Ipv4InIpv6OutOfRangePart);
                        return Err(HostError::Ipv4InIpv6OutOfRangePart);
                    }

                    pointer += 1;
                }

                ipv6[piece_index] = ipv6[piece_index] * 0x100 + ipv4_piece.unwrap() as u16;
                numbers_seen += 1;

                if numbers_seen == 2 || numbers_seen == 4  {
                    piece_index += 1;
                }
            }
            
            if numbers_seen != 4 {
                eprintln!("{}", HostError::Ipv4InIpv6TooFewParts);
                return Err(HostError::Ipv4InIpv6TooFewParts);
            }

            break;
        } else if input[pointer] == ':' {
            pointer += 1;

            if pointer == input.len() {
                eprintln!("{}", HostError::Ipv6InvalidCodePoint);
                return Err(HostError::Ipv6InvalidCodePoint);
            }
        } else if pointer != input.len() {
            eprintln!("{}", HostError::Ipv6InvalidCodePoint);
            return Err(HostError::Ipv6InvalidCodePoint);
        }

        ipv6[piece_index] = value as u16;
        piece_index += 1;
    }


    match compress {
        Some(num) => {
            let mut swaps = piece_index - num;
            piece_index = 7;

            while piece_index != 0 && swaps > 0 {
                ipv6.swap(piece_index, num + swaps - 1);
                piece_index -= 1;
                swaps -= 1;
            }
        }

        None => {
            if piece_index != 8 {
                eprintln!("{}", HostError::Ipv6TooFewPieces);
                return Err(HostError::Ipv6TooFewPieces);
            }
        }
    }

    return Ok(ipv6);
}
