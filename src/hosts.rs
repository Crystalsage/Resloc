use crate::types::hosts::*;


fn public_suffix_list_algorithm(domain: String) -> String {
    return String::new();
}

pub fn get_public_suffix(host: Host) -> Option<String> {
    match host.host_type {
        HostType::Domain => {},
        _ => return None,
    }

    let trailing_dot: &str = match host.value.ends_with('.') {
        true => ".",
        false => "",
    };


    let public_suffix: String = public_suffix_list_algorithm(host.value);

    assert_eq!(public_suffix.ends_with('.'), false);

    return Some(public_suffix + trailing_dot);
}
