use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParseError {
    HttpMethod,
    HttpVersion,
    Headers,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_string = match self {
            Self::HttpMethod => "Incorrect HTTP method.",
            Self::HttpVersion => "Incorrect HTTP version.",
            Self::Headers => "Incorrect headers.",
        };
        write!(f, "ParseError: {error_string}")
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub enum ServerError {
    ParseError(ParseError),
    IoError(std::io::Error),
}
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<ParseError> for ServerError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}
impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::ParseError(err) => err.fmt(f),
        }
    }
}
impl Error for ServerError {}
