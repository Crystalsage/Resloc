use std::fmt;

#[derive(Debug)]
pub enum ReslocError {
    Failure,
}

#[derive(Debug)]
pub enum IDNAError {
    DomainToAscii,
    DomainToUnicode,
}

impl IDNAError {
    fn should_fail(self: Self) -> bool {
        match self {
            Self::DomainToAscii => true,
            Self::DomainToUnicode => false,
        }
    }
}


#[derive(Debug)]
pub enum HostError {
    Ipv4Failure,
    DomainInvalidCodePoint,
    HostInvalidCodePoint,
    Ipv4EmptyPart,
    Ipv4TooManyParts,
    Ipv4NonNumericPart,
    Ipv4NonDecimalPart,
    Ipv4OutOfRangePart,
    Ipv6Unclosed,
    Ipv6InvalidCompression,
    Ipv6MultipleCompression,
    Ipv6TooManyPieces,
    Ipv6InvalidCodePoint,
    Ipv6TooFewPieces,
    Ipv4InIpv6TooManyPieces,
    Ipv4InIpv6InvalidCodePoint,
    Ipv4InIpv6OutOfRangePart,
    Ipv4InIpv6TooFewParts,
}

impl HostError {
    fn should_fail(self: Self) -> bool {
        match self {
            Self::Ipv4Failure => true,
            Self::DomainInvalidCodePoint => true,
            Self::HostInvalidCodePoint => true,
            Self::Ipv4EmptyPart => false,
            Self::Ipv4TooManyParts => true,
            Self::Ipv4NonNumericPart => true,
            Self::Ipv4NonDecimalPart => false,
            Self::Ipv4OutOfRangePart => todo!(),
            Self::Ipv6Unclosed => true, 
            Self::Ipv6InvalidCompression => true,
            Self::Ipv6MultipleCompression => true,
            Self::Ipv6TooManyPieces => true,
            Self::Ipv6InvalidCodePoint => true,
            Self::Ipv6TooFewPieces => true,
            Self::Ipv4InIpv6TooManyPieces => true,
            Self::Ipv4InIpv6InvalidCodePoint => true,
            Self::Ipv4InIpv6OutOfRangePart => true,
            Self::Ipv4InIpv6TooFewParts => true,
        }
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Self::DomainInvalidCodePoint => true,
            // Self::HostInvalidCodePoint => true,
            Self::Ipv4EmptyPart => write!(f, "IPv4 address ends with a U+002E(.)!"),
            // Self::Ipv4TooManyParts => true,
            // Self::Ipv4NonNumericPart => true,
            // Self::Ipv4NonDecimalPart => false,
            // Self::Ipv4OutOfRangePart => todo!(),
            // Self::Ipv6Unclosed => true, 
            // Self::Ipv6InvalidCompression => true,
            // Self::Ipv6TooManyPieces => true,
            // Self::Ipv6InvalidCodePoint => true,
            // Self::Ipv6TooFewPieces => true,
            // Self::Ipv4InIpv6TooManyPieces => true,
            // Self::Ipv4InIpv6InvalidCodePoint => true,
            // Self::Ipv4InIpv6OutOfRangePart => true,
            // Self::Ipv4InIpv6TooFewParts => true,
            _ => todo!(),
        }
    }
}


#[derive(Debug)]
pub enum UrlError {
    InvalidUrlUnit,
    SSMissingFollowingSolidus,
    MissingSchemeNonRelativeUrl,
    InvalidReverseSolidus,
    InvalidCredentials,
    HostMissing,
    PortOutOfRange,
    PortInvalid,
    FileInvalidWdl,
    FileInvalidWdlHost,
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SSMissingFollowingSolidus => write!(f, "Special scheme missing following solidus!"),
            _ => todo!(),
        }
    }
}


impl UrlError {
    fn should_fail(self: Self) -> bool {
        match self {
            Self::InvalidUrlUnit => false,
            UrlError::SSMissingFollowingSolidus => false,
            Self::MissingSchemeNonRelativeUrl => true,
            Self::InvalidReverseSolidus => false,
            Self::InvalidCredentials => todo!(),
            Self::HostMissing => true,
            Self::PortOutOfRange => true,
            Self::PortInvalid => true,
            Self::FileInvalidWdl => false,
            Self::FileInvalidWdlHost => false,
        }
    }
}
