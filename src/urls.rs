use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::errors::{HostError, UrlError};
use crate::types::types::{UrlParseState, Host, HostType};
use crate::{errors::ReslocError, types::types::URL};
use crate::hosts::host_serializer;

lazy_static! {
    static ref SPECIAL_SCHEMES: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("ftp", 21);
        m.insert("file", 0);
        m.insert("http", 80);
        m.insert("https", 443);
        m.insert("ws", 80);
        m.insert("wss", 443);
        m
    };
}


impl URL {
    fn default() -> Self {
        Self { 
            href: "".to_string(),
            origin: "".to_string(),
            scheme: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            fragment: Some("".to_string()),
            query: Some("".to_string()),
            host: Some(Host::new("".to_string(), HostType::Empty)),
            hostname: "".to_string(),
            port: Some("".to_string()),
            path: Vec::new(),
            search: "".to_string(),
            hash: "".to_string(),
        }
    }

    fn new(input: String, base: Option<URL>, state_override: Option<UrlParseState>) -> Self {
        // TODO: What does the optional URL do? 
        let output: Result<URL, ReslocError> = basic_url_parser(input, base, None, state_override);
        match output {
            Ok(_) => output.unwrap(),
            Err(_) => panic!("Basic URL parser failed!"),
        }
    }

    fn equals(self: &Self, other: &Self, exclude_fragment: Option<bool>) -> bool {
        let serialized_self = self.serialize(exclude_fragment);
        let serialized_other = other.serialize(exclude_fragment);

        return serialized_self == serialized_other;
    }


    fn includes_credentials(self: &Self) -> bool {
        return !self.username.is_empty() && !self.password.is_empty()
    }

    fn has_opaque_path(self: &Self) -> bool {
        todo!("Implement has_opaque_path");
    }

    fn get_default_port(scheme: &String) -> Option<String> {
        SPECIAL_SCHEMES.get(scheme.as_str()).map(|x| x.to_string())
    }

    fn is_special(self: &Self) -> bool {
        return Self::is_special_scheme(&self.scheme);
    }

    fn is_special_scheme(scheme: &String) -> bool {
        return SPECIAL_SCHEMES.contains_key(scheme.as_str());
    }

    fn is_normalized_windows_letter(path: String) -> bool { 
        let path_chars: Vec<char> = path.chars().collect();
        return path_chars[0].is_ascii_alphabetic() && path_chars[1] == ':';
    }

    fn shorten_path(self: &mut Self) {
        assert_eq!(self.has_opaque_path(), false);

        let path = self.path;
        if self.scheme == "file" && path.len() == 1 && URL::is_normalized_windows_letter(path[0]) {
            return;
        }

        self.path.pop();
    }

    fn serialize_path(self: &Self) -> String {
        if self.has_opaque_path() {
            // TODO: 
            return self.path.join("");
        }

        let mut output: String = String::new();
        for segment in &self.path {
            output += "/";
            output += &segment;
        }

        return output;
    }

    fn serialize(self: &Self, exclude_fragment: Option<bool>) -> String {
        let mut output: String = self.scheme.to_owned() + ":";

        match self.host {
            Some(_) => { 
                output += "//";

                if self.includes_credentials() {
                    output += &self.username;
                    if !self.password.is_empty() {
                        output += ":";
                        output += &self.password;
                    }
                    output += "@";
                }

                output += &host_serializer(self.host.as_ref().unwrap());
                if self.port.is_some() {
                    output += ":";
                    output += &self.port.as_ref().unwrap();
                }
            },
            None => { 
                if self.has_opaque_path() && self.path.len() > 1 && self.path[0].is_empty() {
                    output += "/";
                    output += ".";
                }
            },
        }

        output += &self.serialize_path();

        if self.query.is_some() {
            output += "?";
            output += &self.query.clone().unwrap();
        }

        if exclude_fragment == Some(false) && self.fragment.is_some() {
            output += "#";
            output += &self.fragment.clone().unwrap();
        }

        return output;
    }
}


