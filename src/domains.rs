use crate::errors::IDNAError;

fn unicode_to_ascii(
    domain: String,
    check_hyphens: bool,
    check_bidi: bool,
    check_joiners: bool,
    use_std3_ascii_rules: bool,
    transitional_processing: bool,
    verify_dns_length: bool) ->  Result<String, IDNAError> {

    Ok("HEY".to_string())
}

pub fn domain_to_ascii(domain: String, be_strict: bool) {
    let result: Result<String, IDNAError> = unicode_to_ascii(domain, be_strict, false, true, true, false, be_strict);
}
