use std::{
    error::Error,
    io::{BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use errors::ServerError;
use request::{ClientRequest, Response};

mod errors;
mod request;

const END_OF_LINE: &str = "\r\n";

fn main() -> Result<(), Box<dyn Error>> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    // The TCP listener for listening to incoming requests
    let listener = TcpListener::bind(address)?;

    println!("Listening to requests: {address}");

    // We are now listening to any incoming connection
    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), ServerError> {
    // For faster/more efficient reading of stream
    let cloned_stream = stream.try_clone()?;
    let buf_reader = BufReader::new(cloned_stream);

    let keep_alive = false;

    // Our buffer for reading any data sent by the client
    let mut bytes = buf_reader.bytes();
    loop {
        // The raw request sent by the client
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

        respond_to_request(stream, &request)?;
        if !keep_alive {
            break;
        }
    }

    Ok(())
}

fn respond_to_request(stream: &mut TcpStream, _request: &ClientRequest) -> Result<(), ServerError> {
    let mut response = Response::default();
    response.set_body(b"test\n");

    let response = response.validate()?;

    // We send the response to the client
    stream.write_all(&response)?;

    Ok(())
}