pub fn basic_url_parser(
    input: String, 
    base: Option<URL>, 
    url: Option<URL>, 
    state_override: Option<UrlParseState>) -> Result<URL, ReslocError> {

    if url.is_none() {
        let input = input.trim();
    }

    let mut url = url.unwrap_or(URL::default());

    let mut input = input.replace("\t", "");
    input = input.replace("\n", "");

    let has_state_override: bool = state_override.is_some();
    let mut state: UrlParseState = state_override.unwrap_or(UrlParseState::SchemeStart);

    let has_base: bool = base.is_some();
    let base: URL = base.unwrap();

    let at_sign_seen: bool = false;
    let inside_brackets: bool = false;
    let password_token_seen: bool = false;

    let mut buffer: String = String::new();
    let mut input: Vec<char> = input.chars().collect();
    let mut pointer: usize = 0;

    loop {
        if pointer == input.len() {
            break;
        }
        pointer += 1;

        let mut c: char = input[pointer];

        match state {
            UrlParseState::SchemeStart => {
                if c.is_ascii_alphabetic() {
                    buffer += &c.to_ascii_lowercase().to_string();
                    state = UrlParseState::Scheme;
                    pointer += 1;
                } else if has_state_override {
                    state = UrlParseState::NoScheme;
                    pointer -= 1;
                } else {
                    return Err(ReslocError::Failure);
                }
            }
            UrlParseState::Scheme => {
                if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.' {
                    buffer += &c.to_lowercase().to_string();
                } else if c == ':' {
                    if has_state_override {
                        if (URL::is_special_scheme(&url.scheme) && URL::is_special_scheme(&buffer)) == false {
                            // Reference: https://url.spec.whatwg.org/#scheme-state
                            // TODO: What to return here? Or does it mean continue with the current
                            // loop iteration?
                            return Err(ReslocError::Failure);
                        }

                        if ((url.includes_credentials() || url.port.is_some()) && buffer == "file") {
                            return Err(ReslocError::Failure);
                        }

                        if url.scheme == "file" && url.host.as_ref().unwrap().value.is_empty() {
                            return Err(ReslocError::Failure);
                        }
                    }

                    url.scheme = buffer.clone();

                    if has_state_override {
                        if url.port == URL::get_default_port(&url.scheme) {
                            url.port = None;
                        }
                        return Err(ReslocError::Failure);
                    }

                    buffer = "".to_string();

                    if url.scheme == "file" {
                        if !input[pointer+1..].starts_with(&['/', '/']) { 
                            eprintln!("{}", UrlError::SSMissingFollowingSolidus);
                            state = UrlParseState::File;
                        }
                    } else if input[pointer+1..].starts_with(&['/']) {
                        state = UrlParseState::PathOrAuthority;
                        pointer += 1;
                    } else if url.is_special() {
                        if let Some(ref b) = base {
                            if b.scheme == url.scheme {
                                assert_eq!(b.is_special(), true);
                                state = UrlParseState::SpecialRelativeOrAuthority;
                            }
                        } else {
                            state = UrlParseState::SpecialAuthoritySlashes;
                        }
                    } else {
                        url.path = vec!["".to_string()];
                        state = UrlParseState::OpaquePath;
                    }
                } else if !has_state_override {
                    buffer = "".to_string();
                    state = UrlParseState::NoScheme;
                    pointer = 0;
                } else {
                    return Err(ReslocError::Failure);
                }
            }
            UrlParseState::NoScheme => {
                match base {
                    None => {
                        eprintln!("{}", UrlError::MissingSchemeNonRelativeUrl);
                        return Err(ReslocError::Failure);
                    }
                    Some(ref b) => {
                        if b.has_opaque_path() {
                            if c != '#' {
                                eprintln!("{}", UrlError::MissingSchemeNonRelativeUrl);
                                return Err(ReslocError::Failure);
                            } else {
                                url.scheme = b.scheme.clone();
                                url.path = b.path.clone();
                                url.query = b.query.clone();
                                url.fragment = Some("".to_string());
                                state = UrlParseState::Fragment;
                            }
                        } else if b.scheme != "file" {
                            state = UrlParseState::Relative;
                            pointer -= 1;
                        } else {
                            state = UrlParseState::File;
                            pointer -= 1;
                        }
                    }
                }
            }
            UrlParseState::SpecialRelativeOrAuthority => {
                if c == '/' && input[pointer+1..].starts_with(&['/']) {
                    state = UrlParseState::SpecialAuthorityIgnoreSlashes;
                    pointer += 1;
                } else {
                    eprintln!("{}", UrlError::SSMissingFollowingSolidus);
                    state = UrlParseState::Relative;
                    pointer -= 1;
                }
            }
            UrlParseState::PathOrAuthority => {
                if c == '/' {
                    state = UrlParseState::Authority;
                } else {
                    state = UrlParseState::Path;
                    pointer -= 1;
                }
            }
            UrlParseState::Relative => {
                assert_eq!(base.scheme, "file".to_string());
                url.scheme = base.scheme;

                if c == '/' {
                    state = UrlParseState::RelativeSlash;
                } else if url.is_special() && c == '\\' {
                    eprintln!("{}", UrlError::InvalidReverseSolidus);
                    state = UrlParseState::RelativeSlash;
                } else {
                    url.username = base.username;
                    url.password = base.password;
                    url.host = base.host;
                    url.port = base.port;
                    url.path = base.path;
                    url.query = base.query;

                    match c {
                        '?' => {
                            url.query = Some("".to_string());
                            state = UrlParseState::Query;
                        }
                        '#' => {
                            url.fragment = Some("".to_string());
                            state = UrlParseState::Fragment;
                        }
                        _ => {
                            url.query = None;
                            url.shorten_path();
                            state = UrlParseState::Path;
                            pointer -= 1;
                        }
                    }
                }
            }
            UrlParseState::RelativeSlash => {
                if url.is_special() && c == '/' || c == '\\' {
                    if c == '\\' {
                        eprintln!("{}", UrlError::InvalidReverseSolidus);
                        state = UrlParseState::SpecialAuthorityIgnoreSlashes;
                    }
                } else if c == '/' {
                    state = UrlParseState::Authority;
                } else {
                    url.username = base.username;
                    url.password = base.password;
                    url.host = base.host;
                    url.port = base.port;
                    state = UrlParseState::Path;
                    pointer -= 1;
                }
            }
            UrlParseState::SpecialAuthoritySlashes => {
                if c == '/' && input[pointer+1..].starts_with(&['/']) {
                    state = UrlParseState::SpecialAuthorityIgnoreSlashes;
                    pointer += 1;
                } else {
                    eprintln!("{}", UrlError::SSMissingFollowingSolidus);
                    state = UrlParseState::SpecialAuthorityIgnoreSlashes;
                    pointer -= 1;
                }
            }
            UrlParseState::SpecialAuthorityIgnoreSlashes => {
                if c != '/' || c != '\\' {
                    state = UrlParseState::Authority;
                    pointer -= 1;
                } else {
                    eprintln!("{}", UrlError::SSMissingFollowingSolidus);
                }
            }
            UrlParseState::Authority => {
                if c == '@' {
                    eprintln!("{}", UrlError::InvalidCredentials);

                    if at_sign_seen {
                        buffer += "%40";
                    }

                    at_sign_seen = true;
                    for bc in buffer.chars() {
                        if bc == ':' && password_token_seen == false {
                            password_token_seen = true;
                            // TODO: Is this continue correct? 
                            continue;
                        } 

                        if password_token_seen {
                            url.password += &bc.to_string();
                        } else {
                            url.username += &bc.to_string();
                        }
                    }

                    buffer = "".to_string();
                }

                if (c == '/' || c == '?' || c == '#') || (url.is_special() && c == '\\') {
                    if at_sign_seen && buffer.is_empty() {
                        eprintln!("{}", UrlError::InvalidCredentials);
                        return Err(ReslocError::Failure);
                    } else {
                        pointer -= buffer.len() + 1;
                        buffer = "".to_string();
                        state = UrlParseState::Host;
                    }
                }
            }
            _ => break,
        }
    }

    return Ok(URL::default());
}
