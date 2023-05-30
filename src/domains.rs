use crate::errors::IDNAError;

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
