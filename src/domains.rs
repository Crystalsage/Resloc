use std::collections::HashMap;

use crate::errors::{IDNAError, HostError};
use crate::types::types::{Ipv4NumberResult, Ipv6Pieces, IPv4};

use idna::{Config, Errors};

fn unicode_to_ascii(
    domain: String,
    check_hyphens: bool,
    use_std3_ascii_rules: bool,
    transitional_processing: bool,
    verify_dns_length: bool) ->  Result<String, IDNAError> {

    Config::default()
        .use_std3_ascii_rules(use_std3_ascii_rules)
        .check_hyphens(check_hyphens)
        .transitional_processing(transitional_processing)
        .verify_dns_length(verify_dns_length)
        .to_ascii(&domain)
        .map_err(|_| {
            IDNAError::DomainToAscii
        })
}

pub fn domain_to_ascii(domain: String, be_strict: bool) -> Result<String, IDNAError> {
    let result: Result<String, IDNAError> = unicode_to_ascii(domain, be_strict, false, false, be_strict);

    if let Ok(ref domain) = result {
        if domain.is_empty() {
            return Err(IDNAError::DomainToAscii);
        }
    }

    return result;
}

fn unicode_to_unicode(
    domain: String,
    check_hyphens: bool,
    use_std3_ascii_rules: bool,
    transitional_processing: bool) ->  (String, Result<(), Errors>) {

    Config::default()
        .use_std3_ascii_rules(use_std3_ascii_rules)
        .check_hyphens(check_hyphens)
        .transitional_processing(transitional_processing)
        .to_unicode(&domain)
}


pub fn domain_to_unicode(domain: String, be_strict: bool) -> String {
    let (result, errors) = unicode_to_unicode(domain, false, be_strict, false);
    if errors.is_err() {
        println!("Unicode to ASCII gave following errors!: {}", errors.unwrap_err());
    } 

    return result;
}

fn parse_ipv4_number(input: &str) -> Ipv4NumberResult {
    let mut input_chars = input.chars().collect::<Vec<char>>();

    if input_chars.len() == 0 {
        return Err(HostError::Ipv4Failure);
    }

    let mut validation_error: bool = false;
    let mut radix: u32 = 10;

    if input_chars.len() > 2 {
        match input_chars[0..2] {
            ['0', 'x'] | ['0', 'X'] => {
                validation_error = true;
                input_chars = input_chars[0..2].to_vec();
                radix = 16;
            }
            ['0'] => {
                validation_error = true;
                input_chars = input_chars[0..1].to_vec();
                radix = 8;
            }

            _ => {},
        }
    }

    if input_chars.len() == 0 {
        return Ok((0, true));
    }

    for point in input_chars {
        if !point.is_digit(radix) {
            return Err(HostError::Ipv4Failure);
        }
    }

    let result: u8 = input
        .parse::<u8>()
        .expect("Number is not parsable!");

    Ok((result, validation_error))
}

pub fn ipv4_parser(input: String) -> Result<IPv4, HostError> {
    let mut parts: Vec<&str> = input.split(".").collect();

    if let Some(last) = parts.last() {
        if last.is_empty() {
            println!("{}", HostError::Ipv4EmptyPart);
            if parts.len() > 1 {
                parts.pop();
            }
        }
    }

    if parts.len() > 4 {
        println!("{}", HostError::Ipv4TooManyParts);
        return Err(HostError::Ipv4TooManyParts);
    }

    let mut numbers: Vec<u32> = Vec::with_capacity(4);

    for part in parts {
        let result: Ipv4NumberResult  = parse_ipv4_number(part);


        match result {
            Err(_) => {
                eprintln!("{}", HostError::Ipv4NonNumericPart);
                return Err(HostError::Ipv4NonNumericPart);
            },
            Ok(num) => {
                match num.1 {
                    true => eprintln!("{}", HostError::Ipv4OutOfRangePart),
                    false => numbers.push(num.0.into()),
                }
            }
        }
    }


    for (idx, num) in numbers.iter().enumerate() {
        if num > &255 {
            eprintln!("{}", HostError::Ipv4OutOfRangePart);
            if idx != numbers.len() - 1 {
                return Err(HostError::Ipv4Failure);
            } else {
                if num > &256_u32.pow(5 - numbers.len() as u32) {
                    return Err(HostError::Ipv4Failure);
                }
            }
        }
    }

    let mut ipv4 = numbers.pop().unwrap();

    let mut counter: u32 = 0;
    for n in numbers {
        ipv4 = ipv4 + (n * 256_u32.pow(3 - counter));
        // TODO: Replace counter with enumerate's index.
        counter += 1;
    }

    Ok(ipv4)
}


pub fn ipv4_serializer(address: u32) -> String {
    let mut output: String = String::new();
    let mut n = address;

    for i in 1..=4 {
        output += &(n % 256).to_string();
        if i != 4 {
            output += ".";
        }

        n = (n as f64 / 256_f64).floor() as u32;
    }

    return output;
}

fn get_first_longest_sequence(address: &Ipv6Pieces) -> Option<usize> {
    let mut sequence_map: HashMap<usize, usize> = HashMap::new();

    let mut count: usize = 0;

    for (idx, piece) in address.iter().enumerate() {
        if *piece == 0 {
            count += 1;
        } else {
            sequence_map.insert(count, idx);
            count = 0;
        }
    }

    sequence_map.get(sequence_map.keys().max().unwrap()).cloned()
}


pub fn ipv6_serializer(address: Ipv6Pieces) -> String {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::Ipv6Address;

    #[test]
    fn test_u128_to_u16_array() {
        let ipv6_address = Ipv6Address(0xabcdefabcdefabcdefabcdefabcdefab);
        assert_eq!(Ipv6Pieces::from(ipv6_address), [0xefab, 0xabcd, 0xcdef, 0xefab, 0xabcd, 0xcdef, 0xefab, 0xabcd]);
    }

    #[test]
    fn test_ipv4_serializer() {
        let ipv4_address = u32::max_value();
        assert_eq!(ipv4_serializer(ipv4_address), "255.255.255.255".to_string());
    }

    fn test_longest_sequence() {
        assert_eq!(get_first_longest_sequence(&[0x0,0xf,0x0,0x0,0xf,0xf,0xf,0xf]), Some(2));
    }
}
