use std::default;

use crate::types::types::{UrlParseState, Host, HostType};
use crate::{errors::ReslocError, types::types::URL};
use crate::hosts::host_serializer;

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

    fn new(input: String, base: Option<String>, state_override: Option<UrlParseState>) -> Self {
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
    base: Option<String>, 
    url: Option<URL>, 
    state_override: Option<UrlParseState>) -> Result<URL, ReslocError> {

    if url.is_none() {
        let url = URL::default();
        let input = input.trim();
    }

    let mut input = input.replace("\t", "");
    input = input.replace("\n", "");

    let state: UrlParseState = match state_override {
        Some(_) => state_override.unwrap(),
        None => UrlParseState::SchemeStart
    };

    let buffer: String = String::new();
    let at_sign_seen: bool = false;
    let inside_brackets: bool = false;
    let password_token_seen: bool = false;

    todo!("Implement state switching");

    return Ok(url.unwrap());
}
