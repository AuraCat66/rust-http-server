use std::{
    io::{BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{
    errors::ServerError,
    request::{ClientRequest, Response},
};

#[derive(Default)]
pub struct HttpServerOptions {
    pub address: Option<SocketAddr>,
}
pub struct HttpServer {
    pub address: SocketAddr,
}
impl HttpServer {
    pub fn new(options: HttpServerOptions) -> Self {
        let address = options
            .address
            .unwrap_or(SocketAddr::from(([127, 0, 0, 1], 8000)));
        Self { address }
    }

    pub fn listen(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.address)?;
        println!("Listening to requests: {}", self.address);

        for stream in listener.incoming() {
            let mut stream = stream?;
            Self::handle_connection(&mut stream)?;
        }
        Ok(())
    }

    fn handle_connection(stream: &mut TcpStream) -> Result<(), ServerError> {
        let cloned_stream = stream.try_clone()?;
        // We use buf reader for reading stream in a faster/more efficient manner
        let buf_reader = BufReader::new(cloned_stream);

        // Not needed yet
        // but I want to implement keep alive feature at some point
        let keep_alive = false;

        // Our buffer for reading any data sent by the client
        let mut bytes = buf_reader.bytes();
        loop {
            let mut raw_request = String::new();

            // We read byte by byte and we push each character into the buffer
            // This is for the HTTP method and the headers, (todo:) the body is read afterwards
            for byte in bytes.by_ref() {
                let byte = byte?;
                let char = byte as char;
                raw_request.push(char);
                // If character is \n and the request ends with \r\n\r\n, we stop reading
                if char == '\n' && raw_request.ends_with("\r\n\r\n") {
                    break;
                }
            }

            let request = ClientRequest::parse_request(&raw_request)?;

            println!("{:?}", raw_request.split("\r\n").collect::<Vec<&str>>());

            Self::respond(stream, &request)?;
            if !keep_alive {
                break;
            }
        }
        Ok(())
    }

    fn respond(stream: &mut TcpStream, _request: &ClientRequest) -> Result<(), ServerError> {
        let mut response = Response::default();
        response.set_body(b"test\n");

        let response = response.validate()?;

        // We send the response to the client
        stream.write_all(&response)?;

        Ok(())
    }
}
