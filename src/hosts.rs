use crate::types::hosts::*;

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
