use super::parse_error::ParseError;

pub type ParseResult<T> = Result<T, ParseError>;
