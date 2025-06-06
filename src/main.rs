use std::error::Error;

use crate::http_server::{HttpServer, HttpServerOptions};

mod errors;
mod http_server;
mod request;

const END_OF_LINE: &str = "\r\n";

fn main() -> Result<(), Box<dyn Error>> {
    let server = HttpServer::new(HttpServerOptions::default());
    server.listen()?;
    Ok(())
}
