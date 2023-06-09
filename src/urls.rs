use crate::{errors::ReslocError, types::types::URL};

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
