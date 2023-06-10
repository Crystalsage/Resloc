use crate::{errors::ReslocError, types::types::URL};
use crate::hosts::host_serializer;

impl URL {
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


pub fn api_url_parser(url: String, base: Option<String>) -> Result<String, ReslocError> {
    let parsed_base: Result<String, ReslocError>;

    if base.is_some() {
        parsed_base = basic_url_parser(base.unwrap(), None, None);
        if parsed_base.is_err() {
            return parsed_base;
        }
    }

    return basic_url_parser(url, None, None);
}


pub fn basic_url_parser(input: String, base: Option<String>, url: Option<URL>) -> Result<String, ReslocError> {
    Ok(("ABCDEF".to_string()))
}
