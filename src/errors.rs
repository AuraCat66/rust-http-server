#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    HttpMethod,
    Headers,
}
