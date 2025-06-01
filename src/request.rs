use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::END_OF_LINE;
use crate::errors::{ParseError, ServerError};

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

        let (method, http_version) = match lines[0].split_once("/") {
            None => Err(ParseError::HttpMethod),
            Some((prefix, suffix)) => Ok((
                HttpMethod::from_str(prefix.trim())?,
                match suffix.split_once("/") {
                    None => Err(ParseError::HttpVersion),
                    Some((_, version)) => Ok(version.to_owned()),
                }?,
            )),
        }?;
        let raw_headers: Vec<&str> = lines[1..]
            .iter()
            .copied()
            .filter(|line| !line.is_empty())
            .collect();
        let headers = Headers::parse_headers(&raw_headers)?;

        let message_body = Vec::new();
        Ok(Self {
            method,
            http_version,
            headers,
            message_body,
        })
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: String,
    pub headers: Headers,
    message_body: Vec<u8>,
}
impl Default for Response {
    fn default() -> Self {
        let mut headers = Headers::default();
        headers.insert("Content-Type".to_owned(), "text/html".to_owned());
        Self {
            status: "200 OK".to_owned(),
            headers,
            message_body: Default::default(),
        }
    }
}
impl Response {
    pub fn set_body(&mut self, new_body: &[u8]) {
        self.message_body = new_body.to_owned();
        self.headers
            .insert("Content-Length".to_owned(), new_body.len().to_string());
    }

    pub fn validate(self) -> Result<Vec<u8>, ServerError> {
        let mut bytes = Vec::new();
        let status = "HTTP/1.1 ".to_owned() + &self.status + END_OF_LINE;
        let headers = self.headers.to_string();

        bytes.write_all(status.as_bytes())?;
        bytes.write_all(headers.as_bytes())?;
        bytes.write_all(END_OF_LINE.as_bytes())?;
        if !self.message_body.is_empty() {
            bytes.write_all(&self.message_body)?;
        }
        Ok(bytes)
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
impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<String>>()
            .join(END_OF_LINE)
            + END_OF_LINE)
            .fmt(f)
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
impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        };
        s.fmt(f)
    }
}
impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}
