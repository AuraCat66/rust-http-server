use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use crate::END_OF_LINE;
use crate::errors::ParseError;

#[derive(Debug)]
pub struct ClientRequest {
    pub method: HttpMethod,
    pub http_version: String,
    pub headers: Headers,
    pub message_body: Vec<u8>,
}
impl ClientRequest {
    pub fn parse_request(raw_request: &str) -> Result<Self, ParseError> {
        let lines: Vec<&str> = raw_request.split("\r\n").collect();

        let method = match lines[0].split_once("/") {
            None => Err(ParseError::HttpMethod),
            Some((prefix, _)) => HttpMethod::from_str(prefix),
        }?;
        let raw_headers: Vec<&str> = lines[1..]
            .iter()
            .copied()
            .filter(|line| line.trim() != END_OF_LINE)
            .collect();
        let headers = Headers::parse_headers(&raw_headers)?;

        let message_body = Vec::new();
        Ok(Self {
            method,
            http_version: "1.1".to_owned(),
            headers,
            message_body,
        })
    }
}

#[derive(Debug, Default)]
pub struct Headers {
    inner: HashMap<String, String>,
}
impl Deref for Headers {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Headers {
    pub fn parse_headers(raw_headers: &[&str]) -> Result<Self, ParseError> {
        let mut headers = Self::default();

        for raw_header in raw_headers {
            if let Some((prefix, suffix)) = raw_header.split_once(":") {
                headers
                    .inner
                    .insert(prefix.to_owned(), suffix.trim().to_owned());
            } else {
                return Err(ParseError::Headers);
            }
        }

        Ok(headers)
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
impl FromStr for HttpMethod {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(ParseError::HttpMethod),
        }
    }
}
