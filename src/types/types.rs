use std::{collections::HashMap};

use crate::errors::HostError;

#[derive(Clone)]
pub enum IPAddress {
    IPv4(u32),
    IPv6(u128),
}

#[derive(Clone)]
pub enum HostType {
    Domain,
    IPAddress(IPAddress),
    Opaque,
    Empty
}

#[derive(Clone)]
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

pub type IPv4 = u32;
pub type Ipv4NumberResult = Result<(u8, bool), HostError>;

pub struct Ipv6Address(pub u128);
pub type Ipv6Pieces = [u16; 8];

impl From<Ipv6Address> for Ipv6Pieces {
    fn from(value: Ipv6Address) -> Self {
        [
            (value.0 >> (0 * 16) & 0xFFFF) as u16,
            (value.0 >> (1 * 16) & 0xFFFF) as u16,
            (value.0 >> (2 * 16) & 0xFFFF) as u16,
            (value.0 >> (3 * 16) & 0xFFFF) as u16,
            (value.0 >> (4 * 16) & 0xFFFF) as u16,
            (value.0 >> (5 * 16) & 0xFFFF) as u16,
            (value.0 >> (6 * 16) & 0xFFFF) as u16,
            (value.0 >> (7 * 16) & 0xFFFF) as u16,
        ]
    }
}

enum UrlSearchParamTypes {
    SeqT(Vec<(String, String)>),
    RecT(HashMap<String, String>),
    StrT(String),
}

pub struct URLSearchParams {
    list: Vec<(String, String)>,
    size: u32
}

impl URLSearchParams {
    fn init(init: UrlSearchParamTypes) -> Self {
        let list: Vec<(String, String)> = match init {
            UrlSearchParamTypes::SeqT(seq) => {
                seq
                    .iter()
                    .map(|(x,y)| (x.to_owned(), y.to_owned()))
                    .collect::<Vec<(String, String)>>()
            }
            UrlSearchParamTypes::RecT(seq) => { 
                seq
                    .iter()
                    .map(|(x,y)| (x.to_owned(), y.to_owned()))
                    .collect::<Vec<(String, String)>>()
            }
            UrlSearchParamTypes::StrT(seq) => { 
                todo!("Add support for UrlSearchParamTypes::StrT");
            }
        };

        return URLSearchParams {
            size: list.len() as u32,
            list,
        }
    }

    fn size(self: Self) -> u32 {
        return self.size;
    } 

    fn append(name: String, value: String) {}
    fn delete(name: String, value: Option<String>) {}

    fn get(self: Self, name: String) -> Option<String> {
        for tuple in self.list {
            if tuple.0 == name {
                return Some(tuple.1);
            }
        }

        return None;
    }

    fn get_all(self: Self, name: String) -> Vec<String> {
        self.list
            .iter()
            .filter(|(f, s)| { f == &name })
            .map(|(first, second)| second.to_owned())
            .collect()
    }

    fn has(self: Self, name: String, value: Option<String>) -> bool {
        for element in self.list {
            if element.0 == name {
                if let Some(v) = value {
                    if element.1 == v {
                        return true;
                    }
                }

                return true;
            }
        }
        return false;
    }

    fn set(name: String, value: String) {}

    // TODO: Extract this into a separate application/x-www-form-urlencoded de/serializer
    fn to_string(self: Self) -> String {
        let mut output: String = String::new();

        for seq in self.list {
            if output.is_empty() {
                output += "&";
            }
            output += &format!("{}={}", seq.0, seq.1).to_string();
        }

        return output;
    }

    fn from_string(input: String) -> Self {
        let sequences: Vec<String> = input.split("&").map(|s| s.to_string()).collect();
        let mut output: Vec<(String, String)> = Vec::new();

        for bytes in sequences {
            if bytes.is_empty() {
                continue;
            }

            let result = bytes.split_once("=") .map(|(n, v)| (n.to_string(), v.to_string()));

            let (mut name, mut value) = match result {
                Some((name, value)) =>  {
                    (name, value)
                }
                None => {
                    (bytes, "".to_string())
                }
            };

            name = name.replace("+", " ");
            value = value.replace("+", " ");

            output.push((name, value));
        } 

        return URLSearchParams {
            size: output.len() as u32,
            list: output,
        };
    }

}

pub struct URL {
    pub href: String,
    pub origin: String,
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub fragment: Option<String>,
    pub query: Option<String>,
    pub host: Option<Host>,
    pub hostname: String,
    pub port: Option<String>,
    pub path: Vec<String>,
    pub search: String,
    pub hash: String,
}

pub enum UrlParseState {
    SchemeStart,
    Scheme,
    NoScheme,
    Authority,
    Relative,
    RelativeSlash,
    SpecialRelativeOrAuthority,
    SpecialAuthoritySlashes,
    SpecialAuthorityIgnoreSlashes,
    Host,
    HostName,
    Port,
    File,
    FileSlash,
    FileHost,
    PathStart,
    Path,
    PathOrAuthority,
    OpaquePath,
    Query,
    Fragment,
}
